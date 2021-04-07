#![allow(non_snake_case)]

#[cfg(not(target_pointer_width = "32"))]
compile_error!("rust_dmmsuite must be compiled for a 32-bit target");

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;

mod Coords;
mod DMM;
mod Prefab;

// Force cargo to rebuild
const _GRAMMAR: &'static str = include_str!("prefab.pest");

#[derive(Parser)]
#[grammar = "prefab.pest"]
pub struct DMMParser;

#[cfg(test)]
mod test;
