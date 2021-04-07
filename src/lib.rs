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

// Force cargo to rebuild
const _GRAMMAR: &'static str = include_str!("prefab.pest");

#[derive(Parser)]
#[grammar = "prefab.pest"]
pub struct DMMParser;

#[derive(Debug, PartialEq, Clone)]
pub struct DMM {
    pub prefabs: Vec<Prefab>,
    pub coordinates: Vec<Coords>,
}

impl DMM {
    pub fn from_parser(map: Pair<Rule>) -> Self {
        assert_eq!(map.as_rule(), Rule::map);

        let sections = map.into_inner();
        let mut prefabs = Vec::new();
        let mut coordinates = Vec::new();
        let mut keysize = 1;
        for section in sections {
            match section.as_rule() {
                Rule::prefabs => {
                    prefabs = Prefab::from_parser_array(section);
                    keysize = prefabs.last().unwrap().key.len();
                }
                Rule::coordinates => {
                    coordinates = Coords::from_parser_array(section, keysize);
                }
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }

        DMM {
            prefabs,
            coordinates,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Coords {
    pub offset: (u32, u32, u32),
    pub keymap: Vec<String>,
}

impl Coords {
    pub fn from_parser_array(array: Pair<Rule>, keysize: usize) -> Vec<Self> {
        assert_eq!(array.as_rule(), Rule::coordinates);

        let coordblocks = array.into_inner();

        let mut coordinates = Vec::new();
        for coordblock in coordblocks {
            assert_eq!(coordblock.as_rule(), Rule::coordblock);
            coordinates.push(Coords::from_parser(coordblock, keysize));
        }

        coordinates
    }

    pub fn from_parser(coordblock: Pair<Rule>, keysize: usize) -> Self {
        assert_eq!(coordblock.as_rule(), Rule::coordblock);

        let mut sections = coordblock.into_inner();
        let offset = sections.next().unwrap();
        assert_eq!(offset.as_rule(), Rule::offset);
        let mut offsets = offset.into_inner();

        let x = offsets.next().unwrap();
        assert_eq!(x.as_rule(), Rule::coord);
        let x: u32 = x.as_str().parse().unwrap();

        let y = offsets.next().unwrap();
        assert_eq!(y.as_rule(), Rule::coord);
        let y: u32 = y.as_str().parse().unwrap();

        let z = offsets.next().unwrap();
        assert_eq!(z.as_rule(), Rule::coord);
        let z: u32 = z.as_str().parse().unwrap();

        let mapblock = sections.next().unwrap();
        assert_eq!(mapblock.as_rule(), Rule::mapblock);

        let map_to_parse = mapblock
            .as_str()
            .replace("\t", "")
            .replace(" ", "")
            .replace("\n", "");

        Coords {
            offset: (x, y, z),
            keymap: Self::parse_map(&map_to_parse, keysize),
        }
    }

    pub fn parse_map(map: &str, keysize: usize) -> Vec<String> {
        let mut iter = map.chars().peekable();

        let mut vec = Vec::new();
        while iter.peek().is_some() {
            vec.push(iter.by_ref().take(keysize).collect())
        }

        vec
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Prefab {
    pub key: String,
    pub path_initializers: Vec<String>,
}

impl Prefab {
    pub fn build() -> Self {
        Self {
            key: String::new(),
            path_initializers: vec![],
        }
    }

    #[cfg(test)]
    pub fn test_build(key: &str, path_initializers: Vec<&str>) -> Self {
        Self {
            key: key.to_string(),
            path_initializers: path_initializers
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        }
    }

    pub fn from_parser_array(array: Pair<Rule>) -> Vec<Self> {
        assert_eq!(array.as_rule(), Rule::prefabs);

        let mut ret_vec = Vec::new();

        let prefabs = array.into_inner();
        for prefab in prefabs {
            assert_eq!(prefab.as_rule(), Rule::prefab);
            ret_vec.push(Prefab::from_parser(prefab));
        }

        ret_vec
    }

    pub fn from_parser(prefab: Pair<Rule>) -> Self {
        assert_eq!(prefab.as_rule(), Rule::prefab);

        let mut sections = prefab.into_inner();
        let id = sections.next().unwrap();
        assert_eq!(id.as_rule(), Rule::id);
        let paths = sections.next().unwrap();
        assert_eq!(paths.as_rule(), Rule::paths);

        let mut new_self = Self {
            key: id.as_str().to_string(),
            path_initializers: Vec::new(),
        };

        new_self.take_paths(paths);

        new_self
    }

    pub fn take_paths(&mut self, pair: Pair<Rule>) {
        assert_eq!(pair.as_rule(), Rule::paths);
        let paths = pair.into_inner();

        for path in paths {
            assert_eq!(path.as_rule(), Rule::path);
            self.path_initializers.push(path.as_str().to_string());
        }
    }
}

#[cfg(test)]
mod test {
    use pest::parses_to;

    use super::*;
    #[test]
    fn parse_basic_prefab() {
        let prefab = r#""aa" = (/turf/icon/white,/area/debug)"#;

        let prefab = DMMParser::parse(Rule::prefab, &prefab)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let mut our_prefab = Prefab::build();

        for pair in prefab.into_inner() {
            match pair.as_rule() {
                Rule::id => our_prefab.key = pair.as_str().to_string(),
                Rule::paths => our_prefab.take_paths(pair),
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        assert_eq!(
            our_prefab,
            Prefab::test_build("aa", vec!["/turf/icon/white", "/area/debug"])
        )
    }

    #[test]
    fn parse_initialized_prefab() {
        let prefab = r#""al" = (/turf/icon/white/green/corner{tag = "icon-whitegreencorner (EAST)"; icon_state = "whitegreencorner"; dir = 4},/area/debug)"#;
        let prefab = DMMParser::parse(Rule::prefab, &prefab)
            .expect("Parsing failed")
            .next()
            .unwrap();

        let mut our_prefab = Prefab::build();
        for pair in prefab.into_inner() {
            match pair.as_rule() {
                Rule::id => our_prefab.key = pair.as_str().to_string(),
                Rule::paths => our_prefab.take_paths(pair),
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        assert_eq!(
            our_prefab,
            Prefab::test_build(
                "al",
                vec![
                    r#"/turf/icon/white/green/corner{tag = "icon-whitegreencorner (EAST)"; icon_state = "whitegreencorner"; dir = 4}"#,
                    r#"/area/debug"#
                ]
            )
        )
    }

    #[test]
    fn parse_tgm_prefab() {
        let prefab = r#""aab" = (
    /obj/structure/sign/warning/bomb_range{
        name = "\improper MINING AREA - WATCH FOR BLASTING"
    },
    /turf/unsimulated/wall/planetary/virgo3b,
    /area/tether/surfacebase/outside/outside1)"#;

        let prefab = DMMParser::parse(Rule::prefab, &prefab)
            .expect("Parsing failed")
            .next()
            .unwrap();
        let mut our_prefab = Prefab::build();
        for pair in prefab.into_inner() {
            match pair.as_rule() {
                Rule::id => our_prefab.key = pair.as_str().to_string(),
                Rule::paths => our_prefab.take_paths(pair),
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        assert_eq!(
            our_prefab,
            Prefab::test_build(
                "aab",
                vec![
                    "/obj/structure/sign/warning/bomb_range{\n        name = \"\\improper MINING AREA - WATCH FOR BLASTING\"\n    }",
                    r#"/turf/unsimulated/wall/planetary/virgo3b"#,
                    r#"/area/tether/surfacebase/outside/outside1"#
                ]
            )
        )
    }

    #[test]
    fn parse_offset() {
        parses_to! {
            parser: DMMParser,
            input: "(1,1,1)",
            rule: Rule::offset,
            tokens: [
                offset(0, 7, [
                    coord(1,2),
                    coord(3,4),
                    coord(5,6)
                ])
            ]
        }
    }

    #[test]
    fn parse_map() {
        let dmm = r#""aaa" = (/turf,/area)
"bbb" = (/turf,/area)

(1,1,1) = {"
    aaabbbaaa
    bbbaaabbb
    bbbbbbbbb
"}"#;
        let parse = DMMParser::parse(Rule::map, &dmm)
            .expect("Failed parse")
            .next()
            .unwrap();

        let map = DMM::from_parser(parse);
        // println!("{:#?}", map);
    }

    #[test]
    fn parse_tgm_map() {
        let tgm = r#""a" = (
    /turf,
    /area
)
"b" = (
    /turf
    /area
)

(1,1,1) = {"
    a
    a
    b
    a
    a
"}

(2,1,1) = {"
    a
    b
    b
    a
    b
"}"#;

        let parse = DMMParser::parse(Rule::map, &tgm)
            .expect("Failed parse")
            .next()
            .unwrap();

        let map = DMM::from_parser(parse);
        // println!("{:#?}", map);
    }
}
