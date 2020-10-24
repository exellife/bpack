use bpack::{pack, unpack};

fn main() {
    // let some_data = [u8];

    let result = pack(&some_data);

    // inside pack function
    // 1. count_freq function call -> returns heap
    // 2. Tree::from_heap(&mut heap) ->  build a tree using heap 
    // 3. run dfs on tree -> get table and topology
    // 4. encode data using a table, save topology as part of encoded data
    // 5. return packed data

    let unpacked_result = unpack(&result);

    // inside unpack function
    // 1. get topology, get encoded data -> 
    // 2. build a tree from topology
    // 3. decode encoded data using a tree
    // 4. return decoded data
}
