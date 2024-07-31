use crate::{numeric::Numeric, vector::Vector};

pub fn linear_combination<K: Numeric>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    if u.len() != coefs.len() {
        panic!("Need same number of vectors and coefficients");
    }
    u.iter().zip(coefs.iter()).map(|(v, c)| {
        let mut copy = v.clone();
        copy.scl(*c);
        copy
    }).fold(Vector { values: vec![K::default(); u[0].values.len()] }, |acc, x| {
        let mut copy = acc.clone();
        copy.add(&x);
        copy
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_combination_works() {
        let v1 = Vector { values: vec![1, 2, 3] };
        let v2 = Vector { values: vec![4, 5, 6] };
        let v3 = Vector { values: vec![7, 8, 9] };

        let result = linear_combination(&[v1, v2, v3], &[1, 2, 3]);

        assert_eq!(result.values, vec![30, 36, 42]);
    }

    #[test]
    fn linear_combination_with_subject_example() {
        let e1 = Vector::from(&[1., 0., 0.]);
        let e2 = Vector::from(&[0., 1., 0.]);
        let e3 = Vector::from(&[0., 0., 1.]);

        let result = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);

        assert_eq!(result.values, vec![10., -2., 0.5]);
    }

    #[test]
    fn linear_combination_with_second_subject_example() {
        let v1 = Vector::from(&[1., 2., 3.]);
        let v2 = Vector::from(&[0., 10., -100.]);

        let result = linear_combination(&[v1, v2], &[10., -2.]);

        assert_eq!(result.values, vec![10., 0., 230.]);
    }

    #[test]
    #[should_panic]
    fn linear_combination_panics_if_not_enough_scalars() {
        let v1 = Vector::from(&[1., 2., 3.]);
        let v2 = Vector::from(&[0., 10., -100.]);

        linear_combination(&[v1, v2], &[10.]);
    }
}