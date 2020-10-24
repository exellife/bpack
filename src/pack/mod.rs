mod count;
mod encode;
mod node;
mod topology;
mod tree;

pub(self) use count::count_freq;
pub(self) use encode::encode_16;
pub(self) use node::Node;
pub(self) use topology::Topology;
pub(self) use tree::Tree;

pub fn pack(src: &[u8]) -> Option<Vec<u8>> {
    // inside pack function
    // 1. count_freq function call -> returns heap
    // 2. Tree::from_heap(&mut heap) ->  build a tree using heap
    // 3. run dfs on tree -> get table and topology
    // 4. encode data using a table, save topology as part of encoded data
    // 5. return packed data

    let mut heap = count_freq(src);
    let tree = Tree::from_heap(&mut heap);
    if let Some((topology, table)) = tree.dfs() {
        let topo_vec = topology.topo();

        let src_len = src.len() as u16;
        let first = ((0xff00 & src_len) >> 8) as u8;
        let second = (0x00ff & src_len) as u8;

        let enc_src = encode_16(table, src);
        let mut result: Vec<u8> = vec![0; enc_src.len() + 121];
        result[0] = first;
        result[1] = second;
        for i in 0..topo_vec.len() {
            result[i + 2] = topo_vec[i];
            if i == topo_vec.len() - 1 {
                println!("last index if topo -> {}", i + 2);
            }
        }

        for i in 0..enc_src.len() {
            result[i + 121] = enc_src[i];
        }
        assert_eq!(0, result[120]);
        //
        // original size -> 2 byte
        // topology itself -> max 119 byte ?
        // packed data -> "x" byte
        // total -> 2 + 119 + "x" byte
        println!("src -> {:?}", src);
        println!("result -> {:?}", result);
        return Some(result);
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    use serde::{Deserialize, Serialize};
    use serde_json;

    use std::time::{Duration, Instant};

    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    #[derive(Deserialize, Serialize, Debug)]
    struct Friend {
        id: u32,
        name: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct MyStruct {
        _id: String,
        index: u32,
        guid: String,
        is_active: bool,
        balance: String,
        picture: String,
        age: u8,
        eye_color: String,
        name: String,
        gender: String,
        company: String,
        email: String,
        phone: String,
        address: String,
        about: String,
        registered: String,
        latitude: f32,
        longitude: f32,
        tags: Vec<String>,
        friends: Vec<Friend>,
        greeting: String,
        favorite_fruit: String,
    }

    #[test]
    // #[ignore]
    fn pack_test_1() {
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        let stop = now + Duration::from_secs(60 * 30);
        loop {
            if Instant::now() >= stop {
                break;
            }

            // let n: u16 = rng.gen();
            // let mut v_bts: Vec<u8> = vec![0; n as usize];
            // for i in 0..v_bts.len() {
            //     let q = rng.gen_range(32, 127);
            //     v_bts[i] = q;
            // }

            let n: u16 = rng.gen_range(2500, u16::MAX);
            let mut v_bts: Vec<u8> = vec![0; n as usize];
            for i in 0..v_bts.len() {
                let q = rng.gen_range(32, 127);
                v_bts[i] = q;
            }
            // let before = Instant::now();
            if let Some(data) = pack(&v_bts) {
                // println!("original data -> {:?}", n);
                // println!("packed data -> {}", data.len());
            }
            // let after = Instant::now();
            // println!("it took {:?} to pack {} size input", after - before, n);
        }

        // let n: u16 = 5500;
        // let mut v_bts: Vec<u8> = vec![0; n as usize];
        // for i in 0..v_bts.len() {
        //     let q = rng.gen_range(32, 127);
        //     v_bts[i] = q;
        // }

        // pack(&v_bts);

        // let mut buf = vec![0; u16::MAX as usize];
        // // let mut buf = String::with_capacity(u16::MAX as usize);
        // let mut f = BufReader::new(File::open("some.json").expect("error opening file"));
        // // let end = f.read(&mut buf).expect("could not read file");

        // let result: Vec<MyStruct> = serde_json::from_reader(f).unwrap();
        // // println!("j -> {:?}", result);

        // for my_struct in result.iter() {
        //     let r = serde_json::to_string(my_struct).unwrap();
        //     // println!("my struct -> {:?}", r);
        //     pack(r.as_bytes());
        //     // for byte in r.as_bytes() {
        //     //     if *byte < 32 || *byte > 126 {
        //     //         println!("byte found -> {}", byte);
        //     //     }
        //     // }
        // }

        assert_eq!(1, 2);
    }
}
