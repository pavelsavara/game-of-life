mod gol;
mod utils;
mod web;

extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    return web::start();
}

#[wasm_bindgen()]
pub fn render(n:f32) -> Result<(), JsValue> {
    return web::render(n);
}