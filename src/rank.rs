use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    fn rank(&self) -> usize {
        let m = self.clone().reduced_row_echelon();
        m.values
            .iter()
            .filter(|row| row.iter().any(|&x| x != K::zero()))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank() {
        let mut m = Matrix {
            values: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };

        assert_eq!(m.rank(), 2);
    }

    #[test]
    fn test_rank_subject_1() {
        let m = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);

        assert_eq!(m.rank(), 3);
    }

    #[test]
    fn test_rank_subject_2() {
        let m = Matrix::from(&[&[1., 2., 0., 0.], &[2., 4., 0., 0.], &[-1., 2., 1., 1.]]);

        assert_eq!(m.rank(), 2);
    }

    #[test]
    fn test_rank_subject_3() {
        let m = Matrix::from(&[
            &[8., 5., -2.],
            &[4., 7., 20.],
            &[7., 6., 1.],
            &[21., 18., 7.],
        ]);

        assert_eq!(m.rank(), 3);
    }
}
