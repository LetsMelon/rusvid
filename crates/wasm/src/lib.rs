use std::collections::VecDeque;
use std::panic;
use std::sync::Mutex;

use lazy_static::{__Deref, lazy_static};
use rusvid_core::holder::likes::*;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::polygon::Polygon;
use rusvid_core::holder::stroke::Stroke;
use rusvid_core::holder::svg_holder::SvgHolder;
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_core::holder::transform::{Transform, TransformLogic};
use rusvid_core::pixel::Pixel;
use rusvid_core::point::Point;
use wasm_bindgen::prelude::*;
use web_sys::console;

lazy_static! {
    static ref OBJECT: Mutex<Object> = Mutex::new(Object::new(TypesLike::Svg(SvgHolder::new())));
    static ref WIDTH: Mutex<u32> = Mutex::new(300);
    static ref HEIGHT: Mutex<u32> = Mutex::new(300);
}

#[wasm_bindgen]
pub fn set_width(value: u32) {
    let mut binding = WIDTH.lock();
    let width = binding.as_mut().unwrap();
    **width = value;
}

#[wasm_bindgen]
pub fn set_height(value: u32) {
    let mut binding = HEIGHT.lock();
    let height = binding.as_mut().unwrap();
    **height = value;
}

#[wasm_bindgen]
pub fn get_width() -> u32 {
    *WIDTH.lock().unwrap()
}

#[wasm_bindgen]
pub fn get_height() -> u32 {
    *HEIGHT.lock().unwrap()
}

#[wasm_bindgen]
pub fn render() -> js_sys::Uint8ClampedArray {
    let width = *WIDTH.lock().unwrap().deref();
    let height = *HEIGHT.lock().unwrap().deref();

    let object = OBJECT.lock().unwrap();

    let plane = object.render(width, height).unwrap();
    let data = plane.as_data();

    js_sys::Uint8ClampedArray::from(&data.iter().flat_map(|p| p.to_raw()).collect::<Vec<u8>>()[..])
}

#[wasm_bindgen]
pub fn transform_color(color: js_sys::Uint8ClampedArray) {
    let color_vec = color.to_vec();

    let mut object = OBJECT.lock().unwrap();
    if color_vec.len() == 4 || color_vec.len() == 3 {
        object
            .transform(&Transform::Color(Some(ColorLike::Color(Pixel::new(
                color_vec[0],
                color_vec[1],
                color_vec[2],
                255,
            )))))
            .unwrap()
    } else {
        object.transform(&Transform::Color(None)).unwrap()
    }
}

#[wasm_bindgen]
pub fn transform_stroke(paint: Option<js_sys::Uint8ClampedArray>, width: Option<f64>) {
    let paint_vec_option = paint.map(|item| item.to_vec()).unwrap_or_default();
    let stroke = match (paint_vec_option.len(), width) {
        (3 | 4, Some(w)) => {
            let c = ColorLike::Color(Pixel::new(
                paint_vec_option[0],
                paint_vec_option[1],
                paint_vec_option[2],
                255,
            ));

            Some(Stroke {
                paint: c,
                width: w,
                ..Default::default()
            })
        }
        _ => None,
    };

    let mut object = OBJECT.lock().unwrap();
    object.transform(&Transform::Stroke(stroke)).unwrap();
}

#[wasm_bindgen]
pub fn transform_move(x: f64, y: f64) {
    let mut object = OBJECT.lock().unwrap();
    object
        .transform(&Transform::Move(Point::new(x, y)))
        .unwrap();
}

#[wasm_bindgen]
pub fn transform_position(x: f64, y: f64) {
    let mut object = OBJECT.lock().unwrap();
    object
        .transform(&Transform::Position(Point::new(x, y)))
        .unwrap();
}

#[wasm_bindgen]
pub fn transform_visibility(value: bool) {
    let mut object = OBJECT.lock().unwrap();
    object.transform(&Transform::Visibility(value)).unwrap();
}

#[wasm_bindgen]
pub fn transform_rotate(value: f64) {
    let mut object = OBJECT.lock().unwrap();
    object
        .transform(&Transform::Rotate(value.to_radians()))
        .unwrap();
}

#[wasm_bindgen]
pub fn transform_scale(x: f64, y: f64) {
    let mut object = OBJECT.lock().unwrap();
    object
        .transform(&Transform::Scale(Point::new(x, y)))
        .unwrap();
}

#[wasm_bindgen]
pub fn calculate_bounding_box(id: String) -> Option<js_sys::Int32Array> {
    let object = OBJECT.lock().unwrap();
    let data = object.data();

    match data {
        TypesLike::Svg(svg_holder) => svg_holder.get_item(id).map(|item| {
            let bounding = item.bounding_box();

            let x1 = bounding.0.x() as i32;
            let y1 = bounding.0.y() as i32;
            let x2 = bounding.1.x() as i32;
            let y2 = bounding.1.y() as i32;

            js_sys::Int32Array::from(&vec![x1, y1, x2, y2][..])
        }),
        TypesLike::Image(_) => todo!(),
    }
}

#[wasm_bindgen]
pub fn add_svg(data: js_sys::Int32Array, color: js_sys::Uint8ClampedArray) -> Option<String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // kind ....
    // 0    x y                     -> Move
    // 1    x y                     -> Line
    // 2    x y x_cs y_cs x_ce y_ce -> Curve
    // 255..                        -> Close

    let mut as_vec = VecDeque::from(data.to_vec());
    let mut paths = Vec::new();

    while !as_vec.is_empty() {
        let kind = as_vec.pop_front().unwrap();

        match kind {
            0 => {
                let x = as_vec.pop_front().unwrap();
                let y = as_vec.pop_front().unwrap();

                paths.push(PathLike::Move(Point::new(x as f64, y as f64)));
            }
            1 => {
                let x = as_vec.pop_front().unwrap();
                let y = as_vec.pop_front().unwrap();

                paths.push(PathLike::Line(Point::new(x as f64, y as f64)));
            }
            2 => {
                let x = as_vec.pop_front().unwrap() as f64;
                let y = as_vec.pop_front().unwrap() as f64;

                let x_cs = as_vec.pop_front().unwrap() as f64;
                let y_cs = as_vec.pop_front().unwrap() as f64;

                let x_ce = as_vec.pop_front().unwrap() as f64;
                let y_ce = as_vec.pop_front().unwrap() as f64;

                paths.push(PathLike::CurveTo(
                    Point::new(x, y),
                    Point::new(x_cs, y_cs),
                    Point::new(x_ce, y_ce),
                ))
            }
            255.. => {
                paths.push(PathLike::Close);
            }
            _ => break,
        }
    }

    let color_as_vec = color.to_vec();
    let color = match color.to_vec().len() {
        4 => {
            let r = color_as_vec[0];
            let g = color_as_vec[1];
            let b = color_as_vec[2];
            let a = color_as_vec[3];

            Some(ColorLike::Color(Pixel::new(r, g, b, a)))
        }
        _ => None,
    };

    let item = SvgItem::new(Polygon::new(&paths), color);

    console::log_1(&format!("{:?}", item).into());

    let mut binding = OBJECT.lock().unwrap();
    let types_like = binding.data_mut();
    if let TypesLike::Svg(svg_holder) = types_like {
        let id = svg_holder.add_item(item);

        return Some(id);
    }

    None
}
