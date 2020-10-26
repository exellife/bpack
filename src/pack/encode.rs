use crate::{get_bit, set_bit};

pub fn encode(table: Vec<(u8, i8)>, src: &[u8]) -> Vec<u8> {
    let mut v = vec![];

    let mut byte: u8 = 0b0000_0000;
    let mut empty_bits = 8;

    for b in src.iter() {
        // assert!(table[(*b - 32) as usize].1 != -1);
        let (code, bits_takes) = table[(*b - 32) as usize];

        if empty_bits == 0 {
            v.push(byte);
            byte = 0b0000_0000;
            empty_bits = 8;
        }

        let to_shift = 8 - empty_bits;
        let to_set = code >> to_shift;

        byte |= to_set;

        empty_bits -= bits_takes;

        // overflow case
        if empty_bits < 0 {
            // byte is "full"
            v.push(byte);
            // reinit byte
            byte = 0b0000_0000;

            let mut idx = 7;
            let mut copy_code = code;
            // set bits we just added to byte to 0s
            for _ in 0..8 - to_shift {
                copy_code = set_bit(copy_code, idx, 0);
                idx -= 1;
            }
            // shift copy_code to
            copy_code <<= 8 - to_shift;

            byte |= copy_code;

            empty_bits = 8 - (bits_takes - (8 - to_shift));

            if empty_bits == 0 {
                v.push(byte);
                byte = 0b0000_0000;
                empty_bits = 8;
            }
        }
    }
    v.push(byte);
    v
}

pub fn encode_16(table: Vec<(u16, i8)>, src: &[u8]) -> Vec<u8> {
    let mut v_idx = 0;

    let mut v: Vec<u8> = vec![0; src.len()];

    let mut b_idx = 15;
    let mut byte: u16 = 0b0000_0000_0000_0000;

    for b in src.iter() {
        // assert!(table[(*b - 32) as usize].1 != -1);
        let (code, bits_takes) = table[(*b - 32) as usize];

        let to_insert = code >> 15 - b_idx;
        byte |= to_insert;

        b_idx -= bits_takes;

        if b_idx < 0 {
            let overflow = !(b_idx);

            let first = ((byte & 0xff00) >> 8) as u8;
            let second = (byte & 0x00ff) as u8;

            v[v_idx] = first;
            v[v_idx + 1] = second;

            v_idx += 2;
            // case when overflow is 0
            if overflow == 0 {
                byte = 0b0000_0000_0000_0000;
                b_idx = 15;
            } else if overflow > 0 {
                let bits_taken = (bits_takes - overflow) as usize;

                // byte = ((0xffff >> bits_taken) & code) << overflow;
                byte = ((0xffff >> bits_taken) & code) << bits_taken;
                b_idx = 15 - overflow;
            } else {
                // still less then 0
                panic!("overflow is negative!");
            }
        }
    }
    // unsafe {
    //     v.set_len(v_idx);
    // }
    let first = ((byte & 0xff00) >> 8) as u8;
    let second = (byte & 0x00ff) as u8;

    v[v_idx] = first;
    v[v_idx + 1] = second;

    v_idx += 2;
    v.truncate(v_idx);
    v
}

// let b_table = [
//     0b1111_1111_1111_1111,
//     0b0111_1111_1111_1111,
//     0b0011_1111_1111_1111,
//     0b0001_1111_1111_1111,
//     0b0000_1111_1111_1111,
//     0b0000_0111_1111_1111,
//     0b0000_0011_1111_1111,
//     0b0000_0001_1111_1111,
//     0b0000_0000_1111_1111,
//     0b0000_0000_0111_1111,
//     0b0000_0000_0011_1111,
//     0b0000_0000_0001_1111,
//     0b0000_0000_0000_1111,
//     0b0000_0000_0000_0111,
//     0b0000_0000_0000_0011,
//     0b0000_0000_0000_0001,
// ];
// byte = (b_table[bits_taken] & code) << overflow;

// initial state; idx = 15;
// byte = 0b0000_0000_0000_0000
// to insert new bits shift right to 15 - idx and call OR on byte
// inserting 0b1111_110 0_0000_0000 7 bits
// 0b1111_110 0_0000_0000 >> 15 - 15 = 0b1111_110 0_0000_0000
// byte |= 0b1111_110 0_0000_0000 = 0b1111_1100_0000_0000
// idx is now 15 - 7(inserted bits) = 8
// inserting 0b0101_11 00_0000_0000 (6 bits)
// 0b0101_11 00_0000_0000 >> (15 - 8 = 7) =
// = 0b0000_000 0_1011_1 000
// byte |= 0b0000_000 0_1011_1 000 = 0b1111_110 0_1011_1000
// idx is now 8 - 6(inserted bits) = 2;
// inserting 0b1111_011 0_0000_0000 7 bits;
// 0b1111_011 0_0000_0000 >> (15 - 2 = 13) =
// = 0b0000_0000_0000_0 111
// byte |= 0b0000_0000_0000_0 111 = 0b1111_110 0_1011_1 111
// idx is now 2 - 7 = -5 (4 bits overflow)
// means that byte is full
// split byte into 2 8 bit integers, add them to vector (say done)
// reinitilize byte
// byte = 0b0000_0000_0000_0000 still 4 bits from last code are remaining (1_011)
// how do we know? well, bits_taken = bits_takes - overflow
// set first bits_taken number of bits to 0
// call shift left to bits_taken on code
// byte |= code << bits_taken
// idx = 15 - overflow(4) = 11
// byte now = 0b1011_0000_0000_0000
// inserting 0b1110_1110 _0000_0000 (8 bits)
// 0b1110_1110 _0000_0000 >> (15 - idx = 15 - 11 = 4) =
// 0b0000_1110_1110 _0000
// byte |= 0b0000_1110_1110 _0000 = 0b1011_1110_1110 _0000
// idx = 11 - 8 = 3
// inserting 0b0011_111 0_0000_0000 (7 bits)
// 0b0011_111 0_0000_0000 >> (15 - idx = 15 - 3 = 12) =
// = 0b0000_0000_0000_0011 (overflow 111)
// byte |= 0b0000_0000_0000_0011 = 0b1011_1110_1110 _0011
// idx = 3 - 7 = -4 (3 bits overflow)

// split byte into 2 8 bit integers, add them to vector (say done)
// reinitilize byte
// byte = 0b0000_0000_0000_0000
// bits_taken = bits_takes(7) - 3 = 4
// b_table[4] = 0b0000_1111_1111_1111
// set code's bits_taken number bits to 0
// 0b0000_1111_1111_1111
// &
// 0b0011_111 0_0000_0000 =
// 0b0000_1110_0000_0000
// shift left to bits_taken number
// 0b0000_1110_0000_0000 << 4 = 1110_0000_0000_0000
// byte = 1110_0000_0000_0000
// idx = 15 - overflow(3) = 12

// what if idx = idx_prev - bits_takes = -1
// overflow = 0 (0 bits overflow)
// in this case
// byte is full
// split it into 2 8bit integers
// add them to vector
// increase index in vector
// reinitialize byte 0b0000_0000_0000_0000
// set idx to 15; idx = 15
// continue to the next byte in src;
// repeat above steps

// 0b1100_11
// &
// 0b1111_11
// 0b1100_11
fn trash() {
    // how many bits we gonna take from code
    // to complete byte?
    // if current byte has free 2 bits we can fill
    // we should take 2 first bits of the code
    // and set them as last 2 bits of the byte
    // shift right (8 - 2) to set all bits to 0
    // except last 2 bits
    // let to_set = code >> 8 - bits_left;
    // byte |= to_set;

    // v.push(byte);

    // byte = 0b0000_0000;

    // // how many bits we took from code?
    // // bits_left number
    // // if we took 2 bits in previous operation
    // // and code itself takes 7 bits
    // // we need to take 5 last bits
    // // and set them as first 5 bits of new byte
    // let to_set = code << bits_left;
    // byte |= to_set;

    // how many bits left in the byte
    // if we already set 5 bits in the byte
    // 3 more left
    // bits_left = 8 - (bits_takes as u8 - bits_left);
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use std::time::{Duration, Instant};

    #[test]
    #[ignore]
    fn pack_test() {
        let mut rng = rand::thread_rng();
        // let mut bts: [u8; 5] = [70, 102, 59, 107, 36];
        // let mut bts: [u8; 255] = [
        //     70, 102, 59, 107, 36, 85, 125, 113, 32, 100, 50, 44, 125, 113, 70, 120, 41, 100, 66,
        //     67, 45, 99, 89, 118, 94, 103, 94, 66, 44, 115, 95, 121, 108, 80, 48, 116, 123, 40, 105,
        //     71, 119, 40, 56, 45, 97, 40, 62, 33, 62, 115, 100, 43, 77, 88, 55, 52, 43, 95, 124,
        //     105, 78, 50, 39, 116, 120, 60, 55, 126, 70, 111, 46, 36, 118, 80, 91, 67, 97, 67, 41,
        //     55, 66, 126, 121, 126, 98, 71, 72, 49, 53, 122, 111, 124, 100, 48, 97, 33, 55, 48, 118,
        //     70, 42, 114, 121, 110, 53, 84, 99, 119, 54, 97, 32, 65, 39, 53, 67, 67, 79, 122, 116,
        //     52, 46, 73, 73, 65, 64, 69, 76, 101, 45, 91, 70, 47, 125, 91, 96, 59, 37, 46, 83, 124,
        //     70, 59, 48, 40, 55, 111, 41, 83, 37, 114, 43, 117, 98, 82, 116, 125, 63, 101, 112, 115,
        //     103, 89, 107, 93, 122, 76, 124, 36, 94, 66, 123, 114, 113, 33, 63, 55, 33, 125, 55, 61,
        //     99, 115, 36, 116, 37, 83, 82, 50, 62, 109, 91, 70, 73, 87, 114, 99, 112, 41, 74, 67,
        //     117, 121, 46, 43, 80, 119, 112, 101, 126, 94, 78, 89, 34, 79, 77, 53, 64, 68, 83, 125,
        //     100, 40, 86, 94, 69, 118, 115, 92, 66, 99, 123, 67, 66, 52, 115, 81, 96, 83, 110, 33,
        //     36, 51, 114, 82, 113, 89, 84, 36, 114, 96, 43, 76, 69, 88, 62,
        // ];

        // for i in 0..bts.len() {
        //     bts[i] = rng.gen_range(32, 127);
        // }

        // let t = vec![
        //     (138, 7),
        //     (204, 6),
        //     (184, 8),
        //     (0, -1),
        //     (0, 5),
        //     (248, 7),
        //     (0, -1),
        //     (126, 7),
        //     (220, 6),
        //     (192, 6),
        //     (208, 8),
        //     (200, 6),
        //     (180, 7),
        //     (8, 6),
        //     (100, 6),
        //     (185, 8),
        //     (68, 6),
        //     (162, 8),
        //     (236, 7),
        //     (154, 8),
        //     (252, 7),
        //     (168, 6),
        //     (153, 8),
        //     (72, 5),
        //     (109, 8),
        //     (0, -1),
        //     (0, -1),
        //     (12, 6),
        //     (99, 8),
        //     (98, 8),
        //     (112, 6),
        //     (124, 7),
        //     (116, 7),
        //     (30, 7),
        //     (32, 5),
        //     (80, 5),
        //     (163, 8),
        //     (238, 7),
        //     (88, 5),
        //     (110, 7),
        //     (155, 8),
        //     (24, 6),
        //     (209, 8),
        //     (0, -1),
        //     (254, 7),
        //     (196, 7),
        //     (186, 7),
        //     (182, 7),
        //     (250, 7),
        //     (152, 8),
        //     (246, 7),
        //     (212, 6),
        //     (176, 7),
        //     (108, 8),
        //     (199, 8),
        //     (198, 8),
        //     (96, 7),
        //     (132, 6),
        //     (0, -1),
        //     (164, 6),
        //     (179, 8),
        //     (178, 8),
        //     (228, 6),
        //     (118, 7),
        //     (210, 7),
        //     (148, 6),
        //     (136, 7),
        //     (216, 6),
        //     (232, 6),
        //     (64, 6),
        //     (28, 7),
        //     (144, 7),
        //     (0, -1),
        //     (142, 7),
        //     (0, -1),
        //     (140, 7),
        //     (161, 8),
        //     (160, 8),
        //     (146, 7),
        //     (242, 7),
        //     (240, 7),
        //     (188, 6),
        //     (48, 5),
        //     (16, 5),
        //     (224, 6),
        //     (158, 7),
        //     (104, 6),
        //     (44, 6),
        //     (156, 7),
        //     (172, 6),
        //     (40, 6),
        //     (244, 7),
        //     (120, 6),
        //     (56, 5),
        //     (128, 6),
        // ];
        // let stop = Instant::now() + Duration::from_secs(30);
        // loop {
        //     if Instant::now() >= stop {
        //         break;
        //     }

        //     let bts = vec![]

        // }
        // let v = pack(t, &bts);
        // println!("v -> {:?}", v.len());
        // for b in v.iter() {
        //     println!("byte -> {:b}", b);
        // }
        assert_eq!(5, 6);
    }
}

//                          byte
// code -> 0101_1000 (5)    0101_1 000
// code -> 0001_1100 (7)    1110_ 0000
// code -> 0000_1100 (6)    11 10_0011
// code -> 1000_1100 (7)    0 00000 01
// code -> 0000_0000 (5)
// code -> 0110_1100 (8)
// code -> 0011_1000 (5)
// code -> 10111100 (6)
// code -> 10001010 (7)
// code -> 11101000 (6)
// code -> 11101100 (7)
// code -> 10110100 (7)
// code -> 111000 (5)
// code -> 10111100 (6)
// code -> 1011000 (5)
// code -> 10011100 (7)
// code -> 11000000 (6)
// code -> 11101000 (6)
// code -> 100000 (5)
// code -> 1010000 (5)
// code -> 1000 (6)
// code -> 11011000 (6)
// code -> 10000100 (6)
// code -> 1101000 (6)
// code -> 11100100 (6)
// code -> 10010000 (7)
// code -> 11100100 (6)
// code -> 100000 (5)
// code -> 10110100 (7)
// code -> 10000 (5)
// code -> 1110110 (7)
// code -> 10101100 (6)
// code -> 10100001 (8)
// code -> 11111010 (7)
// code -> 1000100 (6)
// code -> 11100000 (6)
// code -> 11110100 (7)
// code -> 11011100 (6)
// code -> 10001110 (7)
// code -> 1101110 (7)
// code -> 101100 (6)
// code -> 11011100 (6)
// code -> 1101101 (8)
// code -> 1000 (6)
// code -> 10010100 (6)
// code -> 11011100 (6)
// code -> 1110000 (6)
// code -> 11001100 (6)
// code -> 1110000 (6)
// code -> 10000 (5)
// code -> 11101000 (6)
// code -> 11001000 (6)
// code -> 11000100 (7)
// code -> 1100000 (7)
// code -> 1001000 (5)
// code -> 11111100 (7)
// code -> 11001000 (6)
// code -> 1110110 (7)
// code -> 1111000 (6)
// code -> 10001110 (7)
// code -> 10111010 (7)
// code -> 11101100 (7)
// code -> 1111110 (7)
// code -> 11100000 (6)
// code -> 10011100 (7)
// code -> 1100011 (8)
// code -> 1001000 (5)
// code -> 10000000 (6)
// code -> 1011000 (5)
// code -> 11110010 (7)
// code -> 1100100 (6)
// code -> 0 (5)
// code -> 1101000 (6)
// code -> 11111010 (7)
// code -> 10100100 (6)
// code -> 1010000 (5)
// code -> 10010100 (6)
// code -> 1010000 (5)
// code -> 11000000 (6)
// code -> 1001000 (5)
// code -> 100000 (5)
// code -> 10000000 (6)
// code -> 10101100 (6)
// code -> 10000000 (6)
// code -> 10001000 (7)
// code -> 1101110 (7)
// code -> 10011011 (8)
// code -> 10100010 (8)
// code -> 10101000 (6)
// code -> 101000 (6)
// code -> 11110010 (7)
// code -> 1111000 (6)
// code -> 11101000 (6)
// code -> 1000100 (6)
// code -> 10010100 (6)
// code -> 11001100 (6)
// code -> 1001000 (5)
// code -> 1000100 (6)
// code -> 1101000 (6)
// code -> 1011000 (5)
// code -> 11010000 (8)
// code -> 110000 (5)
// code -> 10101100 (6)
// code -> 10010010 (7)
// code -> 10101000 (6)
// code -> 10110000 (7)
// code -> 11011000 (6)
// code -> 101100 (6)
// code -> 10011001 (8)
// code -> 10010100 (6)
// code -> 10001010 (7)
// code -> 11110 (7)
// code -> 1111110 (7)
// code -> 10101000 (6)
// code -> 1010000 (5)
// code -> 1010000 (5)
// code -> 10110110 (7)
// code -> 101000 (6)
// code -> 11100000 (6)
// code -> 11111100 (7)
// code -> 1100100 (6)
// code -> 11000 (6)
// code -> 11000 (6)
// code -> 11110 (7)
// code -> 1110100 (7)
// code -> 11101110 (7)
// code -> 11111110 (7)
// code -> 1000000 (6)
// code -> 1000 (6)
// code -> 10100100 (6)
// code -> 1011000 (5)
// code -> 10111001 (8)
// code -> 111000 (5)
// code -> 10100100 (6)
// code -> 11010010 (7)
// code -> 1100 (6)
// code -> 11111000 (7)
// code -> 1100100 (6)
// code -> 11010100 (6)
// code -> 1111000 (6)
// code -> 1011000 (5)
// code -> 1100 (6)
// code -> 1000100 (6)
// code -> 11011100 (6)
// code -> 1001000 (5)
// code -> 11110010 (7)
// code -> 11000000 (6)
// code -> 11010100 (6)
// code -> 11111000 (7)
// code -> 110000 (5)
// code -> 11001000 (6)
// code -> 10011110 (7)
// code -> 10001000 (7)
// code -> 11110110 (7)
// code -> 11100000 (6)
// code -> 111000 (5)
// code -> 1111100 (7)
// code -> 1000000 (6)
// code -> 11110000 (7)
// code -> 10000 (5)
// code -> 10010000 (7)
// code -> 10000100 (6)
// code -> 10001100 (7)
// code -> 10110010 (8)
// code -> 101000 (6)
// code -> 11111110 (7)
// code -> 1111000 (6)
// code -> 0 (5)
// code -> 11100100 (6)
// code -> 100000 (5)
// code -> 11110100 (7)
// code -> 110000 (5)
// code -> 10111100 (6)
// code -> 11001100 (6)
// code -> 1111100 (7)
// code -> 1001000 (5)
// code -> 11001100 (6)
// code -> 111000 (5)
// code -> 1001000 (5)
// code -> 1100010 (8)
// code -> 11011000 (6)
// code -> 10000 (5)
// code -> 0 (5)
// code -> 11100000 (6)
// code -> 11111000 (7)
// code -> 11010100 (6)
// code -> 11110110 (7)
// code -> 11101100 (7)
// code -> 1110000 (6)
// code -> 10100000 (8)
// code -> 10100100 (6)
// code -> 1011000 (5)
// code -> 11000 (6)
// code -> 11000110 (8)
// code -> 110000 (5)
// code -> 11011000 (6)
// code -> 11110000 (7)
// code -> 11000000 (6)
// code -> 11010001 (8)
// code -> 1010000 (5)
// code -> 10011110 (7)
// code -> 10101100 (6)
// code -> 1100100 (6)
// code -> 11001000 (6)
// code -> 11111010 (7)
// code -> 101100 (6)
// code -> 11110000 (7)
// code -> 1000000 (6)
// code -> 10000000 (6)
// code -> 11100100 (6)
// code -> 10111010 (7)
// code -> 10000100 (6)
// code -> 10111000 (8)
// code -> 10110110 (7)
// code -> 11000100 (7)
// code -> 10101000 (6)
// code -> 1110100 (7)
// code -> 10100011 (8)
// code -> 11010100 (6)
// code -> 111000 (5)
// code -> 11101000 (6)
// code -> 11011100 (6)
// code -> 11000111 (8)
// code -> 11100100 (6)
// code -> 11101110 (7)
// code -> 1101000 (6)
// code -> 10000 (5)
// code -> 10110011 (8)
// code -> 100000 (5)
// code -> 11011000 (6)
// code -> 11110100 (7)
// code -> 1010000 (5)
// code -> 100000 (5)
// code -> 11111100 (7)
// code -> 10000 (5)
// code -> 10011000 (8)
// code -> 11010010 (7)
// code -> 11010100 (6)
// code -> 10010010 (7)
// code -> 11001100 (6)
// code -> 0 (5)
// code -> 10011010 (8)
// code -> 110000 (5)
// code -> 11110110 (7)
// code -> 10111100 (6)
// code -> 10000100 (6)
// code -> 10110000 (7)
// code -> 0 (5)
// code -> 110000 (5)
// code -> 11010010 (7)
// code -> 11001000 (6)
// code -> 11111110 (7)
// code -> 11101110 (7)
// code -> 1100000 (7)
// code -> 1110000 (6)
