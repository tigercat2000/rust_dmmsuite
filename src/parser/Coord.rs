use crate::parser::DMM::Rule;
use pest::iterators::Pair;
use std::ops::Add;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coord(pub u32, pub u32, pub u32);

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

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
