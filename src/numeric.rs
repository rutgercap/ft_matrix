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
    + std::ops::SubAssign
    + std::ops::DivAssign
    + Default
    + std::fmt::Debug
{
    fn abs(self) -> Self;
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
        + std::ops::SubAssign
        + std::ops::DivAssign
        + Default
        + std::fmt::Debug,
{
    fn abs(self) -> Self {
        if self < Self::zero() {
            -self
        } else {
            self
        }
    }
}
