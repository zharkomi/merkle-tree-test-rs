extern crate criterion;
extern crate merkle;
extern crate rand;
extern crate ring;

use criterion::Criterion;
use self::merkle::MerkleTree;
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
        unimplemented!()
    }

    fn validate(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        unimplemented!()
    }
}