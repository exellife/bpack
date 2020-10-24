#[derive(Clone, Debug)]
pub struct Node {
    pub value: Option<u8>,
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
        if let Some(val) = self.value {}

        println!("my value -> {:?}", self.value);
    }

    pub fn next(&self, bit: u8) -> Option<Box<Node>> {
        // 1. check if self has value
        // if it is, then we are at the leaf node
        // println!("bit -> {}", bit);
        // if let Some(code) = self.value {
        //     v[*v_idx] = code + 32;
        //     *v_idx += 1;
        //     println!("bit in -> {}", bit);
        //     println!("code -> {}, counter -> {}", code, *c);
        //     return None;
        // }

        // 2. check if bit is 0 or 1
        assert!(bit == 0 || bit == 1);
        if bit == 0 {
            return self.left_child.clone();
        } else {
            return self.right_child.clone();
        }
        // Some(self.clone())
    }
}
