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

/// Precondition: `associative(op) && n > 0`
pub fn power_0<I, Op, Args>(a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n == I::one() {
        return a;
    }
    if n.is_even() {
        power_0(op.call(a, a), n.half_nonnegative(), op)
    } else {
        op.call(power_0(op.call(a, a), n.half_nonnegative(), op), a)
    }
}
