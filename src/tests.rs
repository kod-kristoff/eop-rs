
pub fn run_tests() {
    log::info!("Running tests");
    test_ch_7::<i32, char>();
}


fn test_ch_7<Z,X>() {
    log::info!("  Chapter 7");
    algorithms_bifurcate_coordinates_stree();
}

fn algorithms_bifurcate_coordinates_stree() {

    log::info!("   bifurcate coordinates: STree");
    
    use eop::trees::STree;
    use eop::{
        height_recursive, weight_recursive,
        BifurcateCoordinate
    };
    type T = STree<i32>;
    let t0 = T::new();
    let t4 = T::leaf(4);
    let t3_45 = T::tree(3, T::leaf(4), T::leaf(5));
    //let t =

    let r = t3_45.coord();
    assert!(!r.is_empty());
    assert!(r.has_left_successor());
    let c_l = r.left_succesor();
    assert!(t0.coord().is_empty());
    
    assert_eq!(weight_recursive(t0.coord()), 0);
    assert_eq!(weight_recursive(t4.coord()), 1);
    assert_eq!(weight_recursive(t3_45.coord()), 3);
    
    assert_eq!(height_recursive(t0.coord()), 0);
    assert_eq!(height_recursive(t4.coord()), 1);
    assert_eq!(height_recursive(t3_45.coord()), 2);
}
