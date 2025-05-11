pub trait BifurcateCoordinate: Sized {
    fn is_empty(&self) -> bool;

    fn left_successor(&self) -> Option<Self>;

    fn right_successor(&self) -> Option<Self>;
}

pub trait BifurcateCoordinateMut {
    fn set_left_successor(&mut self, l: Self);
    fn set_right_successor(&mut self, r: Self);
}

pub fn height_recursive<C: BifurcateCoordinate>(c: &C) -> usize {
    if c.is_empty() {
        return 0;
    }
    let l = c.left_successor().map_or(0, |succ| height_recursive(&succ));
    let r = c
        .right_successor()
        .map_or(0, |succ| height_recursive(&succ));

    1 + l.max(r)
}

pub fn weight_recursive<C: BifurcateCoordinate>(c: &C) -> usize {
    if c.is_empty() {
        return 0;
    }
    let l = c.left_successor().map_or(0, |succ| weight_recursive(&succ));
    let r = c
        .right_successor()
        .map_or(0, |succ| weight_recursive(&succ));

    1 + l + r
}
