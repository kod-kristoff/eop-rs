use std::{fmt::Debug, ops::Sub};

pub trait Transformation<Args> {
    type Domain;

    fn call(&self, x: Self::Domain) -> Self::Domain;
}

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
pub trait Integer: Sized + Zero + One + Eq + Sub<Self, Output = Self> + Debug + Copy {}
impl Integer for u32 {}
pub fn power_unary<F, N, Args>(mut x: F::Domain, mut n: N, f: F) -> F::Domain
where
    F: Transformation<Args>,
    N: Integer,
{
    while n != N::zero() {
        dbg!(n);
        x = f.call(x);
        n = n - N::one();
    }
    x
}

impl<F, T> Transformation<(T,)> for F
where
    F: Fn(T) -> T,
{
    type Domain = T;
    fn call(&self, x: Self::Domain) -> Self::Domain {
        (self)(x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn sq(x: i32) -> i32 {
        x * x
    }
    mod power_unary {
        use super::*;
        #[test]
        fn with_fn() {
            assert_eq!(power_unary(3, 3u32, sq), 27);
        }
    }
}
