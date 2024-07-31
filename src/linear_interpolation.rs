use std::ops::{Add, Mul, Sub};

use crate::numeric::Numeric;


trait Lerp<V: Numeric> {
    fn lerp(u: V, v: V, t: f32) -> V;
}

impl<T> Lerp<T> for T
where
    T: Numeric + Copy + Add<Output = T> + Mul<f32, Output = T> + Sub<Output = T>,
{
    fn lerp(u: T, v: T, t: f32) -> T {
        u * (1.0 - t) + v * t
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, vector::Vector};

    use super::*;

    #[test]
    fn lerp_works_with_floats() {
        let u = 0.0;
        let v = 1.0;

        assert_eq!(f32::lerp(u, v, 0.5), 0.5);
        assert_eq!(f32::lerp(u, v, 0.), 0.);
        assert_eq!(f32::lerp(u, v, 1.0), 1.);
        assert_eq!(f32::lerp(21., 42., 0.3), 27.3);
    }

    #[test]
    fn lerp_works_with_vectors() {
        let v1 = Vector::from(&[2., 1.]);
        let v2 = Vector::from(&[4., 2.]);
        assert_eq!(Vector::lerp(v1, v2, 0.3), Vector::from(&[2.6, 1.3]));
    }

    #[test]
    fn lerp_works_with_matrices() {
        let m1 = Matrix::from(&[&[2., 1.], &[3., 4.]]);
        let m2 = Matrix::from(&[&[20., 10.], &[30., 40.]]);

        assert_eq!(
            Matrix::lerp(m1, m2, 0.5),
            Matrix::from(&[&[11., 5.5], &[16.5, 22.]])
        );
    }
}
