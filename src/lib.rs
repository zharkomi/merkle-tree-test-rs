extern crate criterion;
extern crate ring;

use ring::digest::{Algorithm, SHA512};

pub trait TreeWrapper<V> {
    fn find(&self, value: &V);
    fn find_and_validate(&self, value: &V);
}

static ALGO: &'static Algorithm = &SHA512;

pub struct VmtWrapper {
    tree: vmt::MerkleTree,
    root: Vec<u8>,
}

impl VmtWrapper {
    pub fn create_tree(data: &Vec<String>) -> VmtWrapper {
        let tree = vmt::MerkleTree::new(data, ALGO);
        let root = tree.get_root().to_vec();
        VmtWrapper { tree: tree, root: root }
    }

    pub fn create_tree_with_map(data: &Vec<String>) -> VmtWrapper {
        let tree = vmt::MerkleTree::new_with_map(data, ALGO);
        let root = tree.get_root().to_vec();
        VmtWrapper { tree: tree, root: root }
    }
}

impl TreeWrapper<String> for VmtWrapper {
    fn find(&self, value: &String) {
        self.tree.build_proof(value);
    }

    fn find_and_validate(&self, value: &String) {
        let result: Option<Vec<&[u8]>> = self.tree.build_proof(value);
        assert_eq!(true, self.tree.validate(value, result.unwrap(), &self.root[..]));
    }
}