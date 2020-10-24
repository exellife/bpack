use crate::{get_bit, set_bit};

#[derive(Debug)]
pub struct Topology {
    vector: Vec<u8>, // TODO set vector to fixed size(119)
    v_idx: usize,
    bit_idx: u8, // 7 to 0 from right to left
}

impl Topology {
    pub fn new() -> Self {
        Topology {
            vector: vec![0; 119],
            v_idx: 0,
            // vector: vec![0],
            bit_idx: 7,
        }
    }

    pub fn topo(&self) -> &Vec<u8> {
        &self.vector
    }

    pub fn write(&mut self, value: Option<u8>) {
        let v = self.vector.clone();
        let mut current = &mut self.vector[self.v_idx];
        // leaf found, set bit to 1, followed by code
        if let Some(mut code) = value {
            // code += 32;
            // set bit to 1 at bit_idx
            *current |= 1 << self.bit_idx;

            // update bit_idx
            if self.bit_idx == 0 {
                // last index in the byte we just wrote to

                // if at this point bit_idx is 0
                // then next entire byte is a code
                self.vector[self.v_idx + 1] = code;

                self.bit_idx = 7;
                self.v_idx += 2;
            } else {
                // self bit index is not 0
                self.bit_idx -= 1;

                // now write part of the "code" after the 1 we just wrote

                // how many bits we can insert into current byte?
                // if bit_idx points to some x number from 7..0
                // say bit_idx = 2
                // it means that we can insert 3 bits
                // 0b0000_0(0)00 <- bit_idx points to 3d 0
                // which means that we should take 3 first bits
                // of the code and make them last 3 bits of the
                // current byte
                // and then take last 5 bits of the code
                // and make them first 5 bits of the new current byte
                // 0b0101_1000 bit_idx is not changing

                let first_bits = code >> (7 - self.bit_idx);
                *current |= first_bits;
                self.v_idx += 1;
                current = &mut self.vector[self.v_idx];
                // now take remaining bits and make them first bits of the current
                let to_shift = 8 - (7 - self.bit_idx);
                let last_bits = (code & (0xff >> to_shift)) << to_shift;

                *current |= last_bits;
                assert!(code <= 95);
                assert_eq!(
                    first_bits << (7 - self.bit_idx) | last_bits >> to_shift,
                    code
                );
            }
        } else {
            // set bit to 0 at bit_idx
            *current &= !(1 << self.bit_idx);

            if self.bit_idx == 0 {
                self.bit_idx = 7;
                self.v_idx += 1;
            } else {
                self.bit_idx -= 1;
            }
        }
    }

    pub fn read(&self) -> Vec<u8> {
        let mut codes = vec![];

        let mut entire = false;

        let mut code = 0b0000_0000;

        let mut to_take = 0;
        let mut end = 8;
        for byte in self.vector.iter() {
            if entire {
                codes.push(*byte);
                entire = false;
                end = 8;
                to_take = 0;
                continue;
            }

            if to_take != 0 {
                let mut bit_idx = 7;

                for i in (0..to_take).rev() {
                    let bit = get_bit(*byte, bit_idx);
                    code = set_bit(code, i, bit);
                    bit_idx -= 1;
                }
                codes.push(code);
                to_take = 0;
                code = 0b0000_0000;
                end = bit_idx + 1;
            }

            for i in (0..end).rev() {
                let bit = get_bit(*byte, i);

                // found 1 at i position
                // 0000_0000
                // 7654 3210
                if bit == 1 {
                    if i == 0 {
                        // if this is a last bit in byte
                        // next entire byte is a code
                        entire = true;
                        break;
                    } else {
                        // next i bits are part of the code
                        let mut bit_idx = 7;
                        for j in (0..i).rev() {
                            let bit = get_bit(*byte, j);
                            code = set_bit(code, bit_idx, bit);
                            bit_idx -= 1;
                        }
                        to_take = 8 - i;
                        break;
                    }
                }
            }
            end = 8;
        }

        codes
    }
}

// let l = self.vector.len() - 1;
// let mut current_byte = self.vector[l];

// // leaf found, set bit to 1, followed by code
// if let Some(code) = value {
//     current_byte = set_bit(current_byte, self.bit_idx, 1);
//     // update byte in vectore (reset)
//     self.vector[l] = current_byte;

//     // last index in the byte we just wrote to
//     if self.bit_idx == 0 {
//         // the next entire byte is a code
//         self.vector.push(code);
//         // after that push another 0
//         // so we can write to it
//         self.vector.push(0);

//         // set bit index to 7
//         // starting point
//         self.bit_idx = 7;
//         return;
//     } else {
//         self.bit_idx -= 1;
//     }

//     // 0000 0000
//     // 7654 3210
//     // remaining empty bits in current byte
//     let bits_left = self.bit_idx + 1;
//     // println!("current_byte -> {:b}", current_byte);
//     // fill them, take first remaining bits of the code
//     // and set them as the last bits of the current byte
//     let to_set = code >> 8 - bits_left;

//     // println!("to_set -> {:b}", to_set);
//     current_byte |= to_set;
//     self.vector[l] = current_byte;

//     // take last bits if the code
//     // and set them as the first bits
//     let mut next_byte = 0b0000_0000;
//     let mut bit_idx = 7;
//     for i in (0..(8 - bits_left)).rev() {
//         let bit = get_bit(code, i);
//         next_byte = set_bit(next_byte, bit_idx, bit);
//         bit_idx -= 1;
//     }

//     self.vector.push(next_byte);
//     self.bit_idx = bit_idx;
// } else {
//     current_byte = set_bit(current_byte, self.bit_idx, 0);
//     self.vector[l] = current_byte;
//     if self.bit_idx == 0 {
//         // last index we just wrote to
//         self.vector.push(0);

//         // reset
//         self.bit_idx = 7;
//     } else {
//         self.bit_idx -= 1;
//     }
// }
