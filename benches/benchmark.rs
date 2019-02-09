#[macro_use]
extern crate criterion;
extern crate mtt;
extern crate rand;

use criterion::Criterion;

use mtt::TreeWrapper;
use mtt::VmtWrapper;

static STEP_SIZE: usize = 500;

fn criterion_benchmark(c: &mut Criterion, counts: Vec<usize>, test_data: Vec<Vec<String>>, tree_factory: &'static Fn(&Vec<String>) -> VmtWrapper) {
    c.bench_function_over_inputs("Vector Merkle Tree",
                                 move |b, i| {
                                     b.iter(|| tree_factory(&test_data[*i / STEP_SIZE - 1]));
                                 },
                                 counts,
    );
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

pub fn creation_benches() {
    let counts = (1..7).map(|i| i * STEP_SIZE).collect();
    let test_data: Vec<Vec<String>> = generate_blocks(&counts);
    let mut criterion: ::criterion::Criterion =
        ::criterion::Criterion::default().configure_from_args();
    criterion_benchmark(&mut criterion, counts, test_data, &VmtWrapper::create_tree);
}

//criterion_group!(benches, criterion_benchmark);
criterion_main!(creation_benches);