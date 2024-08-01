use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    fn augmented_matrix(&self) -> Matrix<K> {
        let mut augmented = Matrix {
            values: vec![vec![K::zero(); self.values.len() * 2]; self.values.len()],
        };

        for i in 0..self.values.len() {
            for j in 0..self.values.len() {
                augmented.values[i][j] = self.values[i][j];
            }
            augmented.values[i][i + self.values.len()] = K::one();
        }
        augmented
    }

    fn inverse_from_augmented(source: &Matrix<K>, comparison: &Matrix<K>) -> Matrix<K> {
        let base_length = comparison.values.len();
        let mut inverse = Matrix {
            values: vec![vec![K::zero(); base_length]; base_length],
        };
        for i in 0..base_length {
            for j in 0..base_length {
                inverse.values[i][j] = source.values[i][j + base_length];
            }
        }
        inverse
    }

    pub fn inverse(&self) -> Result<Matrix<K>, String> {
        if !self.is_square() {
            return Err("Matrix must be square to have an inverse".to_string());
        }
        let mut augmented = self.augmented_matrix().reduced_row_echelon();
        for i in 0..self.values.len() {
            if augmented.values[i][i] == K::zero() {
                return Err("Matrix is not invertible".to_string());
            }
        }
        for i in (0..self.values.len()).rev() {
            for j in (0..i).rev() {
                let factor = augmented.values[j][i];
                augmented.subtract_multiple_of_row(i, j, factor);
            }
        }
        Ok(Self::inverse_from_augmented(&augmented, &self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let m = Matrix {
            values: vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]],
        };

        assert_eq!(
            m.inverse().unwrap_err(),
            "Matrix is not invertible".to_string()
        );
    }

    #[test]
    fn test_simple_inverse() {
        let m = Matrix {
            values: vec![vec![1., 0.], vec![0., 1.]],
        };

        let inv = m.inverse().unwrap();

        assert_eq!(inv.values, vec![vec![1., 0.], vec![0., 1.],]);
    }

    #[test]
    fn inverse_two_matrix() {
        let m = Matrix {
            values: vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 2.]],
        };

        let inv = m.inverse().unwrap();

        assert_eq!(
            inv.values,
            vec![vec![0.5, 0., 0.], vec![0., 0.5, 0.], vec![0., 0., 0.5]]
        );
    }

    #[test]
    fn complicated_inverse() {
        let m = Matrix {
            values: vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]],
        };

        let inv = m.inverse().unwrap();

        assert_eq!(
            inv.values,
            vec![
                vec![0.6494252873563219, 0.09770114942528735, -0.6551724137931034],
                vec![
                    -0.7816091954022988,
                    -0.12643678160919541,
                    0.9655172413793103
                ],
                vec![
                    0.14367816091954022,
                    0.07471264367816091,
                    -0.2068965517241379
                ]
            ]
        );
    }
}
