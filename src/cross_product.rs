use crate::{numeric::Numeric, vector::Vector};

fn cross_product<K: Numeric>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    if u.values.len() != 3 || v.values.len() != 3 {
        panic!("Cross product is only defined for vectors of length 3");
    }
    let mut result = Vector::from(&[K::zero(); 3]);
    result.values[0] = u.values[1] * v.values[2] - u.values[2] * v.values[1];
    result.values[1] = u.values[2] * v.values[0] - u.values[0] * v.values[2];
    result.values[2] = u.values[0] * v.values[1] - u.values[1] * v.values[0];
    result
}

impl<K: Numeric> Vector<K> {
    pub fn cross_product(&self, other: &Vector<K>) -> Vector<K> {
        cross_product(self, other)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn cross_product_panics_if_vectors_dont_have_length_3() {
        let v1 = Vector::from(&[1., 2.]);
        let v2 = Vector::from(&[3., 4., 5.]);
        cross_product(&v1, &v2);
    }

    #[test]
    fn cross_product_works() {
        let v1 = Vector::from(&[0., 0., 1.]);
        let v2 = Vector::from(&[1., 0., 0.]);
        let result = cross_product(&v1, &v2);
        assert_eq!(result, Vector::from(&[0., 1., 0.]));
    }

    #[test]
    fn cross_product_works_2() {
        let v1 = Vector::from(&[1., 2., 3.]);
        let v2 = Vector::from(&[4., 5., 6.]);
        let result = cross_product(&v1, &v2);
        assert_eq!(result, Vector::from(&[-3., 6., -3.]));
    }

    #[test]
    fn cross_product_works_3() {
        let v1 = Vector::from(&[4., 2., -3.]);
        let v2 = Vector::from(&[-2., -5., 16.]);
        let result = cross_product(&v1, &v2);
        assert_eq!(result, Vector::from(&[17., -58., -16.]));
    }
}
