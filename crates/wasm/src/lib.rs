use std::collections::VecDeque;
use std::sync::Mutex;

use lazy_static::{lazy_static, __Deref};
use rusvid_core::holder::likes::{
    color_like::ColorLike, path_like::PathLike, types_like::TypesLike,
};
use rusvid_core::holder::object::Object;
use rusvid_core::holder::svg_holder::{SvgHolder, SvgItem};
use rusvid_core::plane::Plane;
use rusvid_core::point::Point;
use wasm_bindgen::prelude::*;
use web_sys::console;

lazy_static! {
    static ref HOLDER: Mutex<SvgHolder> = Mutex::new(SvgHolder::new());

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
    *WIDTH.lock().unwrap().deref()
}

#[wasm_bindgen]
pub fn get_height() -> u32 {
    *HEIGHT.lock().unwrap().deref()
}

#[wasm_bindgen]
pub fn render() -> js_sys::Uint8ClampedArray {
    let width = *WIDTH.lock().unwrap().deref();
    let height = *HEIGHT.lock().unwrap().deref();

    let holder = HOLDER.lock().unwrap();
    let object = Object::new(TypesLike::Svg(
        holder.deref().clone()
    ));

    let plane = object.render(width, height).unwrap();
    let data = plane.as_data();
    
    let my_data = data.iter().flatten().map(|x| *x).collect::<Vec<u8>>();
    
    js_sys::Uint8ClampedArray::from(&my_data[..])
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
            },
            1 => {
                let x = as_vec.pop_front().unwrap();
                let y = as_vec.pop_front().unwrap();
    
                paths.push(PathLike::Line(Point::new(x as f64, y as f64)));
            },
            255.. => {
                paths.push(PathLike::Close);
            },
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

            Some(ColorLike::Color([r,g,b,a]))
        },
        _ => None
    };

    let item = SvgItem::new(paths, color);

    console::log_1(&format!("{:?}", item).into());

    let mut binding = HOLDER.lock();
    let holder = binding.as_mut().unwrap();
    holder.add_item(item);
}
