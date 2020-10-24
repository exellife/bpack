// extern crate bpack;

use bpack;

#[cfg(test)]
mod test {
    use rand::prelude::*;
    use std::time::{Duration, Instant};

    #[test]
    fn see_if_works() {
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        let stop = now + Duration::from_secs(60 * 30);
        loop {
            if Instant::now() >= stop {
                break;
            }

            let n: u16 = rng.gen_range(2500, u16::MAX);
            let mut v_bts: Vec<u8> = vec![0; n as usize];
            for i in 0..v_bts.len() {
                let q = rng.gen_range(32, 127);
                v_bts[i] = q;
            }
            let before = Instant::now();
            if let Some(data) = bpack::pack(&v_bts) {
                assert_eq!(bpack::unpack(data), v_bts);
            }
            let after = Instant::now();
            println!("it took {:?} to pack {} size input", after - before, n);
        }

        assert_eq!(1, 2);
    }
}
