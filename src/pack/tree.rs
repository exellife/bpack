use crate::pack::{Node, Topology};
use std::collections::BinaryHeap;

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn from_heap(heap: &mut BinaryHeap<Node>) -> Tree {
        let mut tree = Tree { root: None };
        loop {
            if heap.len() == 1 {
                let last = heap.pop().expect("no last item");
                tree.root = Some(Box::new(last));
                break;
            }

            // always select 2 nodes with minimal weight
            let first = heap.pop().expect("no first item");
            let second = heap.pop().expect("no second item");

            assert!(first.weight <= second.weight);

            let new_weight = first.weight + second.weight;
            let new_node = Node::new_non_leaf(first, second, new_weight);
            heap.push(new_node);
        }
        tree
    }

    pub fn dfs(&self) -> Option<(Topology, Vec<(u16, i8)>)> {
        if let Some(root) = &self.root {
            let mut counter: i8 = 0;
            let b: u16 = 0b0000_0000_0000_0000;

            let mut topo = Topology::new();
            let mut table: Vec<(u16, i8)> = vec![(0, -1); 95];

            root.dfs(&mut topo, &mut counter, b, &mut table);

            // println!("table -> {:?}", table);

            return Some((topo, table));
        }

        None
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use rand::prelude::*;
//     use std::time::{Duration, Instant};

//     #[test]
//     #[ignore]
//     fn build_tree_test_1() {
//         let mut rng = rand::thread_rng();
//         let now = Instant::now();
//         let stop = now + Duration::from_secs(60 * 60);
//         loop {
//             if Instant::now() >= stop {
//                 break;
//             }

//             // let after = Instant::now();
//             let n: u32 = rng.gen_range(1, 1000 * 1000);
//             let mut v_bts: Vec<u8> = vec![0; n as usize];
//             for i in 0..n as usize {
//                 let q = rng.gen_range(32, 127);
//                 // assert!(q - 32 <= 95);
//                 v_bts[i] = q;
//             }
//             // let before = Instant::now();
//             let mut heap = count_freq(&v_bts);
//             let mut tree = Tree::from_heap(&mut heap);
//             if let Some((topology, table)) = tree.dfs() {
//                 // println!("table -> {:?}", table);
//             }
//             // println!("it took {:?} for input {}", after - before, n);
//         }

//         assert_eq!(1, 2);
//     }
// }
