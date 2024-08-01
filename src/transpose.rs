use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    fn transpose(&mut self) -> Matrix<K> {
        let mut result = Matrix {
            values: vec![vec![K::zero(); self.values.len()]; self.values[0].len()],
        };
        for i in 0..self.values.len() {
            for j in 0..self.values[0].len() {
                result.values[j][i] = self.values[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_works() {
        let mut m = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);
        let expected = Matrix::from(&[&[1., 4., 7.], &[2., 5., 8.], &[3., 6., 9.]]);
        assert_eq!(m.transpose(), expected);
    }
}
