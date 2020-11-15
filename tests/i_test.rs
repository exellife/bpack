#[cfg(test)]
mod test {
    use bpack::{pack, unpack};
    use rand::prelude::*;
    use std::time::{Duration, Instant};

    #[test]
    fn see_if_works() {
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        let stop = now + Duration::from_secs(60 * 60);

        loop {
            if Instant::now() >= stop {
                break;
            }

            let n: u16 = rng.gen_range(1500, u16::MAX);
            // let n: u16 = u16::MAX;
            let mut v_bts: Vec<u8> = vec![0; n as usize];
            for i in 0..v_bts.len() {
                let q = rng.gen_range(32, 127);
                v_bts[i] = q;
            }
            // for i in v_bts.iter() {
            //     println!("item in src -> {}", i);
            // }
            let before = Instant::now();
            if let Some(data) = pack(&v_bts) {
                assert_eq!(unpack(data), v_bts);
            }
            let after = Instant::now();
            println!(
                "it took {:?} to pack and unpack {} size input",
                after - before,
                n
            );
        }
        // let n: u16 = rng.gen_range(2500, u16::MAX);
        // let n: u16 = u16::MAX;
        // let mut v_bts: Vec<u8> = vec![0; n as usize];
        // for i in 0..v_bts.len() {
        //     let q = rng.gen_range(32, 127);
        //     v_bts[i] = q;
        // }
        // // for i in v_bts.iter() {
        // //     println!("item in src -> {}", i);
        // // }
        // let before = Instant::now();
        // if let Some(data) = pack(&v_bts) {
        //     assert_eq!(v_bts, unpack(data));
        // }
        // let after = Instant::now();
        // println!("it took {:?} to pack {} size input", after - before, n);
        // assert_eq!(1, 2);
    }
}
