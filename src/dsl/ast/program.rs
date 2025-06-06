use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use crate::dsl::parser::parser::Rule;
use super::Klass;

/// Representa um programa TypeScript completo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    pub klasses: Vec<Klass>,
}

impl Program {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::program);

        let mut klasses = Vec::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::klass => {
                    klasses.push(Klass::from_pair(inner_pair));
                }
                _ => {} // Ignorar outros tipos de regras
            }
        }

        Program { klasses }
    }
}
