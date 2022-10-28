#[derive(Default)]
pub struct BinaryTree {
    node_value: i64,
    left_child: Option<Box<BinaryTree>>,
    right_child: Option<Box<BinaryTree>>
}

impl BinaryTree {
    pub fn new(value: i64) -> Self {
        BinaryTree {
            node_value: value,
            ..Default::default()
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