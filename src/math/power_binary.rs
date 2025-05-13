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

/// Precondition: `associative(op) && n > 0`
pub fn power_1<I, Op, Args>(a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n == I::one() {
        return a;
    }
    let mut r = power_0(op.call(a, a), n.half_nonnegative(), op);
    if n.is_odd() {
        r = op.call(r, a);
    }
    r
}

/// Precondition: `associative(op) && n >= 0`
pub fn power_accumulate_0<I, Op, Args>(mut r: Op::Domain, a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n.is_zero() {
        return r;
    }
    if n.is_odd() {
        r = op.call(r, a);
    }
    power_accumulate_0(r, op.call(a, a), n.half_nonnegative(), op)
}

/// Precondition: `associative(op) && n >= 0`
pub fn power_accumulate_1<I, Op, Args>(mut r: Op::Domain, a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n.is_zero() {
        return r;
    }
    if n == I::one() {
        return op.call(r, a);
    }
    if n.is_odd() {
        r = op.call(r, a);
    }
    power_accumulate_1(r, op.call(a, a), n.half_nonnegative(), op)
}

/// Precondition: `associative(op) && n >= 0`
pub fn power_accumulate_2<I, Op, Args>(mut r: Op::Domain, a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n.is_odd() {
        r = op.call(r, a);
        if n == I::one() {
            return r;
        }
    } else if n.is_zero() {
        return r;
    }
    power_accumulate_2(r, op.call(a, a), n.half_nonnegative(), op)
}

/// Precondition: `associative(op) && n >= 0`
pub fn power_accumulate_3<I, Op, Args>(
    mut r: Op::Domain,
    mut a: Op::Domain,
    mut n: I,
    op: Op,
) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n.is_odd() {
        r = op.call(r, a);
        if n == I::one() {
            return r;
        }
    } else if n.is_zero() {
        return r;
    }
    a = op.call(a, a);
    n = n.half_nonnegative();
    power_accumulate_3(r, a, n, op)
}

/// Precondition: `associative(op) && n >= 0`
pub fn power_accumulate_4<I, Op, Args>(
    mut r: Op::Domain,
    mut a: Op::Domain,
    mut n: I,
    op: Op,
) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    loop {
        if n.is_odd() {
            r = op.call(r, a);
            if n == I::one() {
                return r;
            }
        } else if n.is_zero() {
            return r;
        }
        a = op.call(a, a);
        n = n.half_nonnegative();
    }
}

/// Precondition: `associative(op) && n > 0`
pub fn power_accumulate_positive<I, Op, Args>(
    mut r: Op::Domain,
    mut a: Op::Domain,
    mut n: I,
    op: Op,
) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    loop {
        if n.is_odd() {
            r = op.call(r, a);
            if n == I::one() {
                return r;
            }
        }
        a = op.call(a, a);
        n = n.half_nonnegative();
    }
}

/// Precondition: `associative(op) && n >= 0`
pub fn power_accumulate<I, Op, Args>(r: Op::Domain, a: Op::Domain, n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n.is_zero() {
        return r;
    }
    power_accumulate_positive(r, a, n, op)
}

/// Precondition: `associative(op) && positive(n)`
pub fn power<I, Op, Args>(mut a: Op::Domain, mut n: I, op: Op) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    while n.is_even() {
        a = op.call(a, a);
        n = n.half_nonnegative();
    }
    n = n.half_nonnegative();
    if n.is_zero() {
        return a;
    }
    power_accumulate_positive(a, op.call(a, a), n, op)
}

/// Precondition: `associative(op) && !negative(n)`
pub fn power_with_identity<I, Op, Args>(a: Op::Domain, n: I, op: Op, id: Op::Domain) -> Op::Domain
where
    Op: BinaryOperation<Args> + Copy,
    I: Integer,
    Op::Domain: Copy,
{
    if n.is_zero() {
        return id;
    }
    power(a, n, op)
}
