use std::rc::Rc;

use crate::BifurcateCoordinate;

#[derive(Debug)]
pub struct STree<T> {
    root: STreeCoordinate<T>,
}

impl<T> STree<T> {
    pub fn new() -> Self {
        let root = STreeCoordinate::new();
        Self { root }
    }

    pub fn leaf(t: T) -> Self {
        let root = STreeCoordinate::leaf(t);
        Self { root }
    }

    pub fn tree(t: T, left: Self, right: Self) -> Self {
        let root = STreeCoordinate::tree(t, left.root, right.root);
        Self { root }
    }
}

impl<T> STree<T> {
    pub fn coord(&self) -> STreeCoordinate<T> {
        self.root.clone()
    }
}

#[derive(Debug)]
pub struct STreeCoordinate<T> {
    ptr: Option<Rc<STreeNode<T>>>,
}

impl<T> STreeCoordinate<T> {
    pub fn new() -> Self {
        Self { ptr: None }
    }

    pub fn leaf(value: T) -> Self {
        Self {
            ptr: Some(Rc::new(STreeNode {
                value,
                left_successor_link: None,
                right_successor_link: None,
            })),
        }
    }

    pub fn tree(value: T, left: Self, right: Self) -> Self {
        Self {
            ptr: Some(Rc::new(STreeNode {
                value,
                left_successor_link: left.ptr,
                right_successor_link: right.ptr,
            })),
        }
    }
}
impl<T> Clone for STreeCoordinate<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone(),
        }
    }
}

#[derive(Debug)]
struct STreeNode<T> {
    value: T,
    left_successor_link: Option<Rc<STreeNode<T>>>,
    right_successor_link: Option<Rc<STreeNode<T>>>,
}

impl<T> BifurcateCoordinate for STreeCoordinate<T> {
    fn is_empty(&self) -> bool {
        self.ptr.is_none()
    }

    fn has_left_successor(&self) -> bool {
        self.ptr
            .as_ref()
            .map_or(false, |node| node.left_successor_link.is_some())
    }

    fn left_succesor(&self) -> Self {
        match self.ptr.as_ref() {
            None => panic!("No left_succesor"),
            Some(node) => Self {
                ptr: node.left_successor_link.clone(),
            },
        }
    }

    fn has_right_successor(&self) -> bool {
        self.ptr
            .as_ref()
            .map_or(false, |node| node.right_successor_link.is_some())
    }

    fn right_succesor(&self) -> Self {
        match self.ptr.as_ref() {
            None => panic!("No right_succesor"),
            Some(node) => Self {
                ptr: node.right_successor_link.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree_is_empty() {
        let t = STree::<i32>::new();
        assert!(t.coord().is_empty());
    }
}
