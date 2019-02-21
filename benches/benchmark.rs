extern crate criterion;
extern crate mtt;
extern crate rand;


use std::collections::HashMap;

use criterion::Criterion;

use mtt::TreeWrapper;
use mtt::VmtWrapper;

static STEP_SIZE: usize = 500;

fn benchmark_constructor(c: &mut Criterion, counts: Vec<usize>, test_data: Vec<Vec<String>>,
                         wrapper_map: &HashMap<String, &'static Fn(&Vec<String>) -> VmtWrapper>) {
    for (key, tree_factory) in wrapper_map {
        let data: Vec<Vec<String>> = test_data.clone();
        let mut title = key.clone();
        title.push_str(" Creation");
        let factory = *tree_factory;
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| factory(&data[*i / STEP_SIZE - 1]));
                                     },
                                     counts.clone(),
        );
    }
}

fn benchmark_method(c: &mut Criterion, counts: Vec<usize>, test_data: Vec<Vec<String>>,
                    wrapper_map: &HashMap<String, &'static Fn(&Vec<String>) -> VmtWrapper>) {
    for (key, tree_factory) in wrapper_map {
        let data: Vec<Vec<String>> = test_data.clone();
        let mut title = key.clone();
        title.push_str(" Find");
        let mut trees: Vec<VmtWrapper> = vec![];
        for d in &data {
            trees.push(tree_factory(d));
        }
        c.bench_function_over_inputs(title.as_str(),
                                     move |b, i| {
                                         b.iter(|| {
                                             let index = *i / STEP_SIZE - 1;
                                             let item_index = rand::random::<usize>() % data[index].len();
                                             trees[index].find(&data[index][item_index]);
                                         });
                                     },
                                     counts.clone(),
        );
    }
}

fn generate_blocks(counts: &Vec<usize>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    for c in counts {
        let mut block: Vec<String> = vec![];
        for _i in 0..*c {
            let mut rand_str = String::new();
            for _ in 0..10 {
                rand_str.push(rand::random::<u8>() as char);
            }
            block.push(rand_str);
        }
        result.push(block);
    }
    result
}

pub fn creation_benches(wrapper_map: &HashMap<String, &'static Fn(&Vec<String>) -> VmtWrapper>) {
    let (counts, test_data, mut criterion) = prepare_test();
    benchmark_constructor(&mut criterion, counts, test_data, wrapper_map);
}

pub fn find_benches(wrapper_map: &HashMap<String, &'static Fn(&Vec<String>) -> VmtWrapper>) {
    let (counts, test_data, mut criterion) = prepare_test();
    benchmark_method(&mut criterion, counts, test_data, wrapper_map);
}

fn prepare_test() -> (Vec<usize>, Vec<Vec<String>>, Criterion) {
    let counts: Vec<usize> = (1..7).map(|i| i * STEP_SIZE).collect();
    let test_data: Vec<Vec<String>> = generate_blocks(&counts);
    let criterion: ::criterion::Criterion =
        ::criterion::Criterion::default().configure_from_args();
    (counts, test_data, criterion)
}


fn main() {
    let mut wrapper_map: HashMap<String, &'static Fn(&Vec<String>) -> VmtWrapper> = HashMap::new();
    wrapper_map.insert(String::from("Vector Merkle Tree"), &VmtWrapper::create_tree);
    wrapper_map.insert(String::from("Vector Merkle Tree With Map"), &VmtWrapper::create_tree_with_map);

    find_benches(&wrapper_map);
    creation_benches(&wrapper_map);
    ::criterion::Criterion::default().configure_from_args().final_summary();
}
