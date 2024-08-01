use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    pub fn divide_row(&mut self, row: usize, divisor: K) {
        if divisor == K::zero() {
            panic!("Division by zero in row operation");
        }
        self.values[row].iter_mut().for_each(|x| *x = *x / divisor);
    }

    fn subtract_multiple_of_row(&mut self, source_row: usize, target_row: usize, factor: K) {
        let source_row_clone = self.values[source_row].clone();
        for (i, x) in self.values[target_row].iter_mut().enumerate() {
            *x = *x - factor * source_row_clone[i];
        }
    }

    pub fn row_echelon(&mut self) {
        let mut pivot_row = 0;
        for col in 0..self.values[0].len() {
            let mut pivot_found = false;
            for row in pivot_row..self.values.len() {
                if self.values[row][col] != K::zero() {
                    self.values.swap(row, pivot_row);
                    pivot_found = true;
                    break;
                }
            }
            if pivot_found {
                let pivot_element = self.values[pivot_row][col];
                self.divide_row(pivot_row, pivot_element);
                for row in pivot_row + 1..self.values.len() {
                    let factor = self.values[row][col];
                    self.subtract_multiple_of_row(pivot_row, row, factor);
                }
                pivot_row += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_echelon_works() {
        let mut m = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
        m.row_echelon();
        let expected = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn row_echelon_works_2() {
        let mut m = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        m.row_echelon();
        let expected = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn row_echelon_works_3() {
        let mut m = Matrix::from(&[&[1., 2.], &[2., 4.]]);
        m.row_echelon();
        let expected = Matrix::from(&[&[1., 2.], &[0., 0.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn row_echelon_works_4() {
        let mut m = Matrix::from(&[
            &[8., 5., -2., 4., 28.],
            &[4., 2.5, 20., 4., -4.],
            &[8., 5., 1., 4., 17.],
        ]);
        m.row_echelon();
        let expected = Matrix::from(&[
            &[1.0, 0.625, 0.0, 0.0, -12.1666667],
            &[0.0, 0.0, 1.0, 0.0, -3.6666667],
            &[0.0, 0.0, 0.0, 1.0, 29.5],
        ]);
        assert_eq!(m, expected);
    }
}
