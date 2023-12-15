use std::collections::{HashMap,hash_map::Entry};
use std::fmt::{Debug,Formatter};
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

    pub fn shift_south(&mut self) -> u64 {
        let mut weight: u64 = 0;
        for x in 0..self.width {
            let mut last_immovable = self.height - 1;
            for y in 0..self.height {
                let dy = self.height - y - 1;
                match coord![self, x, dy] {
                    b'#' => last_immovable = dy - 1,
                    b'O' => {
                        weight += last_immovable as u64;
                        coord![self, x, dy] = b'.';
                        coord![self, x, last_immovable] = b'O';
                        last_immovable -= 1;
                    }
                    _ => ()
                }
            }
        }

        weight
    }
    
    pub fn shift_west(&mut self) -> u64 {
        let mut weight: u64 = 0;
        for y in 0..self.height {
            let mut last_immovable = 0;
            for x in 0..self.width {
                match coord![self, x, y] {
                    b'#' => last_immovable = x + 1,
                    b'O' => {
                        weight += (self.width - last_immovable) as u64;
                        coord![self, x, y] = b'.';
                        coord![self, last_immovable, y] = b'O';
                        last_immovable += 1;
                    }
                    _ => ()
                }
            }
        }

        weight
    }

    pub fn shift_east(&mut self) -> u64 {
        let mut weight: u64 = 0;
        for y in 0..self.height {
            let mut last_immovable = self.width - 1;
            for x in 0..self.width {
                let dx = self.width - x - 1;
                match coord![self, dx, y] {
                    b'#' => last_immovable = dx - 1,
                    b'O' => {
                        weight += last_immovable as u64;
                        coord![self, dx, y] = b'.';
                        coord![self, last_immovable, y] = b'O';
                        last_immovable -= 1;
                    }
                    _ => ()
                }
            }
        }

        weight
    }

    pub fn north_weight(&self) -> u64 {
        let mut weight: u64 = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if coord![self, x, y] == b'O' {
                    weight += (self.height - y) as u64;
                }
            }
        }
        weight
    }

    pub fn spin(&mut self, iterations: u64) -> u64 {
        let mut cycles = HashMap::<(u64,u64,u64,u64),u64>::new();
        let mut north_weights = Vec::new();
        for i in 0..iterations {
            let n = self.shift_north();
            let w = self.shift_west();
            let s = self.shift_south();
            let e = self.shift_east();

            match cycles.entry((n, w, s, e)) {
                Entry::Occupied(entry) => {
                    let first_encountered = entry.get();
                    let last_cycle = (iterations - first_encountered) % (i - first_encountered) + first_encountered - 1;
                    return *north_weights.get(last_cycle as usize).expect("Oops? How?");

                },
                Entry::Vacant(entry) => {
                    entry.insert(i);
                    north_weights.push(self.north_weight());
                }
            };
        };

        *north_weights.last().expect("Should be at least one here...")
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.data.clone()).map_err(|_| std::fmt::Error)?)
    }    
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let mut vec = Vec::new();
    get_puzzle_input()?.read_to_end(&mut vec)?;

    let mut grid = Grid::new(vec);

    if cfg!(not(feature = "part2")) {
        let weight = grid.shift_north();
        println!("Answer: {}", weight);
    }
    else {
        let weight = grid.spin(1_000_000_000);
        println!("Answer: {}", weight);
    }

    Ok(())
}
