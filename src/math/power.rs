use crate::concepts::Integer;
use crate::transformations::Transformation;

pub fn power_unary<F, N, Args>(mut x: F::Domain, mut n: N, f: F) -> F::Domain
where
    F: Transformation<Args>,
    N: Integer,
{
    while n != N::zero() {
        x = f.call(x);
        n = n - N::one();
    }
    x
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
            assert_eq!(power_unary(3, 2u32, sq), 81);
        }
    }
}
