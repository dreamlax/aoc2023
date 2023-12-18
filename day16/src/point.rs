use aoc_utils::prelude::*;

#[derive(Eq,PartialEq,Hash,Clone,Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn checked_offset(mut self, dx: isize, dy: isize) -> PuzzleResult<Self> {
        self.x = self.x.checked_add_signed(dx).ok_or(PuzzleErrorKind::IntegerError)?;
        self.y = self.y.checked_add_signed(dy).ok_or(PuzzleErrorKind::IntegerError)?;
        Ok(self)
    }

    pub fn checked_cloned_offset(&self, dx: isize, dy: isize) -> PuzzleResult<Self> {
        self.clone().checked_offset(dx, dy)
    }
}

impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Self {
            x: p.0,
            y: p.1
        }
    }
}
