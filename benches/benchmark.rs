extern crate criterion;
extern crate mtt;
extern crate rand;

use std::collections::HashMap;

use criterion::Criterion;

use mtt::*;
use mtt::mtrs_wrapper::*;
use mtt::vmt_wrapper::*;

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

fn prepare_test() -> (Vec<usize>, Vec<Vec<String>>, Criterion) {
    let counts: Vec<usize> = (1..7).map(|i| i * mtt::STEP_SIZE).collect();
    let test_data: Vec<Vec<String>> = generate_blocks(&counts);
    let criterion: ::criterion::Criterion =
        ::criterion::Criterion::default().configure_from_args();
    (counts, test_data, criterion)
}

pub fn creation_benches(wrapper_map: &HashMap<String, Box<TreeWrapper<String>>>) {
    let (counts, test_data, mut criterion) = prepare_test();
    for (key, tree_wrapper) in wrapper_map {
        let data: Vec<Vec<String>> = test_data.clone();
        let mut title = key.clone();
        title.push_str(" Creation");
        tree_wrapper.create(&mut criterion, counts.clone(), data, title);
    }
}

pub fn find_benches(wrapper_map: &HashMap<String, Box<TreeWrapper<String>>>) {
    let (counts, test_data, mut criterion) = prepare_test();
    for (key, tree_wrapper) in wrapper_map {
        let data: Vec<Vec<String>> = test_data.clone();
        let mut title = key.clone();
        title.push_str(" Find");
        tree_wrapper.find(&mut criterion, counts.clone(), data, title);
    }
}

pub fn validation_benches(wrapper_map: &HashMap<String, Box<TreeWrapper<String>>>) {
    let (counts, test_data, mut criterion) = prepare_test();
    for (key, tree_wrapper) in wrapper_map {
        let data: Vec<Vec<String>> = test_data.clone();
        let mut title = key.clone();
        title.push_str(" Validate");
        tree_wrapper.validate(&mut criterion, counts.clone(), data, title);
    }
}

fn main() {
    let mut wrapper_map: HashMap<String, Box<TreeWrapper<String>>> = HashMap::new();
    wrapper_map.insert(String::from("Vector Merkle Tree"), Box::new(VmtWrapper::new(false, false)));
    wrapper_map.insert(String::from("Vector Merkle Tree With Map"), Box::new(VmtWrapper::new(false, true)));
    //wrapper_map.insert(String::from("Cloned Vector Merkle Tree"), Box::new(VmtWrapper::new(true, false)));
    //wrapper_map.insert(String::from("Cloned Vector Merkle Tree With Map"), Box::new(VmtWrapper::new(true, true)));
    wrapper_map.insert(String::from("Merkle Tree RS"), Box::new(MtrsWrapper::new()));

    find_benches(&wrapper_map);
    creation_benches(&wrapper_map);
    validation_benches(&wrapper_map);
    ::criterion::Criterion::default().configure_from_args().final_summary();
}
