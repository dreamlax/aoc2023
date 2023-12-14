use std::io::Read;
use aoc_utils::prelude::*;

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

macro_rules! coord {
    [$v: ident, $x: expr, $y: expr] => {
        $v.data[$y * ($v.width + 1) + $x]
    };
}

impl Grid {
    pub fn new(data: Vec<u8>) -> Self {
        let width = data.iter().position(|b| *b == b'\n').expect("Missing newline");
        assert_eq!(data.len() % width, 0, "Data is not gridular");
        let height = (data.len() / width) - 1;
        Self {
            data,
            width,
            height
        }
    }

    pub fn shift_north(&mut self) -> u64 {
        let mut weight: u64 = 0;
        for x in 0..self.width {
            let mut last_immovable = 0;
            for y in 0..self.height {
                match coord![self, x, y] {
                    b'#' => last_immovable = y + 1,
                    b'O' => {
                        weight += (self.height - last_immovable) as u64;
                        coord![self, x, y] = b'.';
                        coord![self, x, last_immovable] = b'O';
                        last_immovable += 1;
                    }
                    _ => ()
                }
            }
        }

        weight
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let mut vec = Vec::new();
    get_puzzle_input()?.read_to_end(&mut vec)?;

    let mut grid = Grid::new(vec);
    let weight = grid.shift_north();

    println!("Answer: {}", weight);

    Ok(())
}
