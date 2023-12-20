#[derive(Clone,Debug,Eq,PartialEq,Hash,Ord,PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T, U> From<(U, U)> for Point<T> where T: From<U> {
    fn from(value: (U, U)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into()
        }
    }
}

impl Point<usize> {
    pub fn checked_offset(&self, dx: isize, dy: isize) -> Option<Self> {
        if let (Some(new_x), Some(new_y)) = (self.x.checked_add_signed(dx), self.y.checked_add_signed(dy)) {
            Some(Self {
                x: new_x,
                y: new_y
            })
        }
        else {
            None   
        }
    }
}
