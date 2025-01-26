struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        let new_leaf = Some(Box::new(TreeNode {
            value: value,
            right: None,
            left: None,
        }));

        if self.left.is_none() && self.right.is_none() {
            if value > self.value {
                self.right = new_leaf;
                return;
            } else if value < self.value {
                self.left = new_leaf;
                return;
            }
        }

        if let Some(node) = &mut self.left {
            node.insert(value);
        } else {
            self.left = new_leaf;
            return;
        }

        if let Some(node) = &mut self.right {
            node.insert(value);
        } else {
            self.right = new_leaf;
            return;
        }
    }

    fn find(&self, value: i32) -> bool {
        if value == self.value {
            return true;
        }

        if self.left.is_none() && self.right.is_none() {
            return false;
        } else {
            if value > self.value {
                if let Some(node) = &self.right {
                    return node.find(value);
                }
            }

            if value < self.value {
                if let Some(node) = &self.left {
                    return node.find(value);
                }
            }
        }

        false
    }
}

pub fn run_training_one() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(7);

    println!("Find 7: {}", root.find(7)); // Expected: true
    println!("Find 20: {}", root.find(20)); // Expected: false
}
