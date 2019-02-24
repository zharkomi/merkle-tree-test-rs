extern crate criterion;
extern crate rand;
extern crate ring;
extern crate vmt;

use criterion::Criterion;
use self::ring::digest::{Algorithm, SHA512};
use self::vmt::MerkleTree;

static ALGO: &'static Algorithm = &SHA512;

pub struct VmtWrapper {
    use_map: bool,
}

impl VmtWrapper {
    pub fn new(map_flag: bool) -> impl ::TreeWrapper<String> {
        VmtWrapper {
            use_map: map_flag
        }
    }
}

impl ::TreeWrapper<String> for VmtWrapper {
    fn create(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        let flag = self.use_map;
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| MerkleTree::new_with_flag(&data[*i / ::STEP_SIZE - 1], ALGO, flag));
                                     },
                                     counts.clone(),
        );
    }

    fn find(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        let mut trees: Vec<MerkleTree> = vec![];
        for d in &data {
            trees.push(MerkleTree::new_with_flag(&d, ALGO, self.use_map));
        }
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| {
                                             let index = *i / ::STEP_SIZE - 1;
                                             let item_index = rand::random::<usize>() % data[index].len();
                                             trees[index].build_proof(&data[index][item_index]);
                                         });
                                     },
                                     counts.clone(),
        );
    }

    fn validate(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        let mut trees: Vec<MerkleTree> = vec![];
        let mut proofs: Vec<Vec<Vec<u8>>> = vec![];
        for d in &data {
            let tree = MerkleTree::new_with_flag(&d, ALGO, self.use_map);
            let mut tree_proofs: Vec<Vec<u8>> = vec![];
            for v in d {
                let proof_ref = tree.build_proof(v).unwrap();
                let mut proof: Vec<u8> = vec![];
                for p in proof_ref {
                    proof.extend_from_slice(p);
                }
                tree_proofs.push(proof);
            }
            trees.push(tree);
            proofs.push(tree_proofs);
        }
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| {
                                             let index = *i / ::STEP_SIZE - 1;
                                             let item_index = rand::random::<usize>() % data[index].len();
                                             let proof_vec = &proofs[index][item_index];
                                             let mut proof : Vec<&[u8]> = vec![];
                                             let mut pi = 0;
                                             while pi < proof_vec.len() {
                                                 proof.push(&proof_vec[pi..(pi+ALGO.output_len)]);
                                                 pi += ALGO.output_len;
                                             }
                                             assert_eq!(true, trees[index].validate(&data[index][item_index], proof, trees[index].get_root()));
                                         });
                                     },
                                     counts.clone(),
        );
    }
}