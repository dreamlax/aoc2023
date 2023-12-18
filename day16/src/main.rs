use std::collections::HashSet;
use std::io::Read;

use aoc_utils::prelude::*;

mod direction;
mod point;

use direction::Direction;
use point::Point;

#[derive(Clone)]
struct LaserBoard {
    grid: Vec<u8>,
    width: usize,
    height: usize
}

impl std::fmt::Debug for LaserBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board is {}x{}", self.width, self.height)?;
        writeln!(f, "{}", String::from_utf8(self.grid.clone()).expect("Should convert"))
    }
}

impl TryFrom<Vec<u8>> for LaserBoard {
    type Error = PuzzleError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        let width = data
            .iter()
            .position(|b| *b == b'\n')
            .ok_or(PuzzleError::from(PuzzleErrorKind::ParseError))?;

        assert_eq!(data.len() % width, 0, "Data is not gridular");
        let height = data.len() / width - 1;
        
        Ok(Self {
            grid: data,
            width,
            height
        })
    }
}

impl LaserBoard {
    /// Returns the number of "energised tiles"
    fn layzer_boim(&mut self, first: Direction) -> usize {
        let mut visited_special_tiles: HashSet<Point> = HashSet::new();
        
        macro_rules! coord(
            ($x: expr, $y: expr) => {
                self.grid[$y * (self.width + 1) + $x]
            };
        );

        let mut continuations: Vec<Direction> = vec![first];

        while let Some(direction) = continuations.pop() {
            let (mut origin, (dx, dy)) = match direction {
                Direction::Up(p) => (p, (0isize, -1isize)),
                Direction::Down(p) => (p, (0isize, 1isize)),
                Direction::Left(p) => (p, (-1isize, 0isize)),
                Direction::Right(p) => (p, (1isize, 0isize))
            };

            loop {
                if origin.x >= self.width || origin.y >= self.height {
                    break;
                }

                match coord![origin.x, origin.y] {
                    b'|' => {
                        if !visited_special_tiles.insert(origin.clone()) {
                            break;
                        }

                        if dx != 0 {
                            if let Ok(up) = origin.checked_cloned_offset(0, -1) {
                                continuations.push(Direction::Up(up));
                            }
                            if let Ok(down) = origin.checked_cloned_offset(0, 1) {
                                continuations.push(Direction::Down(down));
                            }
                            break;
                        }
                    },

                    b'-' => {
                        if !visited_special_tiles.insert(origin.clone()) {
                            break;
                        }

                        if dy != 0 {
                            if let Ok(left) = origin.checked_cloned_offset(-1, 0) {
                                continuations.push(Direction::Left(left));
                            }
                            if let Ok(right) = origin.checked_cloned_offset(1, 0) {
                                continuations.push(Direction::Right(right));
                            }
                            break;
                        }
                    }

                    b'/' => {
                        visited_special_tiles.insert(origin.clone());
                        
                        match (dx, dy) {
                            (0, -1) => if let Ok(right) = origin.checked_cloned_offset(1, 0) {
                                continuations.push(Direction::Right(right));
                            },
                            (0, 1) => if let Ok(left) = origin.checked_cloned_offset(-1, 0) {
                                continuations.push(Direction::Left(left));
                            },
                            (-1, 0) => if let Ok(down) = origin.checked_cloned_offset(0, 1) {
                                continuations.push(Direction::Down(down));
                            },
                            (1, 0) => if let Ok(up) = origin.checked_cloned_offset(0, -1) {
                                continuations.push(Direction::Up(up));
                            },
                            _ => panic!("hmmmm")
                        }
                        
                        break;
                    }

                    b'\\' => {
                        visited_special_tiles.insert(origin.clone());
                        
                        match (dx, dy) {
                            (0, -1) => if let Ok(left) = origin.checked_cloned_offset(-1, 0) {
                                continuations.push(Direction::Left(left));
                            },
                            (0, 1) => if let Ok(right) = origin.checked_cloned_offset(1, 0) {
                                continuations.push(Direction::Right(right));
                            },
                            (-1, 0) => if let Ok(up) = origin.checked_cloned_offset(0, -1) {
                                continuations.push(Direction::Up(up));
                            },
                            (1, 0) => if let Ok(down) = origin.checked_cloned_offset(0, 1) {
                                continuations.push(Direction::Down(down));
                            },
                            _ => panic!("hmmmm")
                        }

                        break;
                    }
                    
                    b'.' => coord![origin.x, origin.y] = b'#',

                    b'#' => (),

                    b => panic!("Impossibru: {b:?} at {:?}\n{:?}", origin, self)
                }

                if let Ok(new_origin) = origin.checked_offset(dx, dy) {
                    origin = new_origin;
                }
                else {
                    break;
                }
            }
        }

        self.grid.iter().filter(|b| **b == b'#').count() + visited_special_tiles.len()

    }
}


fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let mut buf = Vec::new();
    get_puzzle_input()?.read_to_end(&mut buf)?;
    let mut laser_board = LaserBoard::try_from(buf)?;

    if cfg!(not(feature = "part2")) {
        let answer = laser_board.layzer_boim(Direction::Right((0, 0).into()));
        println!("Answer: {answer}");
    }
    else {
        let width = laser_board.width;
        let height = laser_board.height;
        
        let up_maximum = (0..width)
            .into_iter()
            .map(|x|
                laser_board.clone().layzer_boim(Direction::Up((x, height - 1).into()))
            )
            .max()
            .unwrap_or(0);

        let down_maximum = (0..width)
            .into_iter()
            .map(|x|
                laser_board.clone().layzer_boim(Direction::Down((x, 0).into()))
            )
            .max()
            .unwrap_or(0);

        let left_maximum = (0..height)
            .into_iter()
            .map(|y|
                laser_board.clone().layzer_boim(Direction::Left((width - 1, y).into()))
            )
            .max()
            .unwrap_or(0);

        let right_maximum = (0..height)
            .into_iter()
            .map(|y|
                laser_board.clone().layzer_boim(Direction::Right((0, y).into()))
            )
            .max()
            .unwrap_or(0);

        let answer = up_maximum
            .max(down_maximum)
            .max(left_maximum)
            .max(right_maximum);

        println!("Answer: {}", answer);
    }

    Ok(())
}
