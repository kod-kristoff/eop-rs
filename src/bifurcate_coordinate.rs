pub trait BifurcateCoordinate {
    fn is_empty(&self) -> bool;

    fn has_left_successor(&self) -> bool;
    fn left_succesor(&self) -> Self;

    fn has_right_successor(&self) -> bool;
    fn right_succesor(&self) -> Self;
}

pub fn height_recursive(c: impl BifurcateCoordinate) -> usize {
    if c.is_empty() {
        return 0;
    }
    let l = if c.has_left_successor() {
        height_recursive(c.left_succesor())
    } else {
        0
    };
    let r = if c.has_right_successor() {
        height_recursive(c.right_succesor())
    } else {
        0
    };
    1 + l.max(r)
}

pub fn weight_recursive(c: impl BifurcateCoordinate) -> usize {
    if c.is_empty() {
        return 0;
    }
    let l = if c.has_left_successor() {
        weight_recursive(c.left_succesor())
    } else {
        0
    };
    let r = if c.has_right_successor() {
        weight_recursive(c.right_succesor())
    } else {
        0
    };
    1 + l + r
}
