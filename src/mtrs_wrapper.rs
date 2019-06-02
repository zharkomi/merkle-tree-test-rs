extern crate criterion;
extern crate merkle;
extern crate rand;
extern crate ring;

use criterion::Criterion;

use self::merkle::MerkleTree;
use self::merkle::Proof;
use self::ring::digest::{Algorithm, SHA512};

static ALGO: &'static Algorithm = &SHA512;

pub struct MtrsWrapper {}

impl MtrsWrapper {
    pub fn new() -> impl ::TreeWrapper<String> {
        MtrsWrapper {}
    }
}

impl ::TreeWrapper<String> for MtrsWrapper {
    fn create(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| MerkleTree::from_vec(ALGO, data[*i / ::STEP_SIZE - 1].clone()));
                                     },
                                     counts.clone(),
        );
    }

    fn find(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        let mut trees: Vec<MerkleTree<String>> = vec![];
        for d in &data {
            trees.push(MerkleTree::from_vec(ALGO, d.clone()));
        }
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| {
                                             let index = *i / ::STEP_SIZE - 1;
                                             let item_index = rand::random::<usize>() % data[index].len();
                                             trees[index].gen_proof(data[index][item_index].clone());
                                         });
                                     },
                                     counts.clone(),
        );
    }

    fn validate(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        let mut trees: Vec<MerkleTree<String>> = vec![];
        for d in &data {
            let tree = MerkleTree::from_vec(ALGO, d.clone());
            trees.push(tree);
        }
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| {
                                             let index = *i / ::STEP_SIZE - 1;
                                             let item_index = rand::random::<usize>() % data[index].len();
                                             let tree = &trees[index];
                                             let proof = tree.gen_proof(data[index][item_index].clone()).unwrap();
                                             proof.validate(proof.root_hash.as_ref());
                                         });
                                     },
                                     counts.clone(),
        );
    }
}