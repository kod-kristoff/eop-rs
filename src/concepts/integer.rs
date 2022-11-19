use std::{fmt::Debug, ops::Sub};

pub trait Zero: Sized {
    fn zero() -> Self;
}
impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}
pub trait One: Sized {
    fn one() -> Self;
}
impl One for u32 {
    fn one() -> Self {
        1
    }
}
pub trait Integer: Sized + Zero + One + Eq + Sub<Self, Output = Self> + Copy {}
impl Integer for u32 {}
