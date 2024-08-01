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
        let rows = self.values.len();
        let cols = if rows > 0 { self.values[0].len() } else { 0 };

        let mut lead = 0;
        for r in 0..rows {
            if lead >= cols {
                break;
            }
            let mut i = r;
            while self.values[i][lead] == K::zero() {
                i += 1;
                if i == rows {
                    i = r;
                    lead += 1;
                    if lead == cols {
                        return;
                    }
                }
            }
            self.values.swap(i, r);

            let leading_value = self.values[r][lead];
            self.divide_row(r, leading_value);
            for i in 0..rows {
                if i != r {
                    let leading_value = self.values[i][lead];
                    self.subtract_multiple_of_row(r, i, leading_value);
                }
            }
            lead += 1;
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
            &[1.0, 0.625, 0.0, 0.0, -12.166666666666668],
            &[0.0, 0.0, 1.0, 0.0, -3.666666666666667],
            &[0.0, 0.0, 0.0, 1.0, 29.500000000000004],
        ]);
        assert_eq!(m, expected);
    }
}
