fn mask(n: u8) -> u8 {
    1 << n
}

fn mask_16(n: u16) -> u16 {
    1 << n
}

pub fn set_bit(n: u8, pos: u8, val: u8) -> u8 {
    if val == 1 {
        return n | mask(pos);
    }

    n & !mask(pos)
}

pub fn set_bit_16(n: u16, pos: u16, val: u8) -> u16 {
    if val == 1 {
        return n | mask_16(pos);
    }

    n & !mask_16(pos)
}

pub fn get_bit(n: u8, pos: u8) -> u8 {
    if n & mask(pos) == 0 {
        return 0;
    } else {
        return 1;
    }
}

// pub fn get_bit_16(n: u16, pos: u16) -> u16 {
//     if n & mask_16(pos) == 0 {
//         return 0;
//     } else {
//         return 1;
//     }
// }

// pub fn read(src: &[u8]) -> Vec<u8> {
//     let mut codes = vec![];

//     let mut entire = false;

//     let mut code = 0b0000_0000;

//     let mut to_take = 0;
//     let mut end = 8;

//     for byte in src.iter() {
//         if entire {
//             codes.push(*byte);
//             entire = false;
//             end = 8;
//             to_take = 0;
//             println!("entire code -> {}", *byte as char);
//             continue;
//         }

//         if to_take != 0 {
//             let mut bit_idx = 7;

//             for i in (0..to_take).rev() {
//                 let bit = get_bit(*byte, bit_idx);
//                 code = set_bit(code, i, bit);
//                 bit_idx -= 1;
//             }
//             println!("code -> {}", code as char);
//             to_take = 0;
//             code = 0b0000_0000;
//             end = bit_idx + 1;
//         }

//         for i in (0..end).rev() {
//             let bit = get_bit(*byte, i);

//             // found 1 at i position
//             // 0000_0000
//             // 7654 3210
//             if bit == 1 {
//                 if i == 0 {
//                     // if this is a last bit in byte
//                     // next entire byte is a code
//                     println!("got here");
//                     entire = true;
//                     break;
//                 } else {
//                     // next i bits are part of the code
//                     let mut bit_idx = 7;
//                     for j in (0..i).rev() {
//                         let bit = get_bit(*byte, j);
//                         code = set_bit(code, bit_idx, bit);
//                         bit_idx -= 1;
//                     }
//                     to_take = 8 - i;
//                     break;
//                 }
//             }
//         }
//         end = 8;
//     }

//     codes
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use rand::prelude::*;
//     use std::convert::TryInto;
//     use std::time::{Duration, Instant};

//     #[test]
//     // #[ignore]
//     fn set_bit_test() {
//         let mut rng = rand::thread_rng();
//         let stop = Instant::now() + Duration::from_secs(30);
//         // loop {
//         //     if Instant::now() >= stop {
//         //         break;
//         //     }

//         //     let n: u16 = rng.gen();
//         //     let first: u8 = ((n & 0xff00) >> 8).try_into().unwrap();
//         //     let second: u8 = (n & 0x00ff).try_into().unwrap();

//         //     let first_u8 = ((n & 0xff00) >> 8) as u8;
//         //     let second_u8 = (n & 0x00ff) as u8;

//         //     assert_eq!(first, first_u8);
//         //     assert_eq!(second, second_u8);
//         // }
//         // let byte = 0b0000_1011_0101_0000;
//         // let first: u8 = ((byte & 0xff00) >> 8).try_into().unwrap();
//         // let second: u8 = (byte & 0x00ff).try_into().unwrap();
//         let b = 0b0000_1110_0000_0000;
//         println!(
//             "b -> {}, shifted -> {:b}, result -> {}",
//             b,
//             b << 4,
//             b >> 15 - 10
//         );
//         assert_eq!(1, 2);
//     }

//     #[test]
//     #[ignore]
//     fn read_test() {
//         let src = [
//             0b1011_0000,
//             0b1010_1100,
//             0b0100_0001,
//             0b0110_0011,
//             0b0000_0000,
//             0b0000_1011,
//             0b0101_1000,
//         ];
//         read(&src);
//         assert_eq!(1, 2);
//     }
// }
