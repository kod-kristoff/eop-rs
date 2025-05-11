use crate::{bifurcate_coordinate::BifurcateCoordinateMut, BifurcateCoordinate};

pub trait BidirectionalBifurcateCoordinate: BifurcateCoordinate {
    fn has_predecessor(&self) -> bool;
    fn predecessor(&self) -> Self;
}

pub trait BidirectionalBifurcateCoordinateMut: BifurcateCoordinateMut {
    // fn set_left_successor(&mut self, l: Self);
    // // {
    // //     BifurcateCoordinateMut::set_left_successor(&mut self, l);
    // //     l.set_predecessor(self);
    // // }

    // fn set_right_successor(&mut self, r: Self);

    fn set_predecessor(&mut self, p: Self);
}
