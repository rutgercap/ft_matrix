#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<K> {
    pub values: Vec<Vec<K>>,
}

impl<K: Clone> Matrix<K> {
    pub fn from(values: &[&[K]]) -> Self {
        let matrix_values: Vec<Vec<K>> = values.iter().map(|row| row.to_vec()).collect();
        Matrix { values: matrix_values }
    }
}

impl<K> Matrix<K>
where
    K: std::ops::Sub<Output = K>
        + std::ops::Add<Output = K>
        + std::ops::Mul<Output = K>
        + PartialEq
        + Copy,
{
    fn add(&mut self, v: &Matrix<K>) {
        if self.values.len() != v.values.len() {
            panic!("Matrices must have the same dimensions to add them");
        }
        self.values.iter_mut().zip(&v.values).for_each(|(x, y)| {
            if x.len() != y.len() {
                panic!("Matrices must have the same dimensions to add them");
            }
            x.iter_mut().zip(y).for_each(|(a, b)| *a = *a + *b)
        });
    }

    fn sub(&mut self, v: &Matrix<K>) {
        if self.values.len() != v.values.len() {
            panic!("Matrices must have the same dimensions to subtract them");
        }
        self.values.iter_mut().zip(&v.values).for_each(|(x, y)| {
            if x.len() != y.len() {
                panic!("Matrices must have the same dimensions to subtract them");
            }
            x.iter_mut().zip(y).for_each(|(a, b)| *a = *a - *b);
        });
    }

    fn scl(&mut self, a: K) {
        self.values
            .iter_mut()
            .for_each(|x| x.iter_mut().for_each(|y| *y = *y * a));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_properly_scales_a_matrix() {
        let mut v = Matrix {
            values: vec![vec![1, 2, 3], vec![2, 4, 6]],
        };
        v.scl(2);
        assert_eq!(v.values, vec![vec![2, 4, 6], vec![4, 8, 12]]);
    }

    #[test]
    fn add_properly_adds_a_matrix() {
        let mut v = Matrix {
            values: vec![vec![1, 2, 3], vec![2, 4, 6]],
        };
        let second = Matrix {
            values: vec![vec![1, 2, 3], vec![2, 4, 6]],
        };
        v.add(&second);
        assert_eq!(v.values, vec![vec![2, 4, 6], vec![4, 8, 12]]);
    }

    #[test]
    #[should_panic]
    fn add_panics_if_matrices_do_not_have_same_dimensions() {
        let mut v = Matrix {
            values: vec![vec![1, 2], vec![2, 4, 6]],
        };
        let second = Matrix {
            values: vec![vec![1, 2, 3], vec![2, 4, 6]],
        };
        v.add(&second);
    }

    #[test]
    fn sub_properly_subtracts_a_matrix() {
        let mut v = Matrix {
            values: vec![vec![10, 10, 10], vec![2, 4, 6]],
        };
        let second = Matrix {
            values: vec![vec![1, 2, 3], vec![2, 4, 6]],
        };
        v.sub(&second);
        assert_eq!(v.values, vec![vec![9, 8, 7], vec![0, 0, 0]]);
    }

    #[test]
    #[should_panic]
    fn sub_panics_if_matrices_do_not_have_same_dimensions() {
        let mut v = Matrix {
            values: vec![vec![1, 2], vec![2, 4, 6]],
        };
        let second = Matrix {
            values: vec![vec![1, 2, 3], vec![2, 4, 6]],
        };
        v.sub(&second);
    }
}
