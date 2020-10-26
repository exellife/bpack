use crate::unpack::Node;
use crate::unpack::Table;
use crate::{get_bit, set_bit};

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn from_topo(src: Vec<u8>) -> Self {
        let mut tree = Tree { root: None };

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
