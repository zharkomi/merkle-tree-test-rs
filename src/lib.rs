extern crate criterion;

use criterion::Criterion;

pub mod vmt_wrapper;
pub mod mtrs_wrapper;

pub const STEP_SIZE: usize = 500;

pub trait TreeWrapper<V> {
    fn create(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<V>>, title: String);
    fn find(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<V>>, title: String);
    fn validate(&self, c: &mut Criterion, counts: Vec<usize>, data: Vec<Vec<V>>, title: String);
}