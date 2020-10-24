use crate::pack::Node;
use std::collections::BinaryHeap;

pub fn count_freq(src: &[u8]) -> BinaryHeap<Node> {
    let mut t: Vec<Node> = [0; 95]
        .iter()
        .enumerate()
        .map(|x| Node::new_leaf(x.0 as u8))
        .collect();

    for i in src.iter() {
        t[(*i - 32) as usize].weight += 1;
    }

    t.iter()
        .filter(|x| x.weight > 0)
        .map(|node| node.clone())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use std::time::{Duration, Instant};

    #[test]
    #[ignore]
    fn count_freq_test_() {
        let mut rng = rand::thread_rng();
        let mut bts: [u8; 1024 * 1024] = [0; 1024 * 1024];

        // rng.fill_bytes(&mut bts);
        for i in 0..bts.len() {
            bts[i] = rng.gen_range(32, 127);
        }

        let start = Instant::now();
        count_freq(&bts);
        let stop = Instant::now();

        println!("it took {:?} to count 1mb", stop - start);

        assert_eq!(1, 2);
    }
}

// let mut t: Vec<Node> = vec![
//     Node::new_leaf(0),
//     Node::new_leaf(1),
//     Node::new_leaf(2),
//     Node::new_leaf(3),
//     Node::new_leaf(4),
//     Node::new_leaf(5),
//     Node::new_leaf(6),
//     Node::new_leaf(7),
//     Node::new_leaf(8),
//     Node::new_leaf(9),
//     Node::new_leaf(10),
//     Node::new_leaf(11),
//     Node::new_leaf(12),
//     Node::new_leaf(13),
//     Node::new_leaf(14),
//     Node::new_leaf(15),
//     Node::new_leaf(16),
//     Node::new_leaf(17),
//     Node::new_leaf(18),
//     Node::new_leaf(19),
//     Node::new_leaf(20),
//     Node::new_leaf(21),
//     Node::new_leaf(22),
//     Node::new_leaf(23),
//     Node::new_leaf(24),
//     Node::new_leaf(25),
//     Node::new_leaf(26),
//     Node::new_leaf(27),
//     Node::new_leaf(28),
//     Node::new_leaf(29),
//     Node::new_leaf(30),
//     Node::new_leaf(31),
//     Node::new_leaf(32),
//     Node::new_leaf(33),
//     Node::new_leaf(34),
//     Node::new_leaf(35),
//     Node::new_leaf(36),
//     Node::new_leaf(37),
//     Node::new_leaf(38),
//     Node::new_leaf(39),
//     Node::new_leaf(40),
//     Node::new_leaf(41),
//     Node::new_leaf(42),
//     Node::new_leaf(43),
//     Node::new_leaf(44),
//     Node::new_leaf(45),
//     Node::new_leaf(46),
//     Node::new_leaf(47),
//     Node::new_leaf(48),
//     Node::new_leaf(49),
//     Node::new_leaf(50),
//     Node::new_leaf(51),
//     Node::new_leaf(52),
//     Node::new_leaf(53),
//     Node::new_leaf(54),
//     Node::new_leaf(55),
//     Node::new_leaf(56),
//     Node::new_leaf(57),
//     Node::new_leaf(58),
//     Node::new_leaf(59),
//     Node::new_leaf(60),
//     Node::new_leaf(61),
//     Node::new_leaf(62),
//     Node::new_leaf(63),
//     Node::new_leaf(64),
//     Node::new_leaf(65),
//     Node::new_leaf(66),
//     Node::new_leaf(67),
//     Node::new_leaf(68),
//     Node::new_leaf(69),
//     Node::new_leaf(70),
//     Node::new_leaf(71),
//     Node::new_leaf(72),
//     Node::new_leaf(73),
//     Node::new_leaf(74),
//     Node::new_leaf(75),
//     Node::new_leaf(76),
//     Node::new_leaf(77),
//     Node::new_leaf(78),
//     Node::new_leaf(79),
//     Node::new_leaf(80),
//     Node::new_leaf(81),
//     Node::new_leaf(82),
//     Node::new_leaf(83),
//     Node::new_leaf(84),
//     Node::new_leaf(85),
//     Node::new_leaf(86),
//     Node::new_leaf(87),
//     Node::new_leaf(88),
//     Node::new_leaf(89),
//     Node::new_leaf(90),
//     Node::new_leaf(91),
//     Node::new_leaf(92),
//     Node::new_leaf(93),
//     Node::new_leaf(94),
//     // Node::new_leaf(95),
// ];
