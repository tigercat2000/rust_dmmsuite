use crate::parser::DMM::Rule;
use auxtools::Value;
use pest::iterators::Pair;
use std::collections::{BTreeMap, HashMap};

struct InitializeStatement {
    pub key: Value,
    pub value: Value,
}

impl InitializeStatement {
    /// This turns the string
    /// > desc = "A station intercom. It looks like it has been modified to not broadcast."
    /// into
    /// ```
    /// InitializeStatement {
    ///    key: Value::from_string("desc"),
    ///    value: Value::from_string("A station intercom. It looks like it has been modified to not broadcast.")
    /// }
    /// ```
    ///
    pub fn new(str: String) -> Self {
        // This turns `desc = "A station intercom. It looks like it has been modified to not broadcast."` into
        // ["desc", "\"A station intercom. It looks like it has been modified to not broadcast.\""]
        let vec: Vec<&str> = str.splitn(2, '=').map(|x| x.trim()).collect();
        Self {
            key: parse_value(vec[0].to_owned()),
            value: parse_value(vec[1].to_owned()),
        }
    }

    /// This implements parsing a subset of DM to turn text into Value's
    fn parse_value(str: String) -> Value {
        todo!()
    }
}

struct Prefab {
    pub typepath: String,
    pub initializers: Vec<InitializeStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefabList {
    pub key: String,
    pub prefabs: Vec<String>,
}

impl PrefabList {
    pub fn build() -> Self {
        Self {
            key: String::new(),
            prefabs: vec![],
        }
    }

    pub fn from_parser_array(array: Pair<Rule>) -> HashMap<String, Self> {
        #[cfg(test)]
        assert_eq!(array.as_rule(), Rule::prefabs);

        array
            .into_inner()
            .map(|prefab| PrefabList::from_parser(prefab))
            .collect()
    }

    pub fn from_parser(prefab: Pair<Rule>) -> (String, Self) {
        #[cfg(test)]
        assert_eq!(prefab.as_rule(), Rule::prefab);

        let mut sections = prefab.into_inner();
        let id = sections.next().unwrap();
        #[cfg(test)]
        assert_eq!(id.as_rule(), Rule::id);
        let paths = sections.next().unwrap();
        #[cfg(test)]
        assert_eq!(paths.as_rule(), Rule::paths);

        let mut new_self = Self {
            key: id.as_str().to_string(),
            prefabs: Vec::new(),
        };

        new_self.take_paths(paths);

        (new_self.key.clone(), new_self)
    }

    pub fn take_paths(&mut self, pair: Pair<Rule>) {
        #[cfg(test)]
        assert_eq!(pair.as_rule(), Rule::paths);

        let paths = pair.into_inner();

        for path in paths {
            #[cfg(test)]
            assert_eq!(path.as_rule(), Rule::path);
            self.prefabs.push(path.as_str().to_owned());
        }
    }
}
