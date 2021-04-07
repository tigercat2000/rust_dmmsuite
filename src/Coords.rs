use super::*;
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
