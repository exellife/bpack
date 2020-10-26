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

