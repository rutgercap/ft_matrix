pub trait Numeric:
    std::ops::Sub<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self> 
    + Default
    + PartialEq
    + Copy
{}

impl<T> Numeric for T where
    T: std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = Self>
        + Default
        + PartialEq
        + Copy
{}