use serde::{Deserialize, Serialize};
use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

use super::Property;

/// Representa uma declaração de classe TypeScript
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Klass {
    pub name: String,
    pub properties: Vec<Property>,
}

impl Klass {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::klass);

        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let klass_body = inner.next().unwrap();

        let mut properties = Vec::new();
        for property_pair in klass_body.into_inner() {
            if property_pair.as_rule() == Rule::property {
                properties.push(Property::from_pair(property_pair));
            }
        }

        Klass { name, properties }
    }
}


