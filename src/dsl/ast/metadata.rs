use serde::{Deserialize, Serialize};
use pest::iterators::Pair;
use crate::dsl::parser::parser::Rule;

/// Representa uma anotação de tipo TypeScript
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Metadata {
    Number,
    String,
    Boolean,
}

impl Metadata {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::metadata);

        let primitive_pair = pair.into_inner().next().unwrap();

        match primitive_pair.as_str() {
            "number" => Metadata::Number,
            "string" => Metadata::String,
            "boolean" => Metadata::Boolean,
            _ => panic!("Tipo primitivo não suportado: {}", primitive_pair.as_str()),
        }
    }
}
