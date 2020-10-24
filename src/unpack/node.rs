#[derive(Clone, Debug)]
pub struct Node {
    value: Option<u8>,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    pub fn new_leaf(v: u8) -> Self {
        Node {
            value: Some(v),
            left_child: None,
            right_child: None,
        }
    }

    pub fn new_non_leaf(l: Node, r: Node) -> Self {
        Node {
            value: None,
            left_child: Some(Box::new(l)),
            right_child: Some(Box::new(r)),
        }
    }

    pub fn dfs(&self) {
        // go left first
        if let Some(node) = &self.left_child {
            node.dfs();
        }

        // go right
        if let Some(node) = &self.right_child {
            node.dfs();
        }

        // do the logic
        if let Some(val) = self.value {
            
        }

        println!("my value -> {:?}", self.value);
    }
}
