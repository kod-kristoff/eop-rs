use std::ops::{Add, Mul, Sub};

use eop::math::power_binary::{
    power, power_0, power_1, power_accumulate, power_accumulate_0, power_accumulate_1,
    power_accumulate_2, power_accumulate_3, power_accumulate_4, power_accumulate_positive,
    power_left_associated,
};

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

fn algorithm_power_accumulate_positive(pow: fn(i32, i32, i32, fn(i32, i32) -> i32) -> i32) {
    assert_eq!(pow(99, 1, 1, multiply), 99 * 1);
    assert_eq!(pow(99, 10, 1, multiply), 99 * 10);
    assert_eq!(pow(99, 1, 10, multiply), 99 * 1);
    assert_eq!(pow(99, 2, 2, multiply), 99 * 4);
    assert_eq!(pow(99, 2, 10, multiply), 99 * 1024);
    assert_eq!(pow(99, 10, 2, multiply), 99 * 100);
}

fn algorithm_power_accumulate(pow: fn(i32, i32, i32, fn(i32, i32) -> i32) -> i32) {
    algorithm_power_accumulate_positive(pow);
    assert_eq!(pow(99, 1, 0, multiply), 99);
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
fn power_0_works() {
    algorithm_power(power_0);
}

#[test]
fn power_1_works() {
    algorithm_power(power_1);
}

#[test]
fn power_accumulate_0_works() {
    algorithm_power_accumulate(power_accumulate_0);
}

#[test]
fn power_accumulate_1_works() {
    algorithm_power_accumulate(power_accumulate_1);
}

#[test]
fn power_accumulate_2_works() {
    algorithm_power_accumulate(power_accumulate_2);
}

#[test]
fn power_accumulate_3_works() {
    algorithm_power_accumulate(power_accumulate_3);
}

#[test]
fn power_accumulate_4_works() {
    algorithm_power_accumulate(power_accumulate_4);
}

#[test]
fn power_accumulate_positive_works() {
    algorithm_power_accumulate_positive(power_accumulate_positive);
}

#[test]
fn power_accumulate_works() {
    algorithm_power_accumulate(power_accumulate);
}

#[test]
fn power_works() {
    algorithm_power(power);
}
