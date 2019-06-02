extern crate criterion;
extern crate rand;
extern crate ring;
extern crate vmt;

use criterion::Criterion;

use self::ring::digest::{Algorithm, SHA512};
use self::vmt::MerkleTree;

static ALGO: &'static Algorithm = &SHA512;

pub struct VmtWrapper {
    clone: bool,
    use_map: bool,
}

impl VmtWrapper {
    pub fn new(clone_flag: bool, map_flag: bool) -> impl ::TreeWrapper<String> {
        VmtWrapper {
            use_map: map_flag,
            clone: clone_flag,
        }
    }
}

impl ::TreeWrapper<String> for VmtWrapper {
    fn create(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<String>>, title: String) {
        let flag = self.use_map;
        let clone = self.clone;
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         if clone {
                                             b.iter(|| MerkleTree::new_with_flag(&(data[*i / ::STEP_SIZE - 1].clone()), ALGO, flag));
                                         } else {
                                             b.iter(|| MerkleTree::new_with_flag(&data[*i / ::STEP_SIZE - 1], ALGO, flag));
                                         }
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
        for d in &data {
            let tree = MerkleTree::new_with_flag(&d, ALGO, self.use_map);
            trees.push(tree);
        }
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| {
                                             let index = *i / ::STEP_SIZE - 1;
                                             let item_index = rand::random::<usize>() % data[index].len();
                                             let tree = &trees[index];
                                             let proof_ref = tree.build_proof(&data[index][item_index]).unwrap();
                                             tree.validate(&proof_ref);
                                         });
                                     },
                                     counts.clone(),
        );
    }
}