mod node;
// mod result_src;
mod table;
mod tree;

pub(self) use node::Node;
pub(self) use table::Table;
pub(self) use tree::Tree;


/// Decompresses byte vector
///
/// # Examples
///
/// ```
/// use bpack::{pack, unpack};
///
/// let data = "some very long string".as_bytes();
/// if let Some(packed) = pack(data) {
///     let unpacked = unpack(packed);
///     assert_eq!(data, unpacked.as_slice());
/// }
///
/// ```

pub fn unpack(packed: Vec<u8>) -> Vec<u8> {
    let (first, second) = (packed[0], packed[1]);
    let original_size = (((first as u16) << 8) | second as u16) as usize;
    let topo: Vec<u8> = packed[2..121].to_vec();

    let tree = Tree::from_topo(topo);
    let mut table = Table::new();
    tree.dfs(&mut table);

    let mut result = vec![0; original_size];

    let mut idx = 0;

    for i in 121..packed.len() {
        table.handle(packed[i], &mut result, &mut idx, original_size);
    }

    result
}

// #[cfg(test)]
// mod test {
//     use result_src::{result_, src_};
//     use super::*;
//     use std::time::{Instant};
//     #[test]
//     #[ignore]
//     fn unpack_test() {
//         let packed = result_();
//         let src = src_();
//         println!("src -> {}", src.len());
//         let before = Instant::now();
//         let a = unpack(packed);
//         let after = Instant::now();
//         // println!("result -> {:?}", a);
//         assert_eq!(a, src);
//         for i in 0..a.len() {
//             if a[i] != src[i] {
//                 println!("idx -> {} a_val -> {}, src_val -> {}", i, a[i], src[i]);
//             }
//         }
//         println!("time taken => {:?}", after - before);
//         assert_eq!(1, 2);
//     }
// }
