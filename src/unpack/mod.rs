mod node;
mod result_src;
mod tree;
pub(self) use node::Node;
pub(self) use result_src::{result_, src_};
pub(self) use tree::Tree;

pub fn unpack(packed: Vec<u8>) {
    let (first, second) = (packed[0], packed[1]);
    let original_size = (((first as u16) << 8) | second as u16) as usize;
    let topo: Vec<u8> = packed[2..121].to_vec();
    // let encoded: Vec<u8> = packed[121..].to_vec();
    let tree = Tree::from_topo(topo);
    let result = vec![0; original_size];

    let mut idx = 0;
    for i in 121..packed.len() {
        // tree.handle(packed[i]);
    }

    
}

#[cfg(test)]
mod test {

    use super::*;
    use std::time::{Duration, Instant};
    #[test]
    // #[ignore]
    fn unpack_test() {
        let packed = result_();
        // println!("v size -> {}", v.len());
        let before = Instant::now();
        unpack(packed);
        let after = Instant::now();

        println!("time taken => {:?}", after - before);
        assert_eq!(1, 2);
    }
}
