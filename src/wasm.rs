use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// units = inches;
pub fn dot(
    width: f32,
    height: f32,
    margin: f32,
    r: u8,
    g: u8,
    b: u8,
    num_x: u32,
    num_y: u32,
    d: f32,
    num_pages: usize,
) -> Vec<u8> {
    let mut result = Cursor::new(Vec::new());
    crate::dot(
        &mut result,
        crate::Config {
            width,
            height,
            margin,
            color: (r, g, b),
            num_x,
            num_y,
            d,
            num_pages,
        },
    )
    .ok();

    result.into_inner()
}

#[wasm_bindgen]
/// units = inches;
pub fn grid(
    width: f32,
    height: f32,
    margin: f32,
    r: u8,
    g: u8,
    b: u8,
    num_x: u32,
    num_y: u32,
    d: f32,
    num_pages: usize,
) -> Vec<u8> {
    let mut result = Cursor::new(Vec::new());
    crate::grid(
        &mut result,
        crate::Config {
            width,
            height,
            margin,
            color: (r, g, b),
            num_x,
            num_y,
            d,
            num_pages,
        },
    )
    .ok();

    result.into_inner()
}
