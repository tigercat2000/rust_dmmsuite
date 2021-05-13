use pest::iterators::Pair;
use pest::Parser;

// Force cargo to rebuild if grammar changes
const _GRAMMAR: &'static str = include_str!("DMM.pest");
#[derive(Parser)]
#[grammar = "parser/DMM.pest"]
pub struct DMMParser;

use crate::parser::Coords::Coords;
use crate::parser::Prefab::Prefab;
use std::collections::HashMap;
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

    pub fn to_json(self) -> Result<String, serde_json::Error> {
        use serde::Serialize;

        #[derive(Serialize)]
        struct PrefabInfo {
            pub paths: Vec<String>,
            pub coordinates: Vec<(u32, u32, u32)>,
        }

        let mut prefabs: HashMap<String, PrefabInfo> = HashMap::new();

        let _ = self.prefabs.iter().for_each(|prefab| {
            let info = PrefabInfo {
                paths: prefab.path_initializers.clone(),
                coordinates: Vec::new(),
            };
            prefabs.insert(prefab.key.clone(), info);
        });

        let mut coords: HashMap<(u32, u32, u32), String> = HashMap::new();

        self.coordinates.iter().for_each(|coord| {
            coord.create_coord_to_key_map(&mut coords);
        });

        for (k, v) in coords.iter() {
            let fab = match prefabs.get_mut(v) {
                Some(x) => x,
                None => panic!(format!("Failed on key: {}", v)),
            };
            fab.coordinates.push(k.clone());
        }

        let prefabs: Vec<&PrefabInfo> = prefabs.values().collect();

        Ok(serde_json::to_string(&prefabs)?)
    }

    pub fn read_map(map: &str) -> Self {
        let parse = DMMParser::parse(Rule::map, map)
            .expect("Failed parse")
            .next()
            .unwrap();
        Self::from_parser(parse)
    }
}
