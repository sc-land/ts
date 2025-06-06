use pest::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::dsl::parser::parser::{Rule, TSP};
use crate::Program;

#[derive(Debug, Error)]
pub enum TreeParseError {
    #[error("SC Parsing failed{0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Falha ao processar árvore vazia")]
    EmptyTree,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tree {
    pub program: Program,
}

impl Tree {
    pub fn parse_input(input: String) -> Result<Self, TreeParseError> {
        let parsed = TSP::parse(Rule::ts, &input);
        match parsed {
            Ok(mut parsed) => {
                let ts_pair = parsed.next().ok_or(TreeParseError::EmptyTree)?;
                // Extrair o programa de dentro da regra ts
                let program_pair = ts_pair.into_inner().next().ok_or(TreeParseError::EmptyTree)?;
                let program = Program::from_pair(program_pair);
                Ok(Tree { program })
            }
            Err(e) => Err(TreeParseError::from(e)),
        }
    }
}
