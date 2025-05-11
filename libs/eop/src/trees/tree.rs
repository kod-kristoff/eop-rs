use std::rc::{Rc, Weak};

use bifurcate_coordinate::{
    BidirectionalBifurcateCoordinate, BidirectionalBifurcateCoordinateMut, BifurcateCoordinate,
    BifurcateCoordinateMut,
};

use super::BinaryTree;

pub struct Tree<T> {
    root: TreeCoordinate<T>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            root: TreeCoordinate::default(),
        }
    }
}
pub struct TreeCoordinate<T> {
    node: Option<Rc<TreeNode<T>>>,
}
impl<T> Default for TreeCoordinate<T> {
    fn default() -> Self {
        Self { node: None }
    }
}

pub struct TreeNode<T> {
    value: T,
    left_successor_link: Option<Rc<TreeNode<T>>>,
    right_successor_link: Option<Rc<TreeNode<T>>>,
    predecessor_link: Option<Rc<TreeNode<T>>>,
}

impl<T> BinaryTree for Tree<T> {
    type ValueType = T;
    type CoordinateType = TreeCoordinate<T>;

    fn leaf(value: Self::ValueType) -> Self {
        Self {
            root: TreeCoordinate::leaf(value),
        }
    }

    fn tree(value: Self::ValueType, left: Self, right: Self) -> Self {
        Self {
            root: TreeCoordinate::tree(value, left.root, right.root),
        }
    }
    fn coord(&self) -> Self::CoordinateType {
        self.root.clone()
    }
}

impl<T> Clone for TreeCoordinate<T> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
        }
    }
}
impl<T> TreeCoordinate<T> {
    fn leaf(value: T) -> Self {
        Self {
            node: Some(Rc::new(TreeNode {
                value,
                left_successor_link: None,
                right_successor_link: None,
                predecessor_link: None,
            })),
        }
    }
    fn tree(value: T, left: Self, right: Self) -> Self {
        let mut res = Self::leaf(value);
        res.set_left_successor(left);
        res.set_right_successor(right);
        res
    }
}

impl<T> BifurcateCoordinate for TreeCoordinate<T> {
    fn is_empty(&self) -> bool {
        self.node.is_none()
    }

    fn left_successor(&self) -> Option<Self> {
        self.node.as_ref().map(|node| Self {
            node: node.left_successor_link.clone(),
        })
    }

    fn right_successor(&self) -> Option<Self> {
        self.node.as_ref().map(|node| Self {
            node: node.right_successor_link.clone(),
        })
    }
}

impl<T> BidirectionalBifurcateCoordinate for TreeCoordinate<T> {
    fn has_predecessor(&self) -> bool {
        self.node
            .as_ref()
            .map_or(false, |node| node.predecessor_link.is_some())
    }
    fn predecessor(&self) -> Self {
        match self.node.as_ref() {
            None => panic!("No predecessor"),
            Some(node) => Self {
                node: node.predecessor_link.clone(),
            },
        }
    }
}

impl<T> BifurcateCoordinateMut for TreeCoordinate<T> {
    fn set_left_successor(&mut self, mut l: Self) {
        match &mut self.node {
            None => (),
            Some(node) => {
                if let Some(left) = l.node.take() {
                    node.left_successor_link.replace(left);
                } else {
                    node.left_successor_link.take();
                }
            }
        }
    }
    fn set_right_successor(&mut self, mut l: Self) {
        todo!()
    }
}

impl<T> BidirectionalBifurcateCoordinateMut for TreeCoordinate<T> {
    fn set_left_successor(&mut self, mut l: Self) {
        match &mut self.node {
            None => (),
            Some(node) => {
                &mut node.left_successor_link = l.node;
                if !l.is_empty() {
                    l.set_predecessor(node);
                }
            }
        }
    }
    fn set_right_successor(&mut self, mut l: Self) {
        todo!()
    }
    fn set_predecessor(&mut self, p: Self) {
        match &mut self.node {
            None => (),
            Some(node) => node.predecessor_link = p,
        }
    }
}
