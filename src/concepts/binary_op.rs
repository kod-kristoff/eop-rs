pub trait BinaryOperation<Args> {
    type Domain;

    fn call(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain;
}

impl<F, T> BinaryOperation<(T,)> for F
where
    F: Fn(T, T) -> T,
{
    type Domain = T;

    fn call(&self, x: Self::Domain, y: Self::Domain) -> Self::Domain {
        (self)(x, y)
    }
}
