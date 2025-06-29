use std::ops::{Add, Sub};

/// ! to run, execute `cargo run --bin struct-overloading`

///
/// # Operator Overloading
///
/// Operator Overloading is a process of implementing basic Arithmetic
/// operations to change the behavior of operators. It is also useful when we
/// want to implement arithmetic operations on those types that do not natively
/// support it. One of the example is struct.
///

#[derive(Debug, Clone, Copy)]
struct Score {
    goals: i8,
    penalties: i8,
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
fn main() {
    println!("⛔ Operator Overloading ⛔");
    let score_a = Score {
        goals: 2,
        penalties: 2,
    };
    let score_b = Score {
        goals: 1,
        penalties: 3,
    };

    {
        // Addition
        let total_scores = score_a + score_b;
        println!("Total Scores: {:?}", total_scores);
        //Total Scores: Score { goals: 3, penalties: 5 }
    }

    {
        // Subtraction
        let net_score_a = score_a - score_b;
        println!("Net Score by A: {:?}", net_score_a);
        //Net Score by A: Score { goals: 1, penalties: -1 }

        let net_score_b = score_b - score_a;
        println!("Net Score by B: {:?}", net_score_b);
        //Net Score by B: Score { goals: -1, penalties: 1 }
    }
}

#[cfg(test)]
mod test {
    use crate::Score;

    #[test]
    fn test_addition() {
        let a = Score {
            goals: 5,
            penalties: 5,
        };
        let b = Score {
            goals: 5,
            penalties: 5,
        };
        let c = a + b;

        assert_eq!(a.goals + b.goals, c.goals);
        assert_eq!(a.penalties + b.penalties, c.penalties);
    }

    #[test]
    fn test_subtraction() {
        let a = Score {
            goals: 5,
            penalties: 5,
        };
        let b = Score {
            goals: 5,
            penalties: 5,
        };
        let c = a - b;

        assert_eq!(a.goals - b.goals, c.goals);
        assert_eq!(a.penalties - b.penalties, c.penalties);
    }
}
