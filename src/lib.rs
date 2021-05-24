#![allow(non_snake_case)]

#[cfg(not(target_pointer_width = "32"))]
compile_error!("rust_dmmsuite must be compiled for a 32-bit target");

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

pub use parser::{
    Coord as CoordMod, CoordBlock as CoordsBlockMod, Prefab as PrefabMod, DMM as DMMMod,
};
pub use CoordMod::Coord;
pub use CoordsBlockMod::CoordBlock;
pub use DMMMod::{DMMParser, Rule, DMM};
pub use PrefabMod::Prefab;

pub mod byond;

#[cfg(test)]
mod tests;
