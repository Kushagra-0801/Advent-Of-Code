use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Idx {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdxDelta {
    pub row: isize,
    pub col: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sidx {
    pub row: isize,
    pub col: isize,
}

impl Add for IdxDelta {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<IdxDelta> for Idx {
    type Output = Self;

    fn add(self, rhs: IdxDelta) -> Self::Output {
        Self {
            row: (self.row as isize + rhs.row) as usize,
            col: (self.col as isize + rhs.col) as usize,
        }
    }
}

impl Add<IdxDelta> for Sidx {
    type Output = Self;

    fn add(self, rhs: IdxDelta) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Idx {
    type Output = IdxDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        IdxDelta {
            row: self.row as isize - rhs.row as isize,
            col: self.col as isize - rhs.col as isize,
        }
    }
}

impl Sub for Sidx {
    type Output = IdxDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        IdxDelta {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}
