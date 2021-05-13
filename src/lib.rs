#![allow(non_snake_case)]

#[cfg(not(target_pointer_width = "32"))]
compile_error!("rust_dmmsuite must be compiled for a 32-bit target");

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

pub use parser::{Coords as CoordsMod, Prefab as PrefabMod, DMM as DMMMod};
pub use CoordsMod::Coords;
pub use DMMMod::{DMMParser, Rule, DMM};
pub use PrefabMod::Prefab;

#[cfg(test)]
mod tests;
