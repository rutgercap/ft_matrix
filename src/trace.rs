use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    fn trace(&self) -> K {
        if !self.is_square() {
            panic!("Matrix must be square to calculate trace");
        }
        self.values
            .iter()
            .enumerate()
            .fold(K::zero(), |acc, (i, row)| acc + row[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_works() {
        let m = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);
        assert_eq!(m.trace(), 15.);
    }

    #[test]
    fn trace_works_2() {
        let m = Matrix::from(&[&[2., -5., 0.], &[4., 3., 7.], &[-2., 3., 4.]]);
        assert_eq!(m.trace(), 9.);
    }

    #[test]
    #[should_panic]
    fn traces_panicks_if_matrix_is_not_a_square() {
        let m = Matrix::from(&[&[1., 2.], &[4., 5.], &[7., 8.]]);
        m.trace();
    }
}
