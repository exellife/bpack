use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn heap_like() -> HashMap<i32, u32> {
    let mut arr: Vec<u32> = (0..16).rev().collect();

    let mut map: HashMap<i32, u32> = HashMap::new();

    let left = arr.pop().expect("no first item in arr");
    let right = arr.pop().expect("no second item in arr");

    let parent = left + right;

    insert(&mut map, vec![parent, left, right]);
    // insert(&mut map, vec![1, 0, 1]);
    // insert(&mut map, vec![2]);
    // insert(&mut map, vec![3]);
    // insert(&mut map, vec![9, 4, 5]);
    loop {
        if arr.len() == 0 {
            break;
        }

        let next = arr.pop().expect("no next item in arr");

        // if root is smaller then current item in arr
        // merge them

        if map[&0] <= next {
            insert(&mut map, vec![next]);
        } else {
            if arr.len() > 1 {
                let another = arr.pop().expect("no another item in arr");

                let parent = next + another;
                println!(
                    "parent -> {}, next -> {}, another -> {}",
                    parent, next, another
                );
                insert(&mut map, vec![parent, next, another]);
            } else {
                insert(&mut map, vec![next]);
            }
        }
    }
    // for i in map.iter() {
    //     println!("item -> {}", i);
    // }
    println!("arr -> {:?}", map);
    map
}

fn insert(mut map: &mut HashMap<i32, u32>, items: Vec<u32>) {
    assert!(items.len() == 1 || items.len() == 3);

    // left child at 2i + 1
    // right child at 2i + 2
    // parent floor((n-1) / 2)

    // [1, 0, 1]

    // insert 2
    // [3, 1, 2, 0, 1, -1, -1]

    // insert 3
    // [6, 3, 3, 1, 2, -1, -1, 0, 1, -1, -1]

    // insert 4 + 5
    // [15, 6, 9, 3, 3, 4, 5, 1, 2, -1,-1, -1, -1, 0, 1, -1, -1];

    // insert 6 + 7
    // [28, 15, 13, ]

    // take a value at 0
    println!("inserting -> {:?}", items);
    let root = map.entry(0);
    match root {
        Entry::Vacant(_) => {
            // empty "tree"
            map.insert(0, items[0]);
            map.insert(1, items[1]);
            map.insert(2, items[2]);
        }
        // we have a root,
        // rearrange
        Entry::Occupied(entry) => {
            let new_root = map[&0] + items[0];

            // if items length == 1 -> create new root add smaller
            // one to left, bigger one to the right
            if items.len() == 1 {
                // merge current root with item

                rearrange(&mut map, 0);

                map.insert(0, new_root);
                map.insert(2, items[0]);
            }

            // if items length == 3 -> create new root using current root
            // merge it with new root on the left/right depending on
            // which one is bigger
            if items.len() == 3 {
                // merge current root with parent

                rearrange(&mut map, 0);
                rearrange(&mut map, 2);

                map.insert(0, new_root);
                map.insert(2, items[0]);
                map.insert(5, items[1]);
                map.insert(6, items[2]);
            }
        }
    }
}

fn rearrange(mut map: &mut HashMap<i32, u32>, index: i32) {
    let parent_index = index; // 0
    let left_child_index = 2 * index + 1; // 1
    let right_child_index = 2 * index + 2; // 2

    let parent = map.remove(&parent_index);
    let left_n = map.remove(&left_child_index);
    let right_n = map.remove(&right_child_index);

    if let Some(_) = left_n {
        rearrange(&mut map, left_child_index);
        rearrange(&mut map, right_child_index);
    }

    if let Some(p) = parent {
        map.insert(left_child_index, p);
    }

    if let Some(l) = left_n {
        map.insert(left_child_index * 2 + 1, l);
    }

    if let Some(r) = right_n {
        map.insert(left_child_index * 2 + 2, r);
    }

    // [1, 0, 1] -> [-1, 1, -1, 0, 1, -1, -1]
    // let left = map.entry(left_child_index);
    // match left {
    //     Entry::Vacant(_) => {

    //         if let Some(n) = parent {
    //             map.insert(left_child_index, n);
    //         }

    //         if let Some(n) = left_n {
    //             map.insert(left_child_index * 2 + 1, n);
    //         }

    //         if let Some(n) = right_n {
    //             map.insert(left_child_index * 2 + 2, n);
    //         }
    //     }
    //     Entry::Occupied(_) => {
    //         rearrange(&mut map, left_child_index);
    //         rearrange(&mut map, right_child_index);
    //     }
    // }

    // if map[left_child_index] != -1 {}

    // map[parent_index] = -1;
    // map[left_child_index] = -1;
    // map[right_child_index] = -1;

    // map[left_child_index] = temp_parent;
    // map[left_child_index * 2 + 1] = temp_left;
    // map[left_child_index * 2 + 2] = temp_right;

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn heap_like_test() {
        let map = heap_like();

        assert_eq!(1, 2);
    }
}

// 1
// [1, 0, 1]

// 2
// [3, 1, 2, 0, 1]

// 3
// [6, 3, 3, 1, 2, -1, -1, 0, 1]

// 4
// [10, 6, 4, 3, 3, -1, -1, 1, 2, -1, -1, -1, -1, -1, -1, 0, 1, -1, -1 ]

// 4 + 5 = 9
// [15, 6, 9, 3, 3, 4, 5, 1, 2, -1, -1, -1, -1, -1, -1, ]