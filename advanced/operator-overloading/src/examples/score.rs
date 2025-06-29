use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub goals: i8,
    pub penalties: i8,
}

///
/// ## Overloading the '+' operator
/// If
impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            goals: self.goals + rhs.goals,
            penalties: self.penalties + rhs.penalties,
        }
    }
}

impl Sub for Score {
    type Output = Score;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            goals: self.goals - rhs.goals,
            penalties: self.penalties - rhs.penalties,
        }
    }
}
