use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    pub fn determinant(&self) -> K {
        let temp = self.clone().row_echelon();
        temp.values
            .iter()
            .enumerate()
            .fold(K::one(), |acc, (i, row)| acc * row[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_works() {
        let m = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
        assert_eq!(m.determinant(), 1.);
    }

    #[test]
    fn determinant_works_2() {
        let m = Matrix::from(&[&[1., -1.], &[-1., 1.]]);
        assert_eq!(m.determinant(), 0.);
    }

    #[test]
    fn determinant_works_3() {
        let m = Matrix::from(&[&[2., 0., 0.], &[0., 2., 0.], &[0., 0., 2.]]);
        assert_eq!(m.determinant(), 8.);
    }
}
