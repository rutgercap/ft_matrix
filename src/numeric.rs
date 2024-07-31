use num::Zero;

pub trait Numeric:
    std::ops::Sub<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + Default
    + PartialEq
    + Copy
    + Zero
{
}

impl<T> Numeric for T where
    T: std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = Self>
        + Default
        + PartialEq
        + Zero
        + Copy
{
}
