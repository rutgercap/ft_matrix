use num::{One, Zero};
pub trait Numeric:
    Copy
    + PartialOrd
    + PartialEq
    + Zero
    + One
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::ops::AddAssign
    + std::fmt::Debug
{
    fn magnitude(self) -> f64;
}

impl<T> Numeric for T
where
    T: Copy
        + PartialOrd
        + PartialEq
        + Zero
        + One
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
        + std::ops::Neg<Output = Self>
        + std::ops::AddAssign
        + std::ops::DivAssign
        + std::fmt::Debug
        + Into<f64>,
{
    fn magnitude(self) -> f64 {
        if self < Self::zero() {
            (-self).into()
        } else {
            self.into()
        }
    }
}
