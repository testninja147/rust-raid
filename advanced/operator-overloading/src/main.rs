mod examples;
use crate::examples::{matrix::Matrix, score::Score, vector::Vector};

// use examples::{};

/// ! to run, execute `cargo run --bin struct-overloading`

///
/// # Operator Overloading
///
/// Operator Overloading is a process of implementing basic Arithmetic
/// operations to change the behavior of operators. It is also useful when we
/// want to implement arithmetic operations on those types that do not natively
/// support it. We can use Operator overloading to our user defined data types
/// using traits that implement add, sub, etc.
///
/// The below are some of examples of operator overloading:
/// - Addition and of Subtraction of structs. (Example1: Score struct)
/// - Matrix Multiplication
/// - Scalar Multiplication of a 2d vector (heterogenous data type example)
///
fn main() {
    println!("⛔ Operator Overloading ⛔");
    //==========================================================================
    // Example  1: Overloading `+` and `-` signs in a struct
    // Please check: examples/score.rs for implementation
    //==========================================================================
    {
        let score_a = Score {
            goals: 2,
            penalties: 2,
        };
        let score_b = Score {
            goals: 1,
            penalties: 3,
        };
        let total_scores = score_a + score_b;
        let net_score_a = score_a - score_b;
        let net_score_b = score_b - score_a;

        println!("Net Score by A: {:?}", net_score_a);
        println!("Total Scores: {:?}", total_scores);
        println!("Net Score by B: {:?}", net_score_b);

        //Total Scores: Score { goals: 3, penalties: 5 }
        //Net Score by A: Score { goals: 1, penalties: -1 }
        //Net Score by B: Score { goals: -1, penalties: 1 }
    }
    //==========================================================================
    // Example  2: Matrix Multiplication
    // Overloading `*` sign to perform matrix multiplication
    // Please check: examples/matrix.rs for implementation
    //==========================================================================
    {
        let m1 = Matrix(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m2 = Matrix(vec![vec![1.0, 3.0], vec![2.0, 1.0]]);

        let m3 = m1 * m2;

        println!("Matrix multiplication result:\n{:?}", m3);
        // Matrix multiplication result:
        // Matrix([[5.0, 5.0], [11.0, 13.0]])
    }

    //==========================================================================
    // Example  3: Scalar Multiplication of a vector
    // Overloading `*`  with Heterogenous data types.
    // Please check: examples/vector.rs for implementation
    //==========================================================================
    {
        let v1 = Vector { x: 10, y: 20 };

        let v2 = v1 * 5; // multiply heterogeneous value
        let v3 = 5 * v1; // multiply heterogeneous value (reverse order)

        println!("5 * v1 = {v2:?}");
        println!("v1 * 5 = {v3:?}");
        // 5 * v1 = Vector { x: 50, y: 100 }
        // v1 * 5 = Vector { x: 50, y: 100 }
    }
    //==========================================================================
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
