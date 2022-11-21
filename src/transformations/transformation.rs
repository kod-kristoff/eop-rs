pub trait Transformation<Args> {
    type Domain;
    type DistanceType; // TODO: use default

    fn call(&self, x: Self::Domain) -> Self::Domain;
}

impl<F, T> Transformation<(T,)> for F
where
    F: Fn(T) -> T,
{
    type Domain = T;
    type DistanceType = usize;
    
    fn call(&self, x: Self::Domain) -> Self::Domain {
        (self)(x)
    }
}
