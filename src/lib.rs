extern crate ring;

use ring::digest::{Algorithm, SHA512};

pub trait TreeWrapper<V> {
    fn find(value: &V);
    fn find_and_validate(value: &V);
}

static ALGO: &'static Algorithm = &SHA512;

pub struct VmtWrapper {
    tree: vmt::MerkleTree<String>
}

impl VmtWrapper {
    pub fn create_tree(data: &Vec<String>) -> VmtWrapper {
        VmtWrapper { tree: vmt::MerkleTree::new(data, ALGO) }
    }
}

impl TreeWrapper<String> for VmtWrapper {
    fn find(value: &String) {
//unimplemented!()
    }

    fn find_and_validate(value: &String) {
//unimplemented!()
    }
}