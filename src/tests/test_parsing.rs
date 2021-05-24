use crate::{Coord, DMMParser, Prefab, Rule, DMM};
use pest::parses_to;
use pest::Parser;

impl Prefab {
    pub fn test_build(key: &str, path_initializers: Vec<&str>) -> Self {
        Self {
            key: key.to_string(),
            path_initializers: path_initializers
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        }
    }
}

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

    let _ = DMM::from_parser(parse);
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

    let _ = DMM::from_parser(parse);
    // println!("{:#?}", map);
}

#[test]
fn parse_file_gpt_dmm() {
    let map = include_str!("dmm_files/gpt.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let _ = DMM::from_parser(parse);
}

#[test]
fn parse_file_tether_dmm() {
    let map = include_str!("dmm_files/tether.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let _ = DMM::from_parser(parse);
}

#[test]
fn parse_file_MetaStation_dmm() {
    let map = include_str!("dmm_files/MetaStation.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let _ = DMM::from_parser(parse);
}

#[test]
fn test_jsonize_dmm() {
    let map = include_str!("dmm_files/gpt.dmm");

    let parse = DMMParser::parse(Rule::map, map)
        .expect("Failed parse")
        .next()
        .unwrap();
    let map = DMM::from_parser(parse);

    map.to_json().expect("Failed to create JSON");
}

#[test]
fn test_to_loadable() {
    let dmm = r#""a" = (/turf,/area)
"b" = (/turf,/area)

(1,1,1) = {"
a
b
a
"}"#;
    let map = DMM::read_map(dmm);
    let loadable = map.to_loadable(1, 1, 1);

    // There is two paths per key, and 3 defined turfs.
    assert_eq!(loadable.len(), 6);

    assert_eq!(loadable[0].0, Coord { x: 1, y: 1, z: 1 });
    assert_eq!(loadable[5].0, Coord { x: 1, y: 3, z: 1 });
}

#[test]
fn test_tgm_to_loadable() {
    let dmm = r#""a" = (/turf,/area)
"b" = (/turf,/area)

(1,1,1) = {"
a
b
a
"}

(2,1,1) = {"
b
"}"#;
    let map = DMM::read_map(dmm);
    let loadable = map.to_loadable(1, 1, 1);

    // There is two paths per key, and 4 defined turfs.
    assert_eq!(loadable.len(), 8);

    assert_eq!(loadable[0].0, Coord { x: 1, y: 1, z: 1 });
    assert_eq!(loadable[5].0, Coord { x: 1, y: 3, z: 1 });
    assert_eq!(loadable[7].0, Coord { x: 2, y: 1, z: 1 });
}
