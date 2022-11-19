pub trait Transformation<Args> {
    type Domain;

    fn call(&self, x: Self::Domain) -> Self::Domain;
}

impl<F, T> Transformation<(T,)> for F
where
    F: Fn(T) -> T,
{
    type Domain = T;
    fn call(&self, x: Self::Domain) -> Self::Domain {
        (self)(x)
    }
}
