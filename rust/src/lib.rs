mod gol;
mod utils;
mod web;

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
extern crate web_sys;
//extern crate itertools;

//use itertools::Itertools;
use gol::*;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
lazy_static! {
    static ref BOARD: Mutex<gol::Cells> = Mutex::new(vec![]);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    return web::start();
}

#[wasm_bindgen()]
pub fn render_next() -> Result<(), JsValue> {
    let mut board = BOARD.lock().unwrap();

    *board = gol::next_gen(&board);

    let vertices: Vec<f32> = (*board)
        .iter()
        .map(|current: &Cell| {
            let zero = 0.00_f32;
            let grid = 0.01_f32;
            let size = 0.0080_f32;
            let fx0 = (current.x as f32) * grid;
            let fy0 = (current.y as f32) * grid;
            let fx1 = (current.x as f32) * grid + size;
            let fy1 = (current.y as f32) * grid + size;
            let vert = vec![
                fx0, fy0, zero,
                fx1, fy0, zero,
                fx0, fy1, zero,

                fx1, fy1, zero,
                fx0, fy1, zero,
                fx1, fy0, zero,
            ];
            vert
        })
        .flatten()
        .collect::<Vec<f32>>();

    return web::render(vertices);
}

#[wasm_bindgen()]
pub fn new_board() -> Result<(), JsValue> {
    let pentomimo: Cells = vec![
        Cell { x: -1, y: 0 },
        Cell { x: -1, y: 1 },
        Cell { x: 0, y: -1 },
        Cell { x: 0, y: 0 },
        Cell { x: 1, y: 0 },
    ];

    let mut cells = BOARD.lock().unwrap();
    *cells = pentomimo;

    return Ok(());
}
