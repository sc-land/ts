use serde::{Deserialize, Serialize};
use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

use super::Metadata;

/// Representa uma declaração de propriedade em uma classe
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub metadata: Metadata,
}

impl Property {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::property);

        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let metadata_pair = inner.next().unwrap();

        let metadata = Metadata::from_pair(metadata_pair);

        Property { name, metadata }
    }
}
