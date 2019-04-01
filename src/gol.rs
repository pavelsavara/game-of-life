extern crate itertools;
use self::itertools::Itertools;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

pub type Cells = Vec<Cell>;

pub type BoolCell = (bool, Cell);

pub type BoolCells = Vec<BoolCell>;

const OFFSETS_AROUND: [[i32; 2]; 9] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [0, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

pub fn cells_around(current: &Cell) -> BoolCells {
    OFFSETS_AROUND.iter()
        .map(|o| {
            (
                // 0th element of the tuple is true when this is the original cell location
                o[0] == 0 && o[1] == 0,
                Cell {
                    x: (current.x + o[0]),
                    y: (current.y + o[1]),
                },
            )
        })
        .collect::<BoolCells>()
}

fn should_live<I>(group: I) -> bool where I: IntoIterator<Item = BoolCell>,
{
    // for each candidate location group
    // find if it lived in prev generation
    // and how many other live cells spilled candidate into it
    let (was_alive, count) = group.into_iter()
            .fold((false, 0), |acc, candidate| match candidate.0 {
                true => (true, acc.1),
                false => (acc.0, acc.1 + 1),
            });
    // apply game of life rules
    count == 3 || (count == 2 && was_alive)
}

pub fn next_gen(generation: &Cells) -> Cells {
    // generate 9 cell candidate tuples for each cell of previous generation
    let candidates_with_duplicates = generation.iter()
        .map(cells_around)
        .flatten();
    
    // sort and group cell candidates by location
    let grouped_by_location = candidates_with_duplicates
        .sorted_by(|candidate_a: &BoolCell, candidate_b: &BoolCell| candidate_a.1.cmp(&candidate_b.1))
        .group_by(|candidate: &BoolCell| candidate.1.clone());
    
    grouped_by_location.into_iter()
        .map(|(key, group)| {
            let alive = should_live(group.into_iter());
            let cell =key.clone();
            (alive, cell)
        })
        .filter(|c| c.0)
        .map(|c| c.1)
        .collect::<Cells>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    const WIDTH: usize = 30;
    const HEIGHT: usize = 30;

    pub fn render(generation: &Cells) -> String {
        let population = generation.len();
        if population == 0 {
            return String::from("All died");
        }

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

        let mut sb = format!("{:?}", population);

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

    #[test]
    fn test_blinker() {
        let blinker: Cells = vec![
            Cell { x: 0, y: -1 },
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
        ];
        println!("{:?}", blinker);

        let g2 = next_gen(&blinker);
        println!("{:?}", g2);
    }

    #[test]
    fn test_pentomimo() {
        let millis = time::Duration::from_millis(100);
        let pentomimo: Cells = vec![
            Cell { x: -1, y: 0 },
            Cell { x: -1, y: 1 },
            Cell { x: 0, y: -1 },
            Cell { x: 0, y: 0 },
            Cell { x: 1, y: 0 },
        ];
        println!("{:?}", pentomimo);

        let mut cells = pentomimo;
        for _ in 0..40 {
            cells = next_gen(&cells);
            let view = render(&cells);
            print!("{}[2J", 27 as char);
            print!("{}", &view);
            thread::sleep(millis);
        }
    }
}
