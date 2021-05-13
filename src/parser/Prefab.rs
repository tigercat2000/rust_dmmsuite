use crate::parser::DMM::Rule;
use pest::iterators::Pair;

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
        #[cfg(test)]
        assert_eq!(array.as_rule(), Rule::prefabs);

        array
            .into_inner()
            .map(|prefab| Prefab::from_parser(prefab))
            .collect()
    }

    pub fn from_parser(prefab: Pair<Rule>) -> Self {
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
            path_initializers: Vec::new(),
        };

        new_self.take_paths(paths);

        new_self
    }

    pub fn take_paths(&mut self, pair: Pair<Rule>) {
        #[cfg(test)]
        assert_eq!(pair.as_rule(), Rule::paths);

        let paths = pair.into_inner();

        for path in paths {
            #[cfg(test)]
            assert_eq!(path.as_rule(), Rule::path);
            self.path_initializers.push(path.as_str().to_owned());
        }
    }
}
