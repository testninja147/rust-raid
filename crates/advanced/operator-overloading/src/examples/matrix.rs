use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<f32>>);

/// ## Overloading the '*' operator for matrix multiplication
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.0;
        let b = rhs.0;
        if a[0].len() != b.len() {
            panic!("Matrix dimensions are incompatible")
        }
        let i = b[0].len();
        let j = b.len();
        let mut new_matrix = vec![vec![0f32; i]; j];
        for x in 0..i {
            for y in 0..j {
                // we first determine respective row and column to perform operation
                let row_x = a[x].clone();
                let col_y: Vec<f32> = b.iter().map(|v| v[y]).collect();

                // the zip, map(a*b) and reduce(sum) will perform steps 1 to 4 in each loop.
                new_matrix[x][y] = row_x
                    .iter()
                    .zip(col_y.iter())
                    .map(|(a, b)| a * b)
                    .reduce(|a, b| a + b)
                    .unwrap();
            }
        }
        Self(new_matrix)
    }
}
