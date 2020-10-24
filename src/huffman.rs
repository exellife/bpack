use crate::{get_bit, set_bit};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::{Duration, Instant};

fn count_freq(src: &[u8]) -> BinaryHeap<Node> {
    let mut count_map = HashMap::new();

    for i in src.iter() {
        let counter = count_map.entry(i).or_insert(0);
        *counter += 1;
    }

    let mut node_v: BinaryHeap<Node> = count_map
        .iter()
        .map(|x| Node {
            left_child: None,
            right_child: None,
            weight: *x.1,
            value: Some(**x.0),
        })
        .collect();
    node_v
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    weight: u32,
    value: Option<u8>,
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
    fn dfs(
        &self,
        topo: &mut Topology,
        counter: &mut i8,
        bits: u8,
        table: &mut HashMap<u8, (u8, u8)>,
    ) {
        // println!("counter -> {}", counter);
        // go left first
        if let Some(node) = &self.left_child {
            if *counter >= 8 {
                panic!(
                    "panic -> value -> {:?}, weight -> {:?}",
                    self.value, self.weight
                );
            }
            let code = set_bit(bits, (7 - *counter) as u8, 0);
            *counter += 1;
            node.dfs(topo, counter, code, table);
        }

        // go right
        if let Some(node) = &self.right_child {
            if *counter >= 8 {
                panic!(
                    "panic -> value -> {:?}, weight -> {:?}",
                    self.value, self.weight
                );
            }
            let code = set_bit(bits, (7 - *counter) as u8, 1);
            *counter += 1;
            node.dfs(topo, counter, code, table);
        }

        // do the logic
        if let Some(val) = self.value {
            // store val into the table with its code/bit
            // representation e.g
            // 97 -> (0100_0000, 3)
            // 97u8 could be represented as 010
            // encode the original input using this info

            assert_eq!(table.contains_key(&val), false);

            table.insert(val, (bits, *counter as u8));
            // println!(
            //     "my value -> {:?}, my weight -> {}, my code -> {:b}, counter -> {}",
            //     self.value, self.weight, bits, counter
            // );
        }
        *counter -= 1;
        topo.write(self.value);
    }
}

#[derive(Debug)]
struct Topology {
    vector: Vec<u8>,
    bit_index: u8, // 7 to 0 from right to left
}

impl Topology {
    pub fn new() -> Self {
        Topology {
            vector: vec![0],
            bit_index: 7,
        }
    }

    pub fn topo(&self) -> Vec<u8> {
        self.vector.clone()
    }

    pub fn write(&mut self, value: Option<u8>) {
        let l = self.vector.len() - 1;
        let mut current_byte = self.vector[l];

        if let Some(code) = value {
            current_byte = set_bit(current_byte, self.bit_index, 1);
            self.vector[l] = current_byte;
            if self.bit_index == 0 {
                // last index in the byte we just wrote to

                // the next entire byte is a code
                self.vector.push(code);
                // after that push another 0
                // so we can write to it
                self.vector.push(0);

                // set bit index to 7
                // starting point
                self.bit_index = 7;
                return;
            } else {
                self.bit_index -= 1;
            }

            // 0000 0000
            // 7654 3210
            // remaining empty bits in current byte
            let bits_left = self.bit_index + 1;
            // println!("current_byte -> {:b}", current_byte);
            // fill them, take first remaining bits of the code
            // and set them as the last bits of the current byte
            let to_set = code >> 8 - bits_left;

            // println!("to_set -> {:b}", to_set);
            current_byte |= to_set;
            self.vector[l] = current_byte;

            // take last bits if the code
            // and set them as the first bits
            let mut next_byte = 0b0000_0000;
            let mut bit_idx = 7;
            for i in (0..(8 - bits_left)).rev() {
                let bit = get_bit(code, i);
                next_byte = set_bit(next_byte, bit_idx, bit);
                bit_idx -= 1;
            }

            self.vector.push(next_byte);
            self.bit_index = bit_idx;
        } else {
            current_byte = set_bit(current_byte, self.bit_index, 0);
            self.vector[l] = current_byte;
            if self.bit_index == 0 {
                // last index we just wrote to
                self.vector.push(0);

                // reset
                self.bit_index = 7;
            } else {
                self.bit_index -= 1;
            }
        }
    }

    pub fn read(&self) -> Vec<u8> {
        let mut codes = vec![];

        let mut entire = false;

        let mut code = 0b0000_0000;

        let mut to_take = 0;
        let mut end = 8;
        for byte in self.vector.iter() {
            if entire {
                codes.push(*byte);
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
                codes.push(code);
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
                }
            }
            end = 8;
        }

        codes
    }
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
    topology: Topology,
    table: HashMap<u8, (u8, u8)>,
}

impl Tree {
    pub fn from(src: &[u8]) -> Self {
        let mut tree = Tree {
            root: None,
            topology: Topology::new(),
            table: HashMap::new(),
        };
        tree.build_tree(src);
        tree
    }

    pub fn from_topo(src: Vec<u8>) -> Self {
        let mut tree = Tree {
            root: None,
            topology: Topology::new(),
            table: HashMap::new(),
        };

        let mut stack: Vec<Node> = vec![];

        let mut entire = false;

        let mut code = 0b0000_0000;

        let mut to_take = 0;
        let mut end = 8;

        for byte in src.iter() {
            if entire {
                let node = Node {
                    value: Some(*byte),
                    left_child: None,
                    right_child: None,
                    weight: 0,
                };
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
                let node = Node {
                    value: Some(code),
                    left_child: None,
                    right_child: None,
                    weight: 0,
                };
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

                        let parent = Node {
                            value: None,
                            left_child: Some(Box::new(second)),
                            right_child: Some(Box::new(first)),
                            weight: 0,
                        };

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

    pub fn read_topo(&self) -> Vec<u8> {
        self.topology.read()
    }

    fn build_tree(&mut self, src: &[u8]) {
        let mut heap = count_freq(src);

        loop {
            if heap.len() == 1 {
                let last = heap.pop().expect("no last item");
                self.root = Some(Box::new(last));
                break;
            }

            // always select 2 nodes with minimal weight
            let first = heap.pop().expect("no first item");
            let second = heap.pop().expect("no second item");

            assert!(first.weight <= second.weight);

            let new_weight = first.weight + second.weight;
            let new_node = Node {
                left_child: Some(Box::new(first)),
                right_child: Some(Box::new(second)),
                weight: new_weight,
                value: None,
            };

            heap.push(new_node);
        }
    }

    pub fn dfs(&mut self) {
        if let Some(root) = &self.root {
            let mut counter: i8 = 0;
            let mut b = 0b0000_0000;
            root.dfs(&mut self.topology, &mut counter, b, &mut self.table);
            // println!("table -> {:?}", self.table);
        }
    }

    pub fn get_topology(&self) -> Vec<u8> {
        self.topology.topo()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn sample() {
        let mut rng = rand::thread_rng();

        let start = Instant::now();
        let stop = start + Duration::from_secs(60);

        loop {
            if Instant::now() >= stop {
                break;
            }

            let mut bts = vec![];
            // let mut set = HashSet::new();
            let i = rng.gen_range(256, 4096);
            for _ in 0..i {
                let q = rng.gen_range(32, 127);
                // let q_2 = q;
                bts.push(q);
                // set.insert(q_2);
            }
            let mut tree = Tree::from(&bts);
            tree.dfs();
        }

        // let topo = tree.get_topology();
        // let mut tree_2 = Tree::from_topo(topo);
        // tree_2.dfs();

        assert_eq!(1, 2);
    }
}
