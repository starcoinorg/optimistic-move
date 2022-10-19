module omo::mips_emulator {
    use omo::memory;
    use trie::trie;
    use StarcoinFramework::Vector;
    use omo::mips;
    use trie::hash_value;

    public fun create(signer: &signer) {
        memory::create(signer);
    }
    public fun add_trie_data(emulator_addr: address, data: vector<u8>) {
        let mem = memory::get_mem(emulator_addr);
        {
            let db = memory::borrow_db_mut(&mut mem);
            trie::add_raw_node(db, data);
        };
        memory::return_mem(mem)
    }
    public fun batch_add_trie_data(emulator_addr: address, data: vector<vector<u8>>) {
        let mem = memory::get_mem(emulator_addr);
        {
            let db = memory::borrow_db_mut(&mut mem);
            let i = Vector::length(&data);
            while (i != 0) {
                trie::add_raw_node(db, Vector::pop_back(&mut data));
                i = i - 1;
            };
        };
        memory::return_mem(mem)
    }

    public fun run(emulator: address, state_root: vector<u8>): vector<u8> {
        let mem = memory::get_mem(emulator);
        let root = hash_value::new(state_root);
        let new_root = mips::step(&mut mem, root);
        memory::return_mem(mem);
        hash_value::to_bytes(new_root)
    }
}