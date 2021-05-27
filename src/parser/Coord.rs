use crate::parser::DMM::Rule;
use derive_more::{Add, Sub};
use pest::iterators::Pair;

#[derive(Add, Sub, Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coord(pub u32, pub u32, pub u32);

impl Coord {
    pub fn from_parser(offset: Pair<Rule>) -> Self {
        debug_assert_eq!(offset.as_rule(), Rule::offset);

        let mut offsets = offset.into_inner();

        let x = offsets.next().unwrap();
        debug_assert_eq!(x.as_rule(), Rule::coord);
        let x: u32 = x.as_str().parse().unwrap();

        let y = offsets.next().unwrap();
        debug_assert_eq!(y.as_rule(), Rule::coord);
        let y: u32 = y.as_str().parse().unwrap();

        let z = offsets.next().unwrap();
        debug_assert_eq!(z.as_rule(), Rule::coord);
        let z: u32 = z.as_str().parse().unwrap();

        Self(x, y, z)
    }
}

#[cfg(test)]
mod coord_tests {
    use super::*;
    #[test]
    fn test_derive_addsub() {
        assert_eq!(Coord(1, 1, 1) + Coord(1, 1, 1), Coord(2, 2, 2));
        assert_eq!(Coord(1, 1, 1) - Coord(1, 1, 1), Coord(0, 0, 0));
    }
}
