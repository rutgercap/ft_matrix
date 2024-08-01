use num::Float;

use crate::{numeric::Numeric, vector::Vector};

impl<V: Numeric> Vector<V> {
    pub fn norm_1(&self) -> f64 {
        self.values.iter().fold(0., |acc, x| acc + x.magnitude())
    }

    pub fn norm_inf(&self) -> f64 {
        self.values.iter().fold(0., |acc, x| {
            if x.magnitude() > acc {
                x.magnitude()
            } else {
                acc
            }
        })
    }
}

impl<V: Numeric + Float> Vector<V> {
    pub fn norm_2(&self) -> V {
        self.values
            .iter()
            .fold(V::zero(), |acc, &x| acc + x * x)
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_1_works() {
        let v = Vector::from(&[0., 0., 0.]);
        assert_eq!(v.norm_2(), 0.);
        assert_eq!(v.norm_1(), 0.);
        assert_eq!(v.norm_inf(), 0.);
    }

    #[test]
    fn norm_2_works() {
        let v = Vector::from(&[1., 2., 3.]);
        assert_eq!(v.norm_2(), 3.7416573867739413);
        assert_eq!(v.norm_1(), 6.0);
        assert_eq!(v.norm_inf(), 3.0);
    }

    #[test]
    fn norm_3_works() {
        let v = Vector::from(&[-1., -2.]);
        assert_eq!(v.norm_2(), 2.23606797749979);
        assert_eq!(v.norm_1(), 3.0);
        assert_eq!(v.norm_inf(), 2.0);
    }
}
