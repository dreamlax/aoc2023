pub struct Point {
    pub x: isize,
    pub y: isize
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<(isize,isize)> for Point {
    fn from(value: (isize,isize)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}
