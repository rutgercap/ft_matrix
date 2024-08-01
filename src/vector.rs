use crate::numeric::Numeric;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<K: Numeric> {
    pub values: Vec<K>,
}

impl<K: Numeric> Vector<K> {
    pub fn from(values: &[K]) -> Self {
        Vector {
            values: Vec::from(values),
        }
    }
}

impl Vector<f32> {
    pub fn lerp(u: Vector<f32>, v: Vector<f32>, t: f32) -> Vector<f32> {
        let mut result = u.clone();
        result.scl(1.0 - t);
        let mut v = v.clone();
        v.scl(t);
        result.add(&v);
        result
    }
}

impl<K> Vector<K>
where
    K: Numeric,
{
    pub fn add(&mut self, v: &Vector<K>) {
        if self.values.len() != v.values.len() {
            panic!("Vectors must be the same length");
        }
        self.values
            .iter_mut()
            .zip(v.values.iter())
            .for_each(|(x, y)| *x = *x + *y);
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        if self.values.len() != v.values.len() {
            panic!("Vectors must be the same length");
        }
        self.values
            .iter_mut()
            .zip(v.values.iter())
            .for_each(|(x, y)| *x = *x - *y);
    }

    pub fn scl(&mut self, a: K) {
        self.values.iter_mut().for_each(|x| *x = *x * a);
    }

    pub fn dot(&self, v: &Vector<K>) -> K {
        if self.values.len() != v.values.len() {
            panic!("Vectors must be the same length");
        }
        self.values
            .iter()
            .zip(v.values.iter())
            .fold(K::zero(), |acc, (x, y)| acc + *x * *y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_properly_scales_a_vector() {
        let mut v = Vector {
            values: vec![1, 2, 3],
        };
        v.scl(2);
        assert_eq!(v.values, vec![2, 4, 6]);
    }

    #[test]
    fn add_properly_adds_two_vectors() {
        let mut v = Vector {
            values: vec![1, 2, 3],
        };
        let v2 = Vector {
            values: vec![1, 2, 3],
        };
        v.add(&v2);
        assert_eq!(v.values, vec![2, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn add_panics_if_vectors_are_not_same_length() {
        let mut v = Vector {
            values: vec![1, 2, 3],
        };
        let v2 = Vector { values: vec![1, 2] };
        v.add(&v2);
    }

    #[test]
    fn sub_properly_subtracts_two_vectors() {
        let mut v = Vector {
            values: vec![3, 6, 9],
        };
        let v2 = Vector {
            values: vec![1, 2, 3],
        };
        v.sub(&v2);
        assert_eq!(v.values, vec![2, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn sub_panics_if_vectors_are_not_same_length() {
        let mut v = Vector {
            values: vec![1, 2, 3],
        };
        let v2 = Vector { values: vec![1, 2] };
        v.sub(&v2);
    }

    #[test]
    #[should_panic]
    fn dot_panics_if_vectors_are_not_same_length() {
        let v = Vector {
            values: vec![1, 2, 3],
        };
        let v2 = Vector { values: vec![1, 2] };
        v.dot(&v2);
    }

    #[test]
    fn dot_product_returns_dot_product() {
        let v = Vector {
            values: vec![0., 0.],
        };
        let v2 = Vector {
            values: vec![1., 1.],
        };

        assert_eq!(v.dot(&v2), 0.);
    }

    #[test]
    fn dot_product_returns_dot_product_2() {
        let v = Vector {
            values: vec![1., 1.],
        };
        let v2 = Vector {
            values: vec![1., 1.],
        };

        assert_eq!(v.dot(&v2), 2.);
    }

    #[test]
    fn dot_product_returns_dot_product_3() {
        let v = Vector {
            values: vec![-1., 6.],
        };
        let v2 = Vector {
            values: vec![3., 2.],
        };

        assert_eq!(v.dot(&v2), 9.);
    }
}
