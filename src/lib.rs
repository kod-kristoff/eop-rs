mod bifurcate_coordinate;
pub mod concepts;
pub mod core;
pub mod math;
pub mod transformations;
pub mod trees;

pub use bifurcate_coordinate::BifurcateCoordinate;
pub use bifurcate_coordinate::{height_recursive, weight_recursive};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
