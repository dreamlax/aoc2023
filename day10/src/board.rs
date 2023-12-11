use std::io::BufRead;
use aoc_utils::prelude::*;
use crate::tiletype::TileType;

#[derive(Clone,Debug,Eq,PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl From<(usize,usize)> for Coord {
    fn from(value: (usize,usize)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}

pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
    start: Coord
}

pub struct Surrounding<'a> {
    n: Option<&'a TileType>,
    e: Option<&'a TileType>,
    s: Option<&'a TileType>,
    w: Option<&'a TileType>,
}


pub trait GetTile<T> {
    fn get_tile(&self, c: &T) -> Option<&TileType>;
}

impl GetTile<Coord> for Board {
    fn get_tile(&self, c: &Coord) -> Option<&TileType> {
        self.get_tile(c.x, c.y)
    }
}

pub trait GetSurroundingTiles<'a, T> {
    fn get_surrounding_tiles(&'a self, c: &T) -> Surrounding<'a>;
}

impl<'a> GetSurroundingTiles<'a, Coord> for Board {
    fn get_surrounding_tiles(&'a self, c: &Coord) -> Surrounding<'a> {
        self.get_surrounding_tiles(c.x, c.y)
    }
}

impl Board {
    pub fn determine_tile_type(&self, x: usize, y: usize) -> TileType {
        let surrounding = self.get_surrounding_tiles(x, y);

        let north_goes_south = surrounding.n.is_some_and(|t| t.leads_south());
        let south_goes_north = surrounding.s.is_some_and(|t| t.leads_north());
        let east_goes_west = surrounding.e.is_some_and(|t| t.leads_west());
        let west_goes_east = surrounding.w.is_some_and(|t| t.leads_east());

        match (north_goes_south, east_goes_west, south_goes_north, west_goes_east) {
            (true, true, false, false) => TileType::NE,
            (false, true, true, false) => TileType::SE,
            (false, false, true, true) => TileType::SW,
            (true, false, false, true) => TileType::NW,
            (true, false, true, false) => TileType::NS,
            (false, true, false, true) => TileType::EW,
            _ => panic!("Invalid combination")
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TileType> {
        let idx = y * self.width + x;
        self.tiles.get(idx)
    }

    pub fn get_surrounding_tiles(&self, x: usize, y: usize) -> Surrounding {
        Surrounding {
            n: if y == 0 { None } else { self.get_tile(x, y - 1) },
            e: if x == self.width - 1 { None } else { self.get_tile(x + 1, y) },
            s: if y == self.height - 1 { None } else { self.get_tile(x, y + 1) },
            w: if x == 0 { None } else { self.get_tile(x - 1, y) }
        }
    }

    pub fn try_from<T>(reader: T) -> PuzzleResult<Self> where T: BufRead  {
        let mut start: Option<Coord> = None;
        let mut width: Option<usize> = None;
        let mut height = 0;
        let mut tiles = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.len() == 0 {
                continue;
            }

            let line = line
                .chars()
                .map(TileType::try_from)
                .collect::<Result<Vec<TileType>,_>>()?;

            if start.is_none() {
                start = line
                    .iter()
                    .position(|t| *t == TileType::Start)
                    .and_then(|s| Some(Coord::from((s, height))));
            }

            if width.is_none() {
                width = Some(line.len());
            }
            else if width.unwrap() != line.len() {
                return Err(PuzzleErrorKind::ParseError.into());
            }

            tiles.extend(line);
            height += 1;
        }

        let width = width
            .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::ParseError))?;

        let start = start
            .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::ParseError))?;

        Ok(Self {
            width,
            height,
            tiles,
            start
        })
    }

    pub fn find_furthest_distance(&self) -> usize {
        let mut current_len = 0;
        let start_tile = self.determine_tile_type(self.start.x, self.start.y);

        // pick the first direction, favour east, north, then south (for no raisin)
        let (mut current_pos, mut last_move) = match start_tile {
            TileType::NE => (Coord::new(self.start.x + 1, self.start.y), Direction::East),
            TileType::EW => (Coord::new(self.start.x + 1, self.start.y), Direction::East),
            TileType::SE => (Coord::new(self.start.x + 1, self.start.y), Direction::East),
            TileType::NS => (Coord::new(self.start.x, self.start.y - 1), Direction::North),
            TileType::NW => (Coord::new(self.start.x, self.start.y - 1), Direction::North),
            TileType::SW => (Coord::new(self.start.x, self.start.y + 1), Direction::South),
            t => panic!("Unexpected tile type: {:?}", t)
        };

        // work through the loop
        while current_pos != self.start {
            current_len += 1;

            // figure out what tile we're currently sitting on
            let tile = self.get_tile(current_pos.x, current_pos.y).unwrap();

            // pick the next direction based on the current tile and our last move
            let next_direction = match (tile, last_move.clone()) {
                (TileType::NS, Direction::South) => Direction::South,
                (TileType::NS, Direction::North) => Direction::North,
                (TileType::EW, Direction::West) => Direction::West,
                (TileType::EW, Direction::East) => Direction::East,
                (TileType::NE, Direction::West) => Direction::North,
                (TileType::NE, Direction::South) => Direction::East,
                (TileType::NW, Direction::East) => Direction::North,
                (TileType::NW, Direction::South) => Direction::West,
                (TileType::SE, Direction::North) => Direction::East,
                (TileType::SE, Direction::West) => Direction::South,
                (TileType::SW, Direction::North) => Direction::West,
                (TileType::SW, Direction::East) => Direction::South,
                (t, d) => panic!("Unexpected tile {:?} and origin {:?}", t, d)
            };

            #[cfg(debug_assertions)]
            println!("Last move was {:?} onto {:?} ({}x{}), moving {:?}", last_move, tile, current_pos.x, current_pos.y, next_direction);

            // move to that next position
            current_pos = match next_direction {
                Direction::North => Coord::new(current_pos.x, current_pos.y - 1),
                Direction::East => Coord::new(current_pos.x + 1, current_pos.y),
                Direction::South => Coord::new(current_pos.x, current_pos.y + 1),
                Direction::West => Coord::new(current_pos.x - 1, current_pos.y)
            };
            last_move = next_direction
        }
        
        current_len / 2 + 1
    }

    // shouldn't really copy/paste the code from above but I'm lazy right now
    pub fn count_enclosed_spaces(&self) -> usize {
        let mut path: Vec<Option<i32>> = Vec::new();
        path.resize_with(self.width * self.height, Default::default);

        let start_tile = self.determine_tile_type(self.start.x, self.start.y);

        // pick the first direction, favour east, north, then south (for no raisin)
        let (mut current_pos, mut last_move, weight) = match start_tile {
            TileType::NE => (Coord::new(self.start.x + 1, self.start.y), Direction::East, 0),
            TileType::EW => (Coord::new(self.start.x + 1, self.start.y), Direction::East, 0),
            TileType::SE => (Coord::new(self.start.x + 1, self.start.y), Direction::East, 0),
            TileType::NS => (Coord::new(self.start.x, self.start.y - 1), Direction::North, 2),
            TileType::NW => (Coord::new(self.start.x, self.start.y - 1), Direction::North, 1),
            TileType::SW => (Coord::new(self.start.x, self.start.y + 1), Direction::South, -1),
            t => panic!("Unexpected tile type: {:?}", t)
        };

        path[self.start.y * self.height + self.start.x] = Some(weight);

        // work through the loop
        while current_pos != self.start {
            let (x, y) = (current_pos.x, current_pos.y);
            // figure out what tile we're currently sitting on
            let tile = self.get_tile(current_pos.x, current_pos.y).unwrap();

            // pick the next direction based on the current tile and our last move
            let (next_direction, weight) = match (tile, last_move.clone()) {
                (TileType::NS, Direction::South) => (Direction::South, -2),
                (TileType::NS, Direction::North) => (Direction::North, 2),
                (TileType::EW, Direction::West) => (Direction::West, 0),
                (TileType::EW, Direction::East) => (Direction::East, 0),
                (TileType::NE, Direction::West) => (Direction::North, 1),
                (TileType::NE, Direction::South) => (Direction::East, -1),
                (TileType::NW, Direction::East) => (Direction::North, 1),
                (TileType::NW, Direction::South) => (Direction::West, -1),
                (TileType::SE, Direction::North) => (Direction::East, 1),
                (TileType::SE, Direction::West) => (Direction::South, -1),
                (TileType::SW, Direction::North) => (Direction::West, 1),
                (TileType::SW, Direction::East) => (Direction::South, -1),
                (t, d) => panic!("Unexpected tile {:?} and origin {:?}", t, d)
            };

            path[y * self.width + x] = Some(weight);

            #[cfg(debug_assertions)]
            println!("Last move was {:?} onto {:?} ({}x{}), moving {:?}", last_move, tile, current_pos.x, current_pos.y, next_direction);

            // mark the current position (if required) and then move there
            current_pos = match next_direction {
                Direction::North => Coord::new(current_pos.x, current_pos.y - 1),
                Direction::South => Coord::new(current_pos.x, current_pos.y + 1),
                Direction::East => Coord::new(current_pos.x + 1, current_pos.y),
                Direction::West => Coord::new(current_pos.x - 1, current_pos.y)
            };

            last_move = next_direction
        }

        // work out how many tiles are enclosed using even-odd rule - except
        // that since we have half weights, we need to divide by 2 to work out
        // whether the even-off rule applies
        let mut enclosed = 0;
        for y in 0..self.height {
            let mut ups_downs: i32 = 0;
            for x in 0..self.width {
                let pc = path[y * self.width + x];
                ups_downs += pc.unwrap_or_default();

                if ((ups_downs / 2) % 2).abs() == 1 && pc.is_none() {
                    // we need to count all tiles even if they are not ground
                    // tiles, but we can't include any tiles used by the main
                    // pipe loop itself, but we know if it's part of the loop
                    // because it would have set an up_down weight.
                    #[cfg(debug_assertions)]
                    print!("<X>");
                    enclosed += 1;
                }
                else {
                    #[cfg(debug_assertions)]
                    print!("{:+} ", pc.unwrap_or_default());
                }
            }
            #[cfg(debug_assertions)]
            println!();
        }

        enclosed
    }
}
