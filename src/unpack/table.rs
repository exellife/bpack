use std::collections::HashMap;

pub struct Table {
    // the length of the smallest code
    smallest: u8,

    // table itself
    map: HashMap<u16, (u8, u8)>,

    masks: Vec<u8>,

    need_to_read: u8,

    rest: u16,
    bits_occupies: u8,
}

impl Table {
    pub fn new() -> Self {
        let masks = vec![
            0b0000_0000, // placeholder
            0b1000_0000, // 1
            0b1100_0000, // 2
            0b1110_0000, // 3
            0b1111_0000, // 4
            0b1111_1000, // 5
            0b1111_1100, // 6
            0b1111_1110, // 7
            0b1111_1111, // 8
        ];

        // let rev_masks = vec![
        //     0b1111_1111,
        //     0b0111_1111,
        //     0b0011_1111,
        //     0b0001_1111,
        //     0b0000_1111,
        //     0b0000_0111,
        //     0b0000_0011,
        //     0b0000_0001,
        // ];

        Table {
            smallest: u8::MAX,
            map: HashMap::with_capacity(95),
            masks,
            need_to_read: u8::MAX,
            rest: 0u16,
            bits_occupies: 0,
        }
    }

    pub fn set(&mut self, code: u8, bits: u16, counter: i8) {
        if (counter as u8) < self.smallest {
            self.smallest = counter as u8;
            self.need_to_read = self.smallest;
        }

        // println!("bits -> {:b}, code -> {}", bits, code);
        // assert_eq!(self.map.contains_key(&bits), false);
        self.map.insert(bits, (code + 32, counter as u8));
    }

    pub fn print_k(&self) {
        for (k, _) in self.map.iter() {
            println!("key -> {:b}", k);
        }
    }

    pub fn handle(&mut self, byte: u8, v: &mut Vec<u8>, v_idx: &mut usize, o_size: usize) {
        if *v_idx < o_size {
            if self.bits_occupies == 0 {
                for i in self.smallest..9 {
                    let c = ((byte & self.masks[i as usize]) as u16) << 8;
                    if let Some((code, length)) = self.map.get(&c) {
                        // verify length of the bits of the code
                        // with bits we took from current byte
                        if i != *length {
                            continue;
                        }
                        // println!("code found -> {}", code);
                        if *v_idx < o_size {
                            v[*v_idx] = *code;
                            *v_idx += 1;
                        }

                        // at this point we know that rest is empty
                        // we need to take rest of the bits
                        // and make them first bits of the rest
                        let bits_left = 8 - i;
                        if bits_left >= self.smallest {
                            panic!("bits_left >= self.smallest");
                        }
                        if bits_left == 0 {
                            // nothing is left
                            self.bits_occupies = 0;
                            self.rest = 0u16;
                        } else {
                            // somethin is left
                            // so take rest bits and make them first bits
                            // of the rest
                            // assert!(bits_left > 0);
                            self.rest = (((byte & (0xff >> i)) << i) as u16) << 8;
                            self.bits_occupies = bits_left;
                            self.need_to_read = self.smallest - bits_left;
                        }
                        return;
                    }
                }
                // at this point we have not found anything
                // so bits_occupies is 0
                // means that rest is empty
                // take all bits of the byte
                // and make them rest
                self.rest = (byte as u16) << 8;
                self.bits_occupies = 8;
                self.need_to_read = 1;
                println!("0 bits occupied and we got here");
            } else {
                // we have somthing in the rest in this case
                for i in self.need_to_read..9 {
                    // we have something in the rest and it
                    // occupies some bits
                    // say 0b(000) 0_0000_0000_0000 it takes 3 bits
                    let mut c = ((byte & self.masks[i as usize]) as u16) << 8;
                    c >>= self.bits_occupies;
                    c |= self.rest;
                    if let Some((code, length)) = self.map.get(&c) {
                        // verify length
                        // we had 3 bits and we also took some bits
                        // from current byte
                        // so 3 + some bits should be the same length
                        // as a key we are looking for
                        if (self.bits_occupies + i) != *length {
                            continue;
                        }
                        // println!("code found -> {}", code);
                        if *v_idx < o_size {
                            v[*v_idx] = *code;
                            *v_idx += 1;
                        }
                        // at this point we found a code
                        // how many bits left?
                        let bits_left = 8 - i;
                        if bits_left >= self.smallest {
                            // panic!("bits_left >= self.smallest");
                            // we got to the point when more bits left
                            // then the smallest possible
                            // if we found a key at idx = 1
                            // so bits left = 7
                            // 0b(0) 000_0000
                            // if we found a key at idx = 2
                            // bits left = 6
                            // 0b(00) 00_0000
                            // 0b0000_00 (00) <- we don't need them
                            // 0b0000_000 (0) <- this also garbage
                            // updated byte, without bits we already took
                            let b = (byte & (0xff >> i)) << i;
                            for j in self.smallest..bits_left + 1 {
                                let c = ((b & self.masks[j as usize]) as u16) << 8;
                                if let Some((code, length)) = self.map.get(&c) {
                                    if j != *length {
                                        continue;
                                    }
                                    // println!("code found -> {}", code);
                                    if *v_idx < o_size {
                                        v[*v_idx] = *code;
                                        *v_idx += 1;
                                    }
                                    let b_l = bits_left - j;
                                    if b_l >= self.smallest {
                                        panic!("bits_left >= self.smallest");
                                    }
                                    if b_l == 0 {
                                        // nothing is left
                                        self.bits_occupies = 0;
                                        self.rest = 0u16;
                                        self.need_to_read = self.smallest;
                                    } else {
                                        // somthin is left
                                        // if previously we found at i=1
                                        // 0b(0) 000_0000
                                        // 7 bits left
                                        // we shifted them
                                        // so we added a 0 to the end
                                        // 0b(0000_000) 0
                                        self.rest = (((b & (0xff >> j)) << j) as u16) << 8;
                                        self.bits_occupies = b_l;
                                        self.need_to_read = self.smallest - b_l;
                                    }
                                    return;
                                }
                            }
                            // at this point we have not found anything
                            // need to set everything
                            self.rest = (b as u16) << 8;
                            self.bits_occupies = bits_left;
                            self.need_to_read = 1;
                            return;
                        }
                        if bits_left == 0 {
                            // nothing is left
                            self.bits_occupies = 0;
                            self.rest = 0u16;
                            self.need_to_read = self.smallest;
                        } else {
                            // assert!(bits_left > 0);
                            self.rest = (((byte & (0xff >> i)) << i) as u16) << 8;
                            self.bits_occupies = bits_left;
                            self.need_to_read = self.smallest - bits_left;
                        }
                        return;
                    }
                }
                // at this point we have not found anything
                // so bits_occupies is not 0
                // means that rest is not empty
                // take all bits and add them to the rest
                // update bits_occupies += 8

                self.rest |= ((byte as u16) << 8) >> self.bits_occupies;
                self.bits_occupies += 8;
                self.need_to_read = 1;
            }
        }
    }

    pub fn handle_(&mut self, byte: u8) {
        // TODO

        // 0b1010_1010 0b1011_1100 0b1010_1111
        // case when we need to take all bits from a byte
        // so if smallest was 6
        // we did not find anything for 6, 7 and now 8
        // we found for 8
        // take all bits from it
        // bits_left = 8 - 8 = 0
        // if bits_left = 0
        // means that reset everything
        // rest = 0u16
        // bits_occupies = 0
        // need_to_read = smallest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table_test() {
        assert_eq!(1, 2);
    }
}
