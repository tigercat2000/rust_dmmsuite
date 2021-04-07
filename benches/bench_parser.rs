use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pest::Parser;
#[macro_use]
extern crate pest;
use rust_dmmsuite::{DMMParser, Rule, DMM};

pub fn parse_metastation() {
    let map = include_str!("../src/dmm_tests/MetaStation.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let _ = DMM::DMM::from_parser(parse);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse metastation", |b| b.iter(|| parse_metastation()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
