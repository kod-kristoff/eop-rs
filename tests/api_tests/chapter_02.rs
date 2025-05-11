use std::ops::Mul;

use eop::concepts::{Integer, Zero};
use eop::core::UnaryPredicate;
use eop::math::power_unary;
use eop::transformations::orbits::{
    circular, circular_nonterminating_orbit, collision_point, connection_point,
    connection_point_nonterminating_orbit, orbit_structure, orbit_structure_nonterminating_orbit,
};
use eop::transformations::{Transformation, distance, orbits::terminating};

fn sq<T>(x: T) -> T
where
    T: Mul<T, Output = T> + Copy,
{
    x * x
}
#[test]
fn test_power_unary() {
    for i in 2u64..5u64 {
        for j in 1u64..5u64 {
            let tmp = power_unary(i, j - 1, sq);

            assert_eq!(power_unary(i, j, sq), tmp * tmp);
        }
    }
}

#[test]
fn test_distance() {
    assert_eq!(distance(2, 65536, sq), 4);
}

mod function_objects {
    use super::*;
    // struct GenOrbitPredicate<I, N> {
    #[derive(Debug, Copy, Clone)]
    struct GenOrbitPredicate {
        x_0: i32,
        h: i32,
        c: i32,
    }

    // impl<I,N> GenOrbitPredicate<I,N> {
    //     fn call(&self, x: I) -> bool {
    impl UnaryPredicate<()> for GenOrbitPredicate {
        type Domain = i32;

        fn call(&self, x: Self::Domain) -> bool {
            self.x_0 <= x && x < self.x_0 + self.h + self.c
        }
    }

    // struct GenOrbit<I,N> {
    //     p: GenOrbitPredicate<I,N>,
    #[derive(Debug, Copy, Clone)]
    struct GenOrbit {
        p: GenOrbitPredicate,
    }

    // impl<I,N> GenOrbit<I,N> {
    //     pub fn new(x_0: I, h: N, c: N) -> Self {
    impl GenOrbit {
        pub fn new(x_0: i32, h: i32, c: i32) -> Self {
            Self {
                p: GenOrbitPredicate { x_0, h, c },
            }
        }
    }
    // impl<I,N> Transformation<(I,N)> for GenOrbit<I,N> {
    //     type Domain = I;
    impl Transformation<()> for GenOrbit {
        type Domain = i32;
        type DistanceType = i32;

        fn call(&self, mut x: Self::Domain) -> Self::Domain {
            if !self.p.call(x) {
                println!("validation error: x={}, p={:?}", x, self.p);
            }
            assert!(self.p.call(x));
            x = x + 1;
            if x == self.p.x_0 + self.p.h + self.p.c {
                x = self.p.x_0 + self.p.h;
            }
            x
        }
    }

    #[test]
    fn test_circular_orbit() {
        algorithms_orbit(0, 0, 5);
    }

    #[test]
    fn test_rho_shaped_orbit_1() {
        algorithms_orbit(0, 2, 11);
    }

    #[test]
    fn test_rho_shaped_orbit_2() {
        algorithms_orbit(7, 97, 17);
    }

    #[test]
    fn test_rho_shaped_orbit_3() {
        algorithms_orbit(0, 4, 2);
    }

    #[test]
    fn test_terminating_orbit() {
        algorithms_orbit(0, 101, 0);
    }

    fn algorithms_orbit(x: i32, h: i32, c: i32) {
        let f = GenOrbit::new(x, h, c);

        assert_eq!(c.is_zero(), terminating(x, f, f.p));
        if h.is_zero() && !c.is_zero() {
            assert!(circular(x, f, f.p));
            assert!(circular_nonterminating_orbit(x, f));
        } else if !h.is_zero() {
            assert!(!circular(x, f, f.p));
            if !c.is_zero() {
                assert!(!circular_nonterminating_orbit(x, f));
            }
        }
        let y = connection_point(x, f, f.p);
        assert_eq!(power_unary(x, h, f), y);
        if !c.is_zero() {
            assert_eq!(y, connection_point_nonterminating_orbit(x, f));
        }
        let (m0, m1, m2) = orbit_structure(x, f, f.p);
        if c.is_zero() {
            // terminating
            assert_eq!(m0, h);
            assert!(m1.is_zero());
            assert_eq!(m2, collision_point(x, f, f.p));
        } else if h.is_zero() {
            // circular
            assert!(m0.is_zero());
            assert_eq!(m1, c.predecessor());
            assert_eq!(m2, x);
        } else {
            // rho-shaped
            assert_eq!(m0, h);
            assert_eq!(m1, c.predecessor());
            assert_eq!(m2, y);
        }
        if !c.is_zero() {
            let (m0, m1, m2) = orbit_structure_nonterminating_orbit(x, f);
            if h.is_zero() {
                // circular
                assert!(m0.is_zero());
                assert_eq!(m1, c.predecessor());
                assert_eq!(m2, x);
            } else {
                // rho-shaped
                assert_eq!(m0, h);
                assert_eq!(m1, c.predecessor());
                assert_eq!(m2, y);
            }
        }
        // let y = connection_point(x, f, f.p);
        // assert_eq!(power_unary(x, h, f), y);
    }
}

mod functions {
    use super::*;

    #[test]
    fn test_circular_orbit() {
        let x = 0;
        let h = 0;
        assert!(!terminating(x, f_circ, p_circ));
        assert!(circular(x, f_circ, p_circ));
        assert!(circular_nonterminating_orbit(x, f_circ));
        let y = connection_point(x, f_circ, p_circ);
        assert_eq!(power_unary(x, h, f_circ), y);
        assert_eq!(y, connection_point_nonterminating_orbit(x, f_circ));
    }

    fn f_circ(x: i32) -> i32 {
        if x == 4 { 0 } else { x + 1 }
    }
    fn p_circ(x: i32) -> bool {
        0 <= x && x < 5
    }
}
mod closures {
    use super::*;

    #[test]
    fn test_circular_orbit() {
        assert!(!terminating(
            0,
            |x| if x == 4 { 0 } else { x + 1 },
            |x| 0 <= x && x < 5
        ));
    }
}
