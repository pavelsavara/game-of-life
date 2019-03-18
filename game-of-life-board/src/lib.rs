extern crate cfg_if;
extern crate itertools;
extern crate wasm_bindgen;

mod utils;

use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
	alert("Hello, !!wasm!");
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Position {
	x: i32,
	y: i32,
}

pub type Board = Vec<Position>;

const OFFSETS_AROUND: [[i32; 2]; 8] = [	[-1, -1],	[ 0, -1],	[ 1, -1],
										[-1,  0],				[ 1,  0],
										[-1,  1],	[ 0,  1],	[ 1,  1],
];

fn rules(count: usize, is_alive: bool) -> bool {
	count == 3 || (count == 2 && is_alive)
}

pub fn gol(generation: &Board) -> Board {
	generation
		.iter()
		.map(|current: &Position| {
			OFFSETS_AROUND
				.iter()
				.map(|o| Position {
					x: (current.x + o[0]),
					y: (current.y + o[1]),
				})
				.collect::<Board>()
		})
		.flatten()
		.sorted()
		.dedup()
		.filter(|current: &Position| {
			rules(
				OFFSETS_AROUND
					.iter()
					.map(|o| Position {
						x: (current.x + o[0]),
						y: (current.y + o[1]),
					})
					.filter(|q| generation.contains(q))
					.count(),
				generation.contains(current),
			)
		})
		.collect::<Board>()
}

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub fn render(generation: &Board) -> String {
	let avg_x: i32 = generation.iter().map(|p| p.x).sum::<i32>() / (generation.len() as i32);
	let avg_y: i32 = generation.iter().map(|p| p.y).sum::<i32>() / (generation.len() as i32);
	let mut view: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

	for point in generation.iter() {
		let shift_x: usize = (point.x - avg_x + (WIDTH / 2) as i32) as usize;
		let shift_y: usize = (point.y - avg_y + (HEIGHT / 2) as i32) as usize;
		if shift_x > 0 && shift_y > 0 && shift_x < WIDTH && shift_y < HEIGHT {
			view[shift_y][shift_x] = true;
		}
	}

	let mut sb = String::from("Hello, world!");

	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			if view[y][x] {
				sb.push('X');
			} else {
				sb.push(' ');
			}
		}
		sb.push('\n');
	}

	sb
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	use std::{thread, time};

	#[test]
	fn test_blinker() {
		let blinker: Board = vec![
			Position { x: 0, y: -1 },
			Position { x: 0, y: 0 },
			Position { x: 0, y: 1 },
		];
		println!("{:?}", blinker);

		let g2 = gol(&blinker);
		println!("{:?}", g2);
	}

	#[test]
	fn test_pentomimo() {
		let millis = time::Duration::from_millis(100);
		let pentomimo: Board = vec![
			Position { x: -1, y: 0 },
			Position { x: -1, y: 1 },
			Position { x: 0, y: -1 },
			Position { x: 0, y: 0 },
			Position { x: 1, y: 0 },
		];
		println!("{:?}", pentomimo);

		let mut board = pentomimo;
		for _ in 0..40 {
			board = gol(&board);
			let view = render(&board);
			print!("{}[2J", 27 as char);
			print!("{}", &view);
			thread::sleep(millis);
		}
	}
}
