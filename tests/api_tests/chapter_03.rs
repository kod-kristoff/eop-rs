use std::ops::{Add, Mul};

use eop::math::power_binary::power_left_associated;

fn plus<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
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
    fn works_with_plus() {
        let a = 5;
        assert_eq!(power_left_associated(a, 5, plus), 25);
    }
}

mod power_right_associated {
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
    fn works_with_multiply() {
        let a = 5;
        assert_eq!(power_left_associated(a, 5, plus), 25);
    }
}
