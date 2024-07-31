use num::Float;

use crate::{numeric::Numeric, vector::Vector};

pub fn angle_cos<K: Numeric + Float>(u: &Vector::<K>, v: &Vector::<K>) -> K {
    let dot = u.dot(v);
    dot / (v.norm_2() * v.norm_2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_cos_works() {
        let v1 = Vector::from(&[1., 0.]);
        let v2 = Vector::from(&[0., 1.]);
        assert_eq!(angle_cos(&v1, &v2), 0.);
    }
}