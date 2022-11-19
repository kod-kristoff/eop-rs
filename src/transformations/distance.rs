use crate::transformations::Transformation;

/// Precondition: y is reachable from x under f
pub fn distance<F, Args>(mut x: F::Domain, y: F::Domain, f: F) -> usize
where
    F: Transformation<Args>,
    F::Domain: PartialEq,
{
    let mut n = 0usize;
    while x != y {
        x = f.call(x);
        n = n + 1;
    }
    n
}
