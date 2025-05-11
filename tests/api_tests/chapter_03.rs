use std::ops::{Add, Mul, Sub};

use eop::math::power_binary::{power_0, power_left_associated};

fn plus<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

fn minus<T>(x: T, y: T) -> T
where
    T: Sub<Output = T>,
{
    x - y
}

fn multiply<T>(x: T, y: T) -> T
where
    T: Mul<Output = T>,
{
    x * y
}

fn concat(x: String, y: String) -> String {
    format!("{x}{y}")
}

fn algorithm_power(pow: fn(i32, i32, fn(i32, i32) -> i32) -> i32) {
    assert_eq!(pow(1, 1, multiply), 1);
    assert_eq!(pow(10, 1, multiply), 10);
    assert_eq!(pow(1, 10, multiply), 1);
    assert_eq!(pow(2, 2, multiply), 4);
    assert_eq!(pow(2, 10, multiply), 1024);
    assert_eq!(pow(10, 2, multiply), 100);
}

mod power_left_associated {
    use super::*;

    #[test]
    fn returns_same_when_n_is_1() {
        let a = 5;
        assert_eq!(power_left_associated(a, 1, plus), a);
    }

    #[test]
    fn works_with_concat() {
        let a = "ode".to_string();
        assert_eq!(&power_left_associated(a, 3, concat), "odeodeode");
    }
    #[test]
    fn works() {
        assert_eq!(power_left_associated(-2, 3, minus), 2);
        assert_eq!(power_left_associated(-2, 4, minus), 4);
        algorithm_power(power_left_associated);
    }
}

mod power_right_associated {
    use eop::math::power_binary::power_right_associated;

    use super::*;

    #[test]
    fn returns_same_when_n_is_1() {
        let a = 5;
        assert_eq!(power_right_associated(a, 1, plus), a);
    }

    #[test]
    fn works_with_concat() {
        let a = "ode".to_string();
        assert_eq!(&power_right_associated(a, 3, concat), "odeodeode");
    }
    #[test]
    fn works_with_multiply() {
        let a = 5;
        assert_eq!(power_right_associated(a, 5, multiply), 3125);
    }

    #[test]
    fn works() {
        assert_eq!(power_right_associated(-2, 3, minus), -2);
        assert_eq!(power_right_associated(-2, 4, minus), 0);
        algorithm_power(power_right_associated);
    }
}

#[test]
fn power0_works() {
    algorithm_power(power_0);
}
