use std::ops::Add;

use crate::concepts::{One, Zero};
use crate::core::UnaryPredicate;
use crate::transformations::{distance, Transformation};

pub fn collision_point<F, Fargs, P, Pargs>(x: F::Domain, f: F, p: P) -> F::Domain
where
    F: Transformation<Fargs>,
    P: UnaryPredicate<Pargs, Domain = F::Domain>,
    F::Domain: PartialEq + Copy,
{
    if !p.call(x) {
        return x;
    }
    let mut slow = x;
    let mut fast = f.call(x);
    while fast != slow {
        slow = f.call(slow);
        if !p.call(fast) {
            return fast;
        }
        fast = f.call(fast);
        if !p.call(fast) {
            return fast;
        }
        fast = f.call(fast);
    }
    fast
}

pub fn collision_point_nonterminating_orbit<F, Fargs>(x: F::Domain, f: F) -> F::Domain
where
    F: Transformation<Fargs>,
    F::Domain: PartialEq + Copy,
{
    let mut slow = x;
    let mut fast = f.call(x);
    while fast != slow {
        slow = f.call(slow);
        fast = f.call(fast);
        fast = f.call(fast);
    }
    fast
}

pub fn convergent_point<F, Fargs>(mut x0: F::Domain, mut x1: F::Domain, f: F) -> F::Domain
where
    F: Transformation<Fargs>,
    F::Domain: PartialEq,
{
    while x0 != x1 {
        x0 = f.call(x0);
        x1 = f.call(x1);
    }
    x0
}

pub fn connection_point<F, Fargs, P, Pargs>(x: F::Domain, f: F, p: P) -> F::Domain
where
    F: Transformation<Fargs> + Copy,
    P: UnaryPredicate<Pargs, Domain = F::Domain> + Copy,
    F::Domain: PartialEq + Copy,
{
    let y = collision_point(x, f, p);
    if !p.call(y) {
        y
    } else {
        convergent_point(x, f.call(y), f)
    }
}

pub fn connection_point_nonterminating_orbit<F, Fargs>(x: F::Domain, f: F) -> F::Domain
where
    F: Transformation<Fargs> + Copy,
    F::Domain: PartialEq + Copy,
{
    convergent_point(x, f.call(collision_point_nonterminating_orbit(x, f)), f)
}

pub fn terminating<F, Fargs, P, Pargs>(x: F::Domain, f: F, p: P) -> bool
where
    F: Transformation<Fargs>,
    P: UnaryPredicate<Pargs, Domain = F::Domain> + Copy,
    F::Domain: PartialEq + Copy,
{
    !p.call(collision_point(x, f, p))
}

pub fn circular<F, Fargs, P, Pargs>(x: F::Domain, f: F, p: P) -> bool
where
    F: Transformation<Fargs> + Copy,
    P: UnaryPredicate<Pargs, Domain = F::Domain> + Copy,
    F::Domain: PartialEq + Copy,
{
    let y = collision_point(x, f, p);
    p.call(y) && x == f.call(y)
}

pub fn circular_nonterminating_orbit<F, Fargs>(x: F::Domain, f: F) -> bool
where
    F: Transformation<Fargs> + Copy,
    F::Domain: PartialEq + Copy,
{
    x == f.call(collision_point_nonterminating_orbit(x, f))
}

pub fn orbit_structure<F, Fargs, P, Pargs>(
    x: F::Domain,
    f: F,
    p: P,
) -> (F::DistanceType, F::DistanceType, F::Domain)
where
    F: Transformation<Fargs> + Copy,
    P: UnaryPredicate<Pargs, Domain = F::Domain> + Copy,
    F::Domain: PartialEq + Copy,
    F::DistanceType: Zero + One + Add<F::DistanceType, Output = F::DistanceType>,
{
    let y = connection_point(x, f, p);
    let m = distance(x, y, f);
    let mut n = F::DistanceType::zero();
    if p.call(y) {
        n = distance(f.call(y), y, f);
    }
    (m, n, y)
}

pub fn orbit_structure_nonterminating_orbit<F, Fargs>(
    x: F::Domain,
    f: F,
) -> (F::DistanceType, F::DistanceType, F::Domain)
where
    F: Transformation<Fargs> + Copy,
    F::Domain: PartialEq + Copy,
    F::DistanceType: Zero + One + Add<F::DistanceType, Output = F::DistanceType>,
{
    let y = connection_point_nonterminating_orbit(x, f);
    (distance(x, y, f), distance(f.call(y), y, f), y)
}

#[cfg(test)]
mod tests {
    use super::*;
}
