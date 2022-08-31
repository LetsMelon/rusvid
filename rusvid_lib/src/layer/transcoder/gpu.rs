use anyhow::Result;
use image::RgbaImage;
use pollster::FutureExt;
use tiny_skia::Pixmap;
use wgpu::{
    Adapter, BindGroup, Buffer, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline,
    Device, Extent3d, ImageCopyBuffer, ImageCopyTexture, ImageDataLayout, Instance, MaintainBase,
    MapMode, Origin3d, PowerPreference, Queue, RequestAdapterOptionsBase, Texture, TextureAspect,
    TextureDescriptor,
};

use crate::layer::LayerTranscoder;

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

pub struct GpuLayerTranscoder {
    device: Device,
    queue: Queue,
    dimensions: (u32, u32),
    layers_texture: [Texture; 2],
    texture_size: Extent3d,
    output_texture: Texture,
    compute_pipeline: ComputePipeline,
    texture_bind_group: BindGroup,
    padded_bytes_per_row: usize,
    unpadded_bytes_per_row: usize,
    output_buffer: Buffer,
}

impl GpuLayerTranscoder {
    const WORKGROUP_SIZE: (u32, u32) = (8, 8);
    const LAYER_SHADER: &'static str = include_str!("./combine_layers.wgsl");

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

    fn pass(&self, layer_1: RgbaImage, layer_2: RgbaImage) -> Result<()> {
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

        todo!()
    }
}

impl LayerTranscoder for GpuLayerTranscoder {
    fn combine_renders(&self, _pixmaps: Vec<Pixmap>) -> RgbaImage {
        todo!()
    }
}
