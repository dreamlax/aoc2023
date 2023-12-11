use std::ops::Range;

pub trait Overlaps<T> {
    fn overlaps(&self, r: &Range<T>) -> bool;
}

impl<T> Overlaps<T> for Range<T> where T: Ord + Copy {
    fn overlaps(&self, r: &Range<T>) -> bool {
        self.start <= r.start && r.end < self.end
    }
}

pub trait Intersect<T> {
    fn intersect(&self, r: &Range<T>) -> Option<Range<T>>;
}

impl<T> Intersect<T> for Range<T> where T: Ord + Copy {
    fn intersect(&self, r: &Range<T>) -> Option<Range<T>> {
        if self.overlaps(r) {
            Some(r.start..r.end)
        }
        else if r.overlaps(self) {
            Some(self.start..self.end)
        }
        else if self.start <= r.start && self.end > r.start {
            Some(r.start..self.end)
        }
        else if r.start <= self.start && self.start < r.end {
            Some(self.start..r.end)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let left = 0..15;
        let right = 11..45;
        assert_eq!(left.intersect(&right), Some(11..15));
    }

    #[test]
    fn test_reverse_intersection() {
        let left = 11..45;
        let right = 0..15;
        assert_eq!(left.intersect(&right), Some(11..15));
    }

    #[test]
    fn test_left_overlaps_right() {
        let left = 0..100;
        let right = 20..30;
        assert_eq!(left.intersect(&right), Some(20..30));
    }

    #[test]
    fn test_right_overlaps_left() {
        let left = 20..30;
        let right = 0..100;
        assert_eq!(left.intersect(&right), Some(20..30));
    }

    #[test]
    fn test_no_intersection() {
        let left = 0..10;
        let right = 20..30;
        assert_eq!(left.intersect(&right), None);
    }
}
