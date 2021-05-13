use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pest::Parser;
use rust_dmmsuite::{DMMParser, Rule, DMM};

pub fn parse_tether() {
    let map = include_str!("../src/tests/dmm_files/tether.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let map = DMM::from_parser(parse);
    black_box(map);
}

pub fn parse_metastation() {
    let map = include_str!("../src/tests/dmm_files/MetaStation.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let map = DMM::from_parser(parse);
    black_box(map);
}

pub fn jsonize_metastation() {
    let map = include_str!("../src/tests/dmm_files/MetaStation.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let map = DMM::from_parser(parse);
    let json = map.to_json().unwrap();
    black_box(json);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse metastation", |b| b.iter(|| parse_metastation()));
    c.bench_function("parse tether", |b| b.iter(|| parse_tether()));
    c.bench_function("jsonize metastation", |b| b.iter(|| jsonize_metastation()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
