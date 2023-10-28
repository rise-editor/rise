use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(y: T, x: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialOrd> Point<T> {
    pub fn order(p1: Point<T>, p2: Point<T>) -> (Self, Self) {
        if p1 <= p2 {
            (p1, p2)
        } else {
            (p2, p1)
        }
    }
}

impl<T: PartialOrd> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y < other.y {
            Some(Ordering::Less)
        } else if self.y == other.y && self.x < other.x {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}
