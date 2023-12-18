use crate::point::Point;

pub enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point)
}
