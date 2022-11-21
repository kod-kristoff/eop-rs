use std::ops::Add;

use crate::{
    concepts::{One, Zero},
    transformations::Transformation,
};

/// Precondition: y is reachable from x under f
pub fn distance<F, Args>(mut x: F::Domain, y: F::Domain, f: F) -> F::DistanceType
where
    F: Transformation<Args>,
    F::Domain: PartialEq,
    F::DistanceType: Zero + One + Add<F::DistanceType, Output = F::DistanceType>,
{
    let mut n = F::DistanceType::zero();
    while x != y {
        x = f.call(x);
        n = n + F::DistanceType::one();
    }
    n
}
