pub trait UnaryPredicate<Args> {
    type Domain;

    fn call(&self, x: Self::Domain) -> bool;
}

impl<P, T> UnaryPredicate<(T,)> for P
where
    P: Fn(T) -> bool,
{
    type Domain = T;

    fn call(&self, x: Self::Domain) -> bool {
        (self)(x)
    }
}
