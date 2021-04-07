use super::*;
use Coords::Coords;
use Prefab::Prefab;
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
