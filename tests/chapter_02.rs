use std::ops::Mul;

use eop::math::power_unary;
use eop::transformations::distance;

fn sq<T>(x: T) -> T
where
    T: Mul<T, Output = T> + Copy,
{
    x * x
}
#[test]
fn test_power_unary() {
    for i in 2..5 {
        for j in 1..5 {
            let tmp = power_unary(i, j - 1, sq);

            assert_eq!(power_unary(i, j, sq), tmp * tmp);
        }
    }
}

#[test]
fn test_distance() {
    assert_eq!(distance(2, 65536, sq), 4);
}
