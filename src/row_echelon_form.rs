use crate::{matrix::Matrix, numeric::Numeric};

impl<K: Numeric> Matrix<K> {
    pub fn divide_row(&mut self, row: usize, divisor: K) {
        if divisor == K::zero() {
            panic!("Division by zero in row operation");
        }
        self.values[row].iter_mut().for_each(|x| *x = *x / divisor);
    }

    pub fn scale_row(&mut self, row: usize, factor: K) {
        if factor == K::zero() {
            panic!("Division by zero in row operation");
        }
        self.values[row].iter_mut().for_each(|x| *x = *x * factor);
    }

    pub fn subtract_multiple_of_row(&mut self, source_row: usize, target_row: usize, factor: K) {
        let source_row_clone = self.values[source_row].clone();
        for (i, x) in self.values[target_row].iter_mut().enumerate() {
            *x = *x - factor * source_row_clone[i];
        }
    }

    pub fn row_echelon(mut self) -> Self {
        let rows = self.values.len();
        let cols = if rows > 0 { self.values[0].len() } else { 0 };
        let mut pivot_row = 0;

        for i in 0..rows {
            let mut j = 0;
            while self.values[pivot_row][j] == K::zero() {
                j += 1;
                if j == cols {
                    break;
                }
            }
            if j == cols {
                pivot_row += 1;
                continue;
            }
            if pivot_row != i {
                self.values.swap(pivot_row, i);
            }
            for k in i + 1..rows {
                let leading_value = self.values[k][j];
                self.subtract_multiple_of_row(i, k, leading_value);
            }
            pivot_row += 1;
        }
        self
    }

    pub fn reduced_row_echelon(mut self) -> Self {
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
                        return self;
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
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_echelon_works() {
        let m = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]).row_echelon();
        let expected = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn reduced_row_echelon_again() {
        let mut m = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);
        m = m.reduced_row_echelon();
        let expected = Matrix::from(&[&[1., 0., -1.], &[0., 1., 2.], &[0., 0., 0.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn reduced_row_echelon_and_row_echelon_give_different_results() {
        let mut rr = Matrix::from(&[&[2., 0., 0.], &[0., 2., 0.], &[0., 0., 2.]]);
        let r = rr.clone().row_echelon();
        rr = rr.reduced_row_echelon();
        let expected = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
        assert_eq!(rr, expected);
        let expected = Matrix::from(&[&[2., 0., 0.], &[0., 2., 0.], &[0., 0., 2.]]);
        assert_eq!(r, expected);
    }

    #[test]
    fn row_echelon_works_2() {
        let mut m = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        m = m.reduced_row_echelon();
        let expected = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn row_echelon_works_3() {
        let m = Matrix::from(&[&[1., 2.], &[2., 4.]]).row_echelon();
        let expected = Matrix::from(&[&[1., 2.], &[0., 0.]]);
        assert_eq!(m, expected);
    }

    #[test]
    fn row_echelon_works_4() {
        let m = Matrix::from(&[
            &[8., 5., -2., 4., 28.],
            &[4., 2.5, 20., 4., -4.],
            &[8., 5., 1., 4., 17.],
        ])
        .reduced_row_echelon();
        let expected = Matrix::from(&[
            &[1.0, 0.625, 0.0, 0.0, -12.166666666666668],
            &[0.0, 0.0, 1.0, 0.0, -3.666666666666667],
            &[0.0, 0.0, 0.0, 1.0, 29.500000000000004],
        ]);
        assert_eq!(m, expected);
    }
}
