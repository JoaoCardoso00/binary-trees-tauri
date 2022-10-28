use std::collections::VecDeque;

use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]
pub struct BinaryTree {
    node_value: i64,
    left_child: Option<Box<BinaryTree>>,
    right_child: Option<Box<BinaryTree>>
}

impl BinaryTree {
    pub fn new(value: i64) -> Self {
        BinaryTree {
            node_value: value,
            left_child: None,
            right_child: None
        }
    }
    pub fn from(new_values: &[i64]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: BinaryTree = BinaryTree::new(*first);

        for value in rest {
            root.insert(*value)
        }
        root
    }

    pub fn insert(&mut self, new_value: i64) {
        let mut queue: VecDeque<&mut BinaryTree> = VecDeque::new();
        queue.push_front(self);
        loop {
            let BinaryTree {
                ref mut left_child,
                ref mut right_child,
                ..
            } = queue.pop_back().unwrap();

            match left_child {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left_child = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }

            match right_child {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right_child = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
        }
    }

    pub fn left(mut self, node: BinaryTree) -> Self {
        self.left_child = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: BinaryTree) -> Self {
        self.right_child = Some(Box::new(node));
        self
    }
}