use crate::unpack::Node;
use crate::unpack::Table;
use crate::{get_bit, get_bit_16, set_bit};

pub struct Tree {
    root: Option<Box<Node>>,
    current: Option<Box<Node>>, // TODO reconsider this
}

impl Tree {
    pub fn handle(&mut self, byte: u8, v: &mut Vec<u8>, v_idx: &mut usize, o_size: usize) {
        // 0 -> left, 1 -> right

        if self.current.is_none() {
            // println!("setting current");
            assert!(self.root.is_some());
            self.current = self.root.clone();
        }

        for i in (0..8).rev() {
            if *v_idx >= o_size {
                break;
            }
            let bit = get_bit(byte, i);

            // TODO
            // cache set bit (bit)

            if let Some(node) = self.current.as_ref().unwrap().next(bit) {
                if let Some(code) = node.value {
                    // cache add code (code)
                    // to the bit we were setting earlier
                    v[*v_idx] = code + 32;
                    *v_idx += 1;
                    self.current = self.root.clone();
                } else {
                    self.current = Some(node);
                }
            }
        }
        // println!("about to exit");
    }

    pub fn from_topo_2(src: Vec<u8>) -> Self {
        let mut tree = Tree {
            root: None,
            current: None,
        };

        let mut stack: Vec<Node> = vec![];

        let mut b_idx = 8;
        let mut skip = false;
        for idx in 0..src.len() - 1 {
            if skip {
                skip = false;
                b_idx = 8;
                continue;
            }

            let (byte_1, byte_2) = (src[idx], src[idx + 1]);
            println!("byte_1 -> {}, byte_2 -> {}", byte_1, byte_2);
            for i in (0..b_idx).rev() {
                if get_bit(byte_1, i) == 1 {
                    if i == 0 {
                        assert!(byte_2 <= 95);
                        // means that entire next byte
                        // is a code we are looking for
                        // so push byte_2 into the stack
                        // and skip next iteration
                        stack.push(Node::new_leaf(byte_2));
                        skip = true;
                    } else {
                        // get the remaining bits from byte_1
                    }

                    break;
                } else {
                    if stack.len() > 1 {
                        let first = stack.pop().expect("no first node in stack");

                        let second = stack.pop().expect("no second node in stack");
                        let parent = Node::new_non_leaf(second, first);
                        stack.push(parent);
                    } else if stack.len() == 1 {
                        let root = stack.pop().expect("no root found");
                        tree.root = Some(Box::new(root));
                    }
                }
            }
        }

        tree
    }

    pub fn from_topo(src: Vec<u8>) -> Self {
        let mut tree = Tree {
            root: None,
            current: None,
        };

        let mut stack: Vec<Node> = vec![];

        let mut entire = false;

        let mut code = 0b0000_0000;

        let mut to_take = 0;
        let mut end = 8;

        for byte in src.iter() {
            if entire {
                // assert!(*byte <= 95);
                let node = Node::new_leaf(*byte);
                stack.push(node);
                entire = false;
                end = 8;
                to_take = 0;
                continue;
            }

            if to_take != 0 {
                let mut bit_idx = 7;

                for i in (0..to_take).rev() {
                    let bit = get_bit(*byte, bit_idx);
                    code = set_bit(code, i, bit);
                    bit_idx -= 1;
                }
                // assert!(code <= 95);
                let node = Node::new_leaf(code);
                stack.push(node);
                to_take = 0;
                code = 0b0000_0000;
                end = bit_idx + 1;
            }

            for i in (0..end).rev() {
                let bit = get_bit(*byte, i);

                // found 1 at i position
                // 0000_0000
                // 7654 3210
                if bit == 1 {
                    if i == 0 {
                        // if this is a last bit in byte
                        // next entire byte is a code
                        entire = true;
                        break;
                    } else {
                        // next i bits are part of the code
                        let mut bit_idx = 7;
                        for j in (0..i).rev() {
                            let bit = get_bit(*byte, j);
                            code = set_bit(code, bit_idx, bit);
                            bit_idx -= 1;
                        }
                        to_take = 8 - i;
                        break;
                    }
                } else {
                    if stack.len() > 1 {
                        let first = stack.pop().expect("no first node in stack");

                        let second = stack.pop().expect("no second node in stack");
                        let parent = Node::new_non_leaf(second, first);
                        stack.push(parent);
                    } else if stack.len() == 1 {
                        let root = stack.pop().expect("no root found");
                        tree.root = Some(Box::new(root));
                    }
                }
            }
            end = 8;
        }

        tree
    }

    // byte_1 = 0000_1010
    // byte_2 = 0010_0101

    // idx -> 7 byte_1[7] = 0;
    // idx -> 6 byte_1[6] = 0;
    // idx -> 5 byte_1[5] = 0;
    // idx -> 4 byte_1[4] = 0;
    // idx -> 3 byte_1[3] = 1; // found leaf

    // bits we are interested in byte_1 -> 010
    // last 3 bits
    // to get last 3 bits
    // set first 5 bits to 0 and shift left by 5
    // to_push = ((0xff >> 5) & byte_1) << 5

    // then get first 5 bits of the byte_2
    // and set them as last bits of the to_push
    // bits we are intersted in byte_2 -> 0010_0
    // first 5 bits
    // byte_2 >> 3 = 000 0_0100 (101 overflow) starting point on the next iteration
    // to_push |= (byte_2 >> 3) = 010 0_0100 (code)
    // push(to_push);
    // idx = 3
    // next iteration
    // byte_2(prev) becomes byte_1 with idx = 0..3
    // new byte_2 = 0b1110_1010
    // at i = 2 we found 1 so next 2 bits are what we need
    // to_push = ((0xff >> (8 - 2)) & byte_1) << (8-2)
    // to_push = (0b0000_0010 & 0b0000_0001) << 6
    // to_push = (0b0000_0001) << 6
    // to_push = 0b0100_0000
    // how many bits we need from next byte (byte_2)
    // 8 - 2 = 6
    // take first 6 bits from next byte
    // and make them last bits of the "to_push"
    // byte_2 = 0b1110_1010 >> (8 - 6)
    // byte_2 = 0b0011_1010
    // set to_push
    // to_push |= 0b0011_1010
    // to_push = 0b0100_0000 | 0b0011_1010
    // to_push = 0b0111_1010

    pub fn dfs(&self, table: &mut Table) {
        if let Some(root) = &self.root {
            let mut counter: i8 = 0;
            let b: u16 = 0b0000_0000_0000_0000;
            root.dfs(&mut counter, b, table);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn tree_from_topo_test() {
        let topo: Vec<u8> = vec![
            174, 84, 144, 121, 121, 91, 166, 37, 52, 97, 31, 141, 171, 181, 146, 25, 49, 80, 101,
            193, 36, 148, 42, 48, 180, 168, 140, 195, 212, 57, 104, 163, 198, 146, 120, 233, 6,
            172, 41, 211, 240, 66, 166, 104, 10, 47, 22, 85, 228, 194, 65, 64, 72, 81, 57, 26, 57,
            24, 9, 149, 81, 83, 155, 16, 74, 90, 69, 59, 17, 24, 130, 7, 73, 33, 179, 116, 250, 28,
            10, 45, 17, 9, 164, 19, 201, 202, 189, 43, 21, 8, 66, 5, 44, 42, 209, 69, 62, 180, 21,
            90, 106, 11, 21, 35, 211, 164, 226, 104, 41, 84, 36, 74, 74, 79, 102, 168, 4, 192, 0,
        ];

        // let tree = Tree::from_topo(topo);
        let tree2 = Tree::from_topo_2(topo);
        // tree.dfs();

        assert_eq!(1, 2);
    }
}
