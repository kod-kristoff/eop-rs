pub fn run_tests() {
    log::info!("Running tests");
    test_ch_7::<i32, char>();
}

fn test_ch_7<Z, X>() {
    log::info!("  Chapter 7");
    algorithms_bifurcate_coordinates_stree();
}

#[test]
fn algorithms_bifurcate_coordinates_stree() {
    log::info!("   bifurcate coordinates: STree");

    use eop::trees::STree;
    use eop::{BifurcateCoordinate, height_recursive, weight_recursive};
    type T = STree<i32>;
    let t0 = T::new();
    let t4 = T::leaf(4);
    let t3_45 = T::tree(3, T::leaf(4), T::leaf(5));
    let t2_345_678 = T::tree(
        2,
        T::tree(3, T::leaf(4), T::leaf(5)),
        T::tree(6, T::leaf(7), T::leaf(8)),
    );
    //let t =

    let r = t2_345_678.coord();
    assert!(!r.is_empty());
    assert!(r.has_left_successor());
    let c_l = r.left_succesor();
    assert!(!c_l.is_empty());
    assert!(c_l.has_left_successor());
    let c_l_l = c_l.left_succesor();
    assert!(!c_l_l.is_empty());
    assert!(t0.coord().is_empty());

    assert_eq!(weight_recursive(t0.coord()), 0);
    assert_eq!(weight_recursive(t4.coord()), 1);
    assert_eq!(weight_recursive(t3_45.coord()), 3);

    assert_eq!(height_recursive(t0.coord()), 0);
    assert_eq!(height_recursive(t4.coord()), 1);
    assert_eq!(height_recursive(t3_45.coord()), 2);
}
