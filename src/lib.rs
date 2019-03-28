mod gol;
mod utils;
mod web;

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
extern crate web_sys;
//extern crate itertools;

//use itertools::Itertools;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use gol::*;
lazy_static! {
    static ref BOARD: Mutex<gol::Board> = Mutex::new(vec![]);
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    return web::start();
}

#[wasm_bindgen()]
pub fn render() -> Result<(), JsValue> {
    let mut board = BOARD.lock().unwrap();

    *board = gol::gol(&board);

    let vertices:Vec<f32> = (*board).iter()
    .map(|current: &Position| {
        let fx0=current.x as f32;
        let fy0=current.y as f32;
        let fx1=(current.x+1) as f32;
        let fy1=(current.y+1) as f32;
        let size=0.1_f32;
        let zero=0.00_f32;
        let vert = vec![fx0*size,fy0*size,zero,
                        fx1*size,fy0*size,zero,
                        fx0*size,fy1*size,zero];
        vert
    })    
    .flatten()
    .collect::<Vec<f32>>();

    return web::render(vertices);
}

#[wasm_bindgen()]
pub fn new_board() -> Result<(), JsValue> {
    let pentomimo: Board = vec![
        Position { x: -1, y: 0 },
        Position { x: -1, y: 1 },
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
    ];

    let mut board = BOARD.lock().unwrap();
    *board = pentomimo;
    
    return Ok(());
}