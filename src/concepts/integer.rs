use std::{fmt::Debug, ops::Sub};

pub trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
impl Zero for u32 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}
impl Zero for usize {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}
impl Zero for i32 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
}
impl Zero for u64 {
    fn zero() -> Self {
        0
    }
    fn is_zero(&self) -> bool {
        *self == 0
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
impl One for usize {
    fn one() -> Self {
        1
    }
}
impl One for i32 {
    fn one() -> Self {
        1
    }
}
impl One for u64 {
    fn one() -> Self {
        1
    }
}
pub trait Integer: Sized + Zero + One + Eq + Sub<Self, Output = Self> + Copy {
    fn predecessor(&self) -> Self;
    fn half_nonnegative(&self) -> Self;
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}
impl Integer for u32 {
    fn predecessor(&self) -> Self {
        self - 1
    }
    fn half_nonnegative(&self) -> Self {
        self >> 1
    }
    fn is_even(&self) -> bool {
        self & 1 == 0
    }
    fn is_odd(&self) -> bool {
        self & 1 != 0
    }
}
impl Integer for i32 {
    fn predecessor(&self) -> Self {
        self - 1
    }
    fn half_nonnegative(&self) -> Self {
        self >> 1
    }
    fn is_even(&self) -> bool {
        self & 1 == 0
    }
    fn is_odd(&self) -> bool {
        self & 1 != 0
    }
}
impl Integer for u64 {
    fn predecessor(&self) -> Self {
        self - 1
    }
    fn half_nonnegative(&self) -> Self {
        self >> 1
    }
    fn is_even(&self) -> bool {
        self & 1 == 0
    }
    fn is_odd(&self) -> bool {
        self & 1 != 0
    }
}
