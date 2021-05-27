use pest::iterators::Pair;
use pest::Parser;

// Force cargo to rebuild if grammar changes
const _GRAMMAR: &'static str = include_str!("DMM.pest");
#[derive(Parser)]
#[grammar = "parser/DMM.pest"]
pub struct DMMParser;

use crate::parser::Coord::Coord;
use crate::parser::Prefab::Prefab;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Clone)]
pub struct DMM {
    prefabs: HashMap<String, Prefab>,
    keysize: usize,
    pub map: BTreeMap<Coord, *const Prefab>,
}

impl DMM {
    pub fn from_parser(map: Pair<Rule>) -> Self {
        let mut new_dmm = Self {
            prefabs: HashMap::new(),
            keysize: 0,
            map: BTreeMap::new(),
        };

        debug_assert_eq!(map.as_rule(), Rule::map);

        let sections = map.into_inner();
        for section in sections {
            match section.as_rule() {
                Rule::prefabs => {
                    new_dmm.prefabs = Prefab::from_parser_array(section);
                    new_dmm.keysize = new_dmm.prefabs.iter().next().unwrap().1.key.len();
                }
                Rule::coordinates => {
                    new_dmm.parse_coordinates(section);
                }
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }

        new_dmm
    }

    pub fn calculate_bounds(&self, offset: Coord) -> Coord {
        let big_boy = self.map.iter().last().unwrap().0;

        *big_boy + (offset - Coord(1, 1, 1))
    }

    // Parses an array of coordblocks into our map.
    fn parse_coordinates(&mut self, section: Pair<Rule>) {
        debug_assert_eq!(section.as_rule(), Rule::coordinates);

        section.into_inner().for_each(|block| {
            debug_assert_eq!(block.as_rule(), Rule::coordblock);
            let mut sections = block.into_inner();
            let offset = sections.next().unwrap();
            let mapblock = sections.next().unwrap();

            debug_assert_eq!(offset.as_rule(), Rule::offset);

            let offset = Coord::from_parser(offset);

            debug_assert_eq!(mapblock.as_rule(), Rule::mapblock);

            let mut insertionKey = offset.clone();
            mapblock
                .as_str()
                .replace("\t", "")
                .replace(" ", "")
                .replace("\r", "")
                .split("\n")
                .for_each(|xline| {
                    let vec: Vec<char> = xline.chars().collect();

                    vec.chunks(self.keysize)
                        .map(|key| -> String { key.iter().collect() })
                        .for_each(|key| {
                            self.map.insert(insertionKey, self.find_prefab(&key));
                            insertionKey.0 += 1;
                        });

                    insertionKey.0 = offset.0;
                    insertionKey.1 += 1;
                    // println!("Next coordinate: {:?}", insertionKey);
                })
        });
    }

    fn find_prefab(&self, key: &str) -> *const Prefab {
        debug_assert_eq!(key.len(), self.keysize);
        self.prefabs.get(key).unwrap()
    }

    pub fn read_map(map: &str) -> Self {
        let parse = DMMParser::parse(Rule::map, map)
            .expect("Failed parse")
            .next()
            .unwrap();
        Self::from_parser(parse)
    }
}
