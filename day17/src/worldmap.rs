use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use aoc_utils::prelude::*;
use crate::point::Point;

trait CloneAndPush<T> where T: Clone {
    fn clone_and_push(&self, t: &T) -> Self;
}

impl<T> CloneAndPush<T> for Vec<T> where T: Clone {
    fn clone_and_push(&self, t: &T) -> Self {
        if !cfg!(debug_assertions) {
            Vec::new()
        }
        else {
            let mut clone = self.clone();
            clone.push(t.clone());
            clone
        }
    }
}

pub struct WorldMap {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Debug,Hash,Eq,PartialEq,Ord,PartialOrd,Clone)]
struct Position {
    point: Point<usize>,
    dx: isize,
    dy: isize
}

impl Position {
    fn is_stationary(&self) -> bool {
        self.dx == 0 && self.dy == 0
    }
}

#[derive(Eq,PartialEq)]
pub struct MapState {
    cost: u32,
    path: Vec<Point<usize>>,
    position: Position
}

impl Ord for MapState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost
            .cmp(&self.cost)
    }
}

impl PartialOrd for MapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl WorldMap {
    fn get_ultra_crucible_next_cells(&self, position: &Position) -> Vec<Position> {
        let mut new_possible_positions = Vec::new();

        let can_change_direction = position.is_stationary() || position.dx.abs() >= 4 || position.dy.abs() >= 4;

        // go north
        if (position.dy < 0 && position.dy > -10) || (position.dy == 0 && can_change_direction) {
            if let Some(point) = position.point.checked_offset(0, -1) {
                new_possible_positions.push(Position { point, dx: 0, dy: position.dy - 1 });
            }
        }

        // go south
        if (position.dy > 0 && position.dy < 10) || (position.dy == 0 && can_change_direction) {
            if let Some(point) = position.point.checked_offset(0, 1) {
                if point.y < self.height {
                    new_possible_positions.push(Position { point, dx: 0, dy: position.dy + 1 });
                }
            }
        }

        // go east
        if (position.dx > 0 && position.dx < 10) || (position.dx == 0 && can_change_direction) {
            if let Some(point) = position.point.checked_offset(1, 0) {
                if point.x < self.width {
                    new_possible_positions.push(Position { point, dx: position.dx + 1, dy: 0 });
                }
            }
        }

        // go west
        if (position.dx < 0 && position.dx > -10) || (position.dx == 0 && can_change_direction) {
            if let Some(point) = position.point.checked_offset(-1, 0) {
                new_possible_positions.push(Position { point, dx: position.dx - 1, dy: 0 });
            }
        }

        new_possible_positions
    }

    fn get_normal_crucible_next_cells(&self, position: &Position) -> Vec<Position> {
        let mut new_possible_positions = Vec::new();

        // go north
        if position.dy <= 0 && position.dy > -3 {
            if let Some(point) = position.point.checked_offset(0, -1) {
                new_possible_positions.push(Position { point, dx: 0, dy: position.dy - 1 });
            }
        }

        // go south
        if position.dy >= 0 && position.dy < 3 {
            if let Some(point) = position.point.checked_offset(0, 1) {
                if point.y < self.height {
                    new_possible_positions.push(Position { point, dx: 0, dy: position.dy + 1 });
                }
            }
        }

        // go east
        if position.dx >= 0 && position.dx < 3 {
            if let Some(point) = position.point.checked_offset(1, 0) {
                if point.x < self.width {
                    new_possible_positions.push(Position { point, dx: position.dx + 1, dy: 0 });
                }
            }
        }

        // go west
        if position.dx <= 0 && position.dx > -3 {
            if let Some(point) = position.point.checked_offset(-1, 0) {
                new_possible_positions.push(Position { point, dx: position.dx - 1, dy: 0 });
            }
        }

        new_possible_positions
    }

    pub fn find_best_route(&self, start: Point<usize>, end: Point<usize>) -> u32 {
        let mut best_costs: HashMap<Position,u32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        macro_rules! cell_cost {
            [$x: expr, $y: expr] => {
                (self.grid[$y * (self.width + 1) + $x] - b'0') as u32
            };

            [$p: expr] => {
                (self.grid[$p.y * (self.width + 1) + $p.x] - b'0') as u32
            };
        }

        let position = Position { point: start.clone(), dx: 0, dy: 0 };

        best_costs.insert(position.clone(), 0);

        heap.push(MapState {
            cost: 0, // the rules say you don't incur the cost of the starting node
            position: position,
            path: Vec::from([start.clone()])
        });

        while let Some(MapState { cost, position, path }) = heap.pop() {
            if position.point == end {
                #[cfg(debug_assertions)]
                for y in 0..self.height {
                    for x in 0..self.width {
                        if path.contains(&Point { x, y }) {
                            print!("[{:01}]", cell_cost![x, y]);
                        }
                        else {
                            print!(" {:01} ", cell_cost![x, y]);
                        }
                    }
                    println!();
                }
                return cost;
            }

            let best = best_costs.get(&position).unwrap_or(&u32::MAX);
            if cost > *best{
                continue;
            }

            let new_possible_positions = if cfg!(feature = "part2")
            {
                self.get_ultra_crucible_next_cells(&position)
            }
            else
            {
                self.get_normal_crucible_next_cells(&position)
            };

            for new_position in &new_possible_positions {
                let position_cost = cost + cell_cost![new_position.point];
                let best_cost = best_costs.get(new_position).unwrap_or(&u32::MAX);
                if position_cost < *best_cost {
                    best_costs.insert(new_position.clone(), position_cost);
                    heap.push(MapState {
                        cost: position_cost,
                        path: path.clone_and_push(&new_position.point),
                        position: new_position.clone()
                    });
                }
            }
        }

        panic!("Impossibru route!");
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl TryFrom<Vec<u8>> for WorldMap {
    type Error = PuzzleError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let width = value.iter()
            .position(|b| *b == b'\n')
            .ok_or_else(|| PuzzleErrorKind::ParseError)?;

        if value.len() % width != 0 {
            return Err(PuzzleErrorKind::InputError.into());
        }

        let height = value.len() / width - 1;
        Ok(Self {
            grid: value,
            width,
            height
        })
    }
}
