use std::collections::VecDeque;
use std::sync::Mutex;

use lazy_static::{__Deref, lazy_static};
use rusvid_core::holder::likes::*;
use rusvid_core::holder::object::{Object, TransformLogic};
use rusvid_core::holder::stroke::Stroke;
use rusvid_core::holder::svg_holder::{SvgHolder, SvgItem};
use rusvid_core::holder::transform::Transform;
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

    js_sys::Uint8ClampedArray::from(&data.iter().flatten().map(|x| *x).collect::<Vec<u8>>()[..])
}

#[wasm_bindgen]
pub fn transform_color(color: js_sys::Uint8ClampedArray) {
    let color_vec = color.to_vec();

    let mut object = OBJECT.lock().unwrap();
    if color_vec.len() == 4 || color_vec.len() == 3 {
        object
            .transform(&Transform::Color(Some(ColorLike::Color([
                color_vec[0],
                color_vec[1],
                color_vec[2],
                255,
            ]))))
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
            let c = ColorLike::Color([
                paint_vec_option[0],
                paint_vec_option[1],
                paint_vec_option[2],
                255,
            ]);

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
pub fn add_svg(data: js_sys::Uint32Array, color: js_sys::Uint8ClampedArray) {
    // kind, x, y

    // 0     -> Move
    // 1     -> Line
    // 255.. -> Close

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

            Some(ColorLike::Color([r, g, b, a]))
        }
        _ => None,
    };

    let item = SvgItem::new(paths, color);

    console::log_1(&format!("{:?}", item).into());

    let mut binding = OBJECT.lock().unwrap();
    let types_like = binding.data_mut();
    if let TypesLike::Svg(svg_holder) = types_like {
        svg_holder.add_item(item);
    }
}
