use num::Zero;
pub trait Numeric:
    Copy
    + PartialOrd
    + PartialEq
    + Zero
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::ops::AddAssign
    + Default
{
    fn abs(self) -> Self;
}

impl<T> Numeric for T
where
    T: Copy
        + PartialOrd
        + PartialEq
        + Zero
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
        + std::ops::Neg<Output = Self>
        + std::ops::AddAssign
        + Default,
{
    fn abs(self) -> Self {
        if self < Self::zero() {
            -self
        } else {
            self
        }
    }
}
