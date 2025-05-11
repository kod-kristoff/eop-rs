use std::{fmt::Debug, ops::Add};

use eop::{
    trees::{BinaryTree, Tree},
    BifurcateCoordinate,
};
use num_traits::{One, Zero};

fn algorithms_bifurcate_coordinates<T>()
where
    T: BinaryTree,
    T::ValueType: Zero + One + Add + Copy,
    T::CoordinateType: BifurcateCoordinate,
{
    use eop::{height_recursive, weight_recursive, BifurcateCoordinate};
    let v1 = T::ValueType::one();
    let v2 = v1 + One::one();
    let v3 = v2 + One::one();
    let v4 = v3 + One::one();
    let v5 = v4 + One::one();

    let t0 = T::new();
    let t4 = T::leaf(v4);
    let t3_45 = T::tree(v3, T::leaf(v4), T::leaf(v5));
    //let t =

    let r = t3_45.coord();
    assert!(!r.is_empty());
    assert!(r.has_left_successor());
    let c_l = r.left_successor();
    assert!(t0.coord().is_empty());

    assert_eq!(weight_recursive(t0.coord()), 0);
    assert_eq!(weight_recursive(t4.coord()), 1);
    assert_eq!(weight_recursive(t3_45.coord()), 3);

    assert_eq!(height_recursive(t0.coord()), 0);
    assert_eq!(height_recursive(t4.coord()), 1);
    assert_eq!(height_recursive(t3_45.coord()), 2);
}

#[test]
fn algorithms_bifurcate_coordinates_stree_i32() {
    use eop::trees::STree;
    algorithms_bifurcate_coordinates::<STree<i32>>();
    algorithms_bifurcate_coordinates::<Tree<u8>>();
}
