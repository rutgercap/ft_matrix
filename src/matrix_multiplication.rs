use crate::{matrix::Matrix, numeric::Numeric, vector::Vector};

impl<K: Numeric> Matrix<K> {
    fn mul_vec(&self, vec: Vector<K>) -> Vector<K> {
        if self.values[0].len() != vec.values.len() {
            panic!("Matrix and vector dimensions don't match");
        }
        let result: Vec<K> = self
            .values
            .iter()
            .map(|row| {
                row.iter()
                    .zip(vec.values.iter())
                    .fold(K::zero(), |acc, (&a, &b)| acc + a * b)
            })
            .collect();
        Vector::<K>::from(&result)
    }

    fn mul_mat(&self, mat: Matrix<K>) -> Matrix<K> {
        if self.values[0].len() != mat.values.len() {
            panic!("Matrix dimensions don't match");
        }
        let mut result = Matrix {
            values: vec![vec![K::zero(); mat.values[0].len()]; self.values.len()],
        };
        for i in 0..self.values.len() {
            for j in 0..mat.values[0].len() {
                for k in 0..self.values[0].len() {
                    result.values[i][j] += self.values[i][k] * mat.values[k][j];
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn multiplying_matrix_with_vec_of_incorrect_length_panics() {
        let m = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);

        let v = Vector::from(&[1., 2.]);

        m.mul_vec(v);
    }

    #[test]
    fn matrix_vector_multiplication_works() {
        let m = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);

        let v = Vector::from(&[1., 2., 3.]);

        let result = m.mul_vec(v);

        assert_eq!(result, Vector::from(&[14., 32., 50.]));
    }

    #[test]
    fn matrix_vector_multiplication_works_1() {
        let m = Matrix::from(&[&[1., 0.], &[0., 1.]]);

        let v = Vector::from(&[4., 2.]);

        let result = m.mul_vec(v);

        assert_eq!(result, Vector::from(&[4.0, 2.0]));
    }

    #[test]
    fn matrix_vector_multiplication_works_2() {
        let m = Matrix::from(&[&[2., 0.], &[0., 2.]]);

        let v = Vector::from(&[4., 2.]);

        let result = m.mul_vec(v);

        assert_eq!(result, Vector::from(&[8.0, 4.0]));
    }

    #[test]
    fn matrix_vector_multiplication_works_3() {
        let m = Matrix::from(&[&[2., -2.], &[-2., 2.]]);

        let v = Vector::from(&[4., 2.]);

        let result = m.mul_vec(v);

        assert_eq!(result, Vector::from(&[4.0, -4.0]));
    }

    #[test]
    #[should_panic]
    fn multiplying_matrix_with_matrix_of_incorrect_length_panics() {
        let m1 = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);

        let m2 = Matrix::from(&[&[1., 2.], &[3., 4.]]);

        m1.mul_mat(m2);
    }

    #[test]
    fn matrix_multiplication_works_1() {
        let m1 = Matrix::from(&[&[1., 2.], &[3., 4.]]);

        let m2 = Matrix::from(&[&[1., 0.], &[0., 1.]]);

        let result = m1.mul_mat(m2);

        assert_eq!(result, Matrix::from(&[&[1., 2.], &[3., 4.],]));
    }

    #[test]
    fn matrix_multiplication_works_2() {
        let m1 = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        let m2 = Matrix::from(&[&[1., 0.], &[0., 1.]]);

        let result = m1.mul_mat(m2);

        assert_eq!(result, Matrix::from(&[&[1., 0.], &[0., 1.],]));
    }

    #[test]
    fn matrix_multiplication_works_3() {
        let m1 = Matrix::from(&[&[3., -5.], &[6., 8.]]);
        let m2 = Matrix::from(&[&[2., 1.], &[4., 2.]]);

        let result = m1.mul_mat(m2);

        assert_eq!(result, Matrix::from(&[&[-14., -7.], &[44., 22.],]));
    }
}
