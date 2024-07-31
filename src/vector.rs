
#[derive(Clone, Debug, PartialEq)]
pub struct Vector<K: std::ops::Mul<Output= K>> {
    pub values: Vec<K>,
}

impl<K> Vector<K>
where
    K: std::ops::Sub<Output = K>+ std::ops::Add<Output = K> + std::ops::Mul<Output = K> + PartialEq + Copy,
{
    pub fn add(&mut self, v: &Vector<K>) {
        if self.values.len() != v.values.len() {
            panic!("Vectors must be the same length");
        }
        self.values.iter_mut().zip(v.values.iter()).for_each(|(x, y)| *x = *x + *y);
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        if self.values.len() != v.values.len() {
            panic!("Vectors must be the same length");
        }
        self.values.iter_mut().zip(v.values.iter()).for_each(|(x, y)| *x = *x - *y);
    }

    pub fn scl(&mut self, a: K) {
        self.values.iter_mut().for_each(|x| *x = *x * a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_properly_scales_a_vector() {
        let mut v = Vector { values: vec![1, 2, 3] };
        v.scl(2);
        assert_eq!(v.values, vec![2, 4, 6]);
    }

    #[test]
    fn add_properly_adds_two_vectors() {
        let mut v = Vector { values: vec![1, 2, 3] };
        let v2 = Vector { values: vec![1, 2, 3] };
        v.add(&v2);
        assert_eq!(v.values, vec![2, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn add_panics_if_vectors_are_not_same_length() {
        let mut v = Vector { values: vec![1, 2, 3] };
        let v2 = Vector { values: vec![1, 2] };
        v.add(&v2);
    }

    #[test]
    fn sub_properly_subtracts_two_vectors() {
        let mut v = Vector { values: vec![3, 6, 9] };
        let v2 = Vector { values: vec![1, 2, 3] };
        v.sub(&v2);
        assert_eq!(v.values, vec![2, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn sub_panics_if_vectors_are_not_same_length() {
        let mut v = Vector { values: vec![1, 2, 3] };
        let v2 = Vector { values: vec![1, 2] };
        v.sub(&v2);
    }
}