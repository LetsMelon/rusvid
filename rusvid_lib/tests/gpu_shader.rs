use anyhow::Result;
use image::RgbaImage;
use pollster::FutureExt;
use rusvid_lib::{layer::Layer, resolution::Resolution};
use std::time::Instant;
use wgpu::{
    Adapter, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Buffer,
    BufferDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor,
    ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor, Extent3d,
    ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, Instance, MaintainBase, MapMode, Origin3d,
    PowerPreference, Queue, RequestAdapterOptionsBase, ShaderModuleDescriptor, ShaderSource,
    Texture, TextureAspect, TextureDescriptor, TextureViewDescriptor,
};

fn generate_test_array(resolution: Resolution) -> Result<RgbaImage> {
    use rusvid_lib::composition::Composition;
    use rusvid_lib::figures::rect::rect;
    use rusvid_lib::prelude::RawRender;
    use rusvid_lib::utils::color_from_hex;
    use std::rc::Rc;
    use usvg::{Fill, NodeKind, Paint, Path};

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(1)
        .duration(1)
        .build();

    let (width, height) = {
        let raw = composition.resolution().value();
        (raw.0 as f64, raw.1 as f64)
    };

    let mut layer = Layer::new(composition.resolution());
    layer.add_to_root(NodeKind::Path(Path {
        id: "rect".to_string(),
        fill: Some(Fill {
            paint: Paint::Color(color_from_hex("ff0000".to_string()).unwrap()),
            ..Fill::default()
        }),
        data: Rc::new(rect(0.0, 0.0, width / 2.0, height / 2.0)),
        ..Path::default()
    }));
    composition.add_layer(layer);

    let image_render = RawRender::new();

    composition.update(0).unwrap();
    image_render.calculate_image_buffer(&composition)
}

fn compute_work_group_count(
    (width, height): (u32, u32),
    (workgroup_width, workgroup_height): (u32, u32),
) -> (u32, u32) {
    let x = (width + workgroup_width - 1) / workgroup_width;
    let y = (height + workgroup_height - 1) / workgroup_height;

    (x, y)
}

fn padded_bytes_per_row(width: u32) -> usize {
    let bytes_per_row = width as usize * 4;
    let padding = (256 - bytes_per_row % 256) % 256;
    bytes_per_row + padding
}

struct GpuRender {
    pub device: Device,
    pub queue: Queue,
    pub dimensions: (u32, u32),
    pub layers_texture: [Texture; 2],
    pub texture_size: Extent3d,
    pub output_texture: Texture,
    pub compute_pipeline: ComputePipeline,
    pub texture_bind_group: BindGroup,
    pub padded_bytes_per_row: usize,
    pub unpadded_bytes_per_row: usize,
    pub output_buffer: Buffer,
}

impl GpuRender {
    const WORKGROUP_SIZE: (u32, u32) = (8, 8);
    const LAYER_SHADER: &'static str = include_str!("./shaders/combine_layers.wgsl");

    #[inline]
    async fn get_adapter(instance: &Instance) -> Option<Adapter> {
        instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
    }

    #[inline]
    fn create_input_texture(device: &Device, size: Extent3d, label: Option<&str>) -> Texture {
        device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        })
    }

    #[inline]
    fn create_output_texture(device: &Device, size: Extent3d) -> Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("output texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Uint,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
        })
    }

    fn new(width: u32, height: u32) -> Result<Self> {
        let now = Instant::now();

        let instance = Instance::new(Backends::all());

        let adapter = Self::get_adapter(&instance)
            .block_on()
            .ok_or(anyhow::anyhow!("Couldn't create the adapter"))?;
        println!("{:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .block_on()?;

        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let i_t_1 = Self::create_input_texture(&device, texture_size, Some("layer1 input texture"));
        let i_t_2 = Self::create_input_texture(&device, texture_size, Some("layer2 input texture"));

        let output_texture = Self::create_output_texture(&device, texture_size);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Layer shader"),
            source: ShaderSource::Wgsl(Self::LAYER_SHADER.into()),
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Shader pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &i_t_1.create_view(&TextureViewDescriptor::default()),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(
                        &i_t_2.create_view(&TextureViewDescriptor::default()),
                    ),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(
                        &output_texture.create_view(&TextureViewDescriptor::default()),
                    ),
                },
            ],
        });

        let padded_bytes_per_row = padded_bytes_per_row(width);
        let unpadded_bytes_per_row = width as usize * 4;

        let output_buffer_size =
            padded_bytes_per_row as u64 * height as u64 * std::mem::size_of::<u8>() as u64;
        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: output_buffer_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let dt = now.elapsed().as_millis();
        println!("gpu_magic::new took {}ms", dt);

        Ok(Self {
            device,
            queue,
            dimensions: (width, height),
            layers_texture: [i_t_1, i_t_2],
            texture_size,
            output_texture,
            compute_pipeline: pipeline,
            texture_bind_group,
            padded_bytes_per_row,
            unpadded_bytes_per_row,
            output_buffer,
        })
    }

    fn pass(&self, layer_1: RgbaImage, layer_2: RgbaImage) -> Result<()> {
        let now = Instant::now();

        if layer_1.dimensions() != layer_2.dimensions() {
            return Err(anyhow::anyhow!("Layers must have the same dimensions!"));
        }

        let (width, height) = self.dimensions;

        self.queue.write_texture(
            self.layers_texture[0].as_image_copy(),
            bytemuck::cast_slice(layer_1.as_raw()),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: None,
            },
            self.texture_size,
        );

        self.queue.write_texture(
            self.layers_texture[1].as_image_copy(),
            bytemuck::cast_slice(layer_2.as_raw()),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: None,
            },
            self.texture_size,
        );

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let (dispatch_width, dispatch_height) = compute_work_group_count(
                (self.texture_size.width, self.texture_size.height),
                Self::WORKGROUP_SIZE,
            );

            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Shader pass"),
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            compute_pass.dispatch_workgroups(dispatch_width, dispatch_height, 1);
        }

        encoder.copy_texture_to_buffer(
            ImageCopyTexture {
                aspect: TextureAspect::All,
                texture: &self.output_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(self.padded_bytes_per_row as u32),
                    rows_per_image: std::num::NonZeroU32::new(height),
                },
            },
            self.texture_size,
        );
        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = self.output_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(MaintainBase::Wait);

        if let Some(Ok(())) = receiver.receive().block_on() {
            let data = buffer_slice.get_mapped_range();

            let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

            println!("len: {}", result.len());
            println!("data: {:?}", result);

            // assert_eq!(result[0], 4278190335);
            // TODO where pixels?

            drop(data);
            self.output_buffer.unmap();
        } else {
            return Err(anyhow::anyhow!("failed to run compute on gpu!"));
        }

        let dt = now.elapsed().as_millis();
        println!("gpu_magic::pass took {}ms", dt);

        Ok(())
    }
}

fn gpu_magic(img_1: RgbaImage, img_2: RgbaImage) -> Result<()> {
    let (width, height) = img_1.dimensions();

    let gpu = GpuRender::new(width, height)?;
    gpu.pass(img_1, img_2)?;

    Ok(())
}

fn main() {
    let resolution = Resolution::Custom(64, 64);
    let layer_1 = generate_test_array(resolution).unwrap();
    let layer_2 = generate_test_array(resolution).unwrap();

    // println!("layer_1: {:?}", layer_1.to_vec());
    // println!("layer_1 (len): {:?}", layer_2.to_vec().len());
    // println!("layer_2: {:?}", layer_1.to_vec());
    // println!("layer_2 (len): {:?}", layer_2.to_vec().len());

    let now = Instant::now();
    gpu_magic(layer_1, layer_2).unwrap();
    let dt = now.elapsed().as_millis();
    println!("gpu_magic took {}ms", dt);

    // x + (64 - x % 64)
}
