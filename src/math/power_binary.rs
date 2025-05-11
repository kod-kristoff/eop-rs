use crate::concepts::{BinaryOperation, Integer};

/// Precondition: `n > 0`
pub fn power_left_associated<I, Op, Args>(a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    I: Integer,
    Op: BinaryOperation<Args> + Copy,
    Op::Domain: Clone,
{
    // precondition: n > 0
    if n == I::one() {
        return a;
    }
    op.call(power_left_associated(a.clone(), n.predecessor(), op), a)
}

/// Precondition: `n > 0`
pub fn power_right_associated<I, Op, Args>(a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    I: Integer,
    Op: BinaryOperation<Args> + Copy,
    Op::Domain: Clone,
{
    // precondition: n > 0
    if n == I::one() {
        return a;
    }
    op.call(a.clone(), power_right_associated(a, n.predecessor(), op))
}
