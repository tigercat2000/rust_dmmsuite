use pest::parses_to;

use super::*;
#[test]
fn parse_basic_prefab() {
    let prefab = r#""aa" = (/turf/icon/white,/area/debug)"#;

    let prefab = DMMParser::parse(Rule::prefab, &prefab)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut our_prefab = Prefab::Prefab::build();

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
        Prefab::Prefab::test_build("aa", vec!["/turf/icon/white", "/area/debug"])
    )
}

#[test]
fn parse_initialized_prefab() {
    let prefab = r#""al" = (/turf/icon/white/green/corner{tag = "icon-whitegreencorner (EAST)"; icon_state = "whitegreencorner"; dir = 4},/area/debug)"#;
    let prefab = DMMParser::parse(Rule::prefab, &prefab)
        .expect("Parsing failed")
        .next()
        .unwrap();

    let mut our_prefab = Prefab::Prefab::build();
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
        Prefab::Prefab::test_build(
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
    let mut our_prefab = Prefab::Prefab::build();
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
        Prefab::Prefab::test_build(
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

    let map = DMM::DMM::from_parser(parse);
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

    let map = DMM::DMM::from_parser(parse);
    // println!("{:#?}", map);
}
