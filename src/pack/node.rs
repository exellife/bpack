use crate::pack::Topology;
use crate::set_bit_16;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node {
    value: Option<u8>,
    pub weight: u32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Ord for Node {
    // nodes with minimal weight go left
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new_leaf(v: u8) -> Self {
        Node {
            value: Some(v),
            weight: 0,
            left_child: None,
            right_child: None,
        }
    }

    pub fn new_non_leaf(l: Node, r: Node, w: u32) -> Self {
        Node {
            left_child: Some(Box::new(l)),
            right_child: Some(Box::new(r)),
            weight: w,
            value: None,
        }
    }

    pub fn dfs(
        &self,
        topo: &mut Topology,
        counter: &mut i8,
        bits: u16,
        table: &mut Vec<(u16, i8)>,
    ) {
        // go left first
        if let Some(node) = &self.left_child {
            *counter += 1;

            let idx = (!(*counter - 16) + 1) as u16;
            let code = set_bit_16(bits, idx, 0);

            node.dfs(topo, counter, code, table);
        }

        // go right
        if let Some(node) = &self.right_child {
            *counter += 1;

            let idx = (!(*counter - 16) + 1) as u16;
            let code = set_bit_16(bits, idx, 1);

            node.dfs(topo, counter, code, table);
        }

        // 1 -> 7
        // 2 -> 6
        // 3 -> 5
        // 4 -> 4
        // 5 -> 3
        // 6 -> 2
        // 7 -> 1
        // 8 -> 0

        // do the logic
        if let Some(val) = self.value {
            // store val into the table with its code/bit
            // representation e.g
            // 97 -> (0100_0000, 3)
            // 97u8 could be represented as 010
            // encode the original input using this info
            if *counter > 16 {
                panic!("counter > 16");
            }
            // println!("SETTING BITS! my value -> {:?}, my weight -> {:?}, my code -> {:b}, i take {} bits", val, self.weight, bits, *counter);
            assert_eq!(table[val as usize], (0, -1));
            table[val as usize] = (bits, *counter);

            // println!(
            //     "my value -> {:?}, my weight -> {}, my code -> {:b}, counter -> {}",
            //     self.value, self.weight, bits, counter
            // );
            // println!("my value -> {:?}", self.value);
        }
        *counter -= 1;
        // println!("my value -> {:?}", self.value);
        topo.write(self.value); // + 32?
    }
}

// let idx_2: u16;
// let diff = *counter - 16;
// if diff < 0 {
//     idx_2 = -(diff) as u16;
// } else if diff == 0 {
//     idx_2 = 0;
// } else {
//     panic!("counter - {} > 16", counter);
// }
// assert_eq!(idx, idx_2);
