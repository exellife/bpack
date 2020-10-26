mod node;
mod result_src;
mod table;
mod tree;

pub(self) use node::Node;
pub(self) use result_src::{result_, src_};
pub(self) use table::Table;
pub(self) use tree::Tree;

pub fn unpack(packed: Vec<u8>) -> Vec<u8> {
    let (first, second) = (packed[0], packed[1]);
    let original_size = (((first as u16) << 8) | second as u16) as usize;
    let topo: Vec<u8> = packed[2..121].to_vec();
    // let encoded: Vec<u8> = packed[121..].to_vec();
    let mut tree = Tree::from_topo(topo);
    let mut table = Table::new();
    tree.dfs(&mut table);
    // table.print_k();
    // table.handle(packed[121]);
    // table.handle(packed[122]);
    // table.handle(packed[123]);
    // table.handle(packed[124]);
    let mut result = vec![0; original_size];
    // tree.dfs();
    let mut idx = 0;
    let mut counter = 0;
    // 0000_0100_0000_0000 // 20
    // 0010_0100_0000_0000 // 80
    //
    // for byte in [0b0000_0100, 0b1001_0000].iter() {
    //     tree.handle(*byte, &mut result, &mut idx, &mut counter);
    // }
    for i in 121..packed.len() {
        table.handle(packed[i], &mut result, &mut idx, original_size);

        // it should return u8
        // so result[idx] = returned result from tree handle
        // in some cases packed[i] might not lead to a leaf
        // so it should return Option<u8>
        // e.g
        // if let Some(code) = tree.handle(packed[i]) {
        //      result[idx] = code;
        //      idx += 1;
        // }
        // if idx >= original_size {
        //     break;
        // }
        // // println!("byte -> {}", packed[i]);
        // tree.handle(packed[i], &mut result, &mut idx, original_size);
    }
    // println!("idx => {}", idx);
    result
}

#[cfg(test)]
mod test {

    use super::*;
    use std::time::{Duration, Instant};
    #[test]
    #[ignore]
    fn unpack_test() {
        let packed = result_();
        let src = src_();
        println!("src -> {}", src.len());
        let before = Instant::now();
        let a = unpack(packed);
        let after = Instant::now();
        // println!("result -> {:?}", a);
        assert_eq!(a, src);
        for i in 0..a.len() {
            if a[i] != src[i] {
                println!("idx -> {} a_val -> {}, src_val -> {}", i, a[i], src[i]);
            }
        }
        println!("time taken => {:?}", after - before);
        assert_eq!(1, 2);
    }
}
