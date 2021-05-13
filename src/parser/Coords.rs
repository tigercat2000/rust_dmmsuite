use crate::parser::DMM::Rule;
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Coords {
    pub offset: (u32, u32, u32),
    pub keymap: Vec<String>,
}

impl Coords {
    pub fn from_parser_array(array: Pair<Rule>, keysize: usize) -> Vec<Self> {
        #[cfg(test)]
        assert_eq!(array.as_rule(), Rule::coordinates);
        array
            .into_inner()
            .map(|coordblock| Coords::from_parser(coordblock, keysize))
            .collect()
    }

    pub fn from_parser(coordblock: Pair<Rule>, keysize: usize) -> Self {
        #[cfg(test)]
        assert_eq!(coordblock.as_rule(), Rule::coordblock);

        let mut sections = coordblock.into_inner();
        let offset = sections.next().unwrap();
        #[cfg(test)]
        assert_eq!(offset.as_rule(), Rule::offset);
        let mut offsets = offset.into_inner();

        let x = offsets.next().unwrap();
        #[cfg(test)]
        assert_eq!(x.as_rule(), Rule::coord);
        let x: u32 = x.as_str().parse().unwrap();

        let y = offsets.next().unwrap();
        #[cfg(test)]
        assert_eq!(y.as_rule(), Rule::coord);
        let y: u32 = y.as_str().parse().unwrap();

        let z = offsets.next().unwrap();
        #[cfg(test)]
        assert_eq!(z.as_rule(), Rule::coord);
        let z: u32 = z.as_str().parse().unwrap();

        let mapblock = sections.next().unwrap();
        #[cfg(test)]
        assert_eq!(mapblock.as_rule(), Rule::mapblock);

        let map_to_parse = mapblock
            .as_str()
            .replace("\t", "")
            .replace(" ", "")
            .replace("\r", "");

        Coords {
            offset: (x, y, z),
            keymap: Self::parse_map(&map_to_parse, keysize),
        }
    }

    pub fn parse_map(map: &str, keysize: usize) -> Vec<String> {
        let mut iter = map.chars().peekable();

        let mut vec = Vec::new();
        while iter.peek().is_some() {
            let mut key: String = iter.by_ref().take(keysize).collect();
            let extra = iter.peek();
            if extra.is_some() {
                match extra.unwrap() {
                    '\n' => {
                        let _: String = iter.by_ref().take(1).collect();
                        key.push('\n');
                    }
                    _ => (),
                }
            }
            vec.push(key)
        }

        vec
    }

    pub fn create_coord_to_key_map(&self, map: &mut HashMap<(u32, u32, u32), String>) {
        let mut current_coords = self.offset.clone();

        self.keymap.iter().for_each(|key| {
            map.insert(current_coords, key.trim().to_owned());
            if key.ends_with("\n") {
                current_coords.1 += 1;
                current_coords.0 = self.offset.0;
            } else {
                current_coords.0 += 1;
            }
        });
    }
}
