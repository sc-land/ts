use pest_derive::Parser;
use pest::Parser;

// Módulo AST
pub mod ast;

// Re-exportar os tipos principais da AST
pub use ast::{Program, ClassDeclaration, PropertyDeclaration, TypeAnnotation};

fn print_tree(pair: &pest::iterators::Pair<Rule>, depth: usize, is_last: bool) {
    let indent = if depth == 0 {
        String::new()
    } else {
        let mut result = String::new();
        for _ in 0..depth - 1 {
            result.push_str("  ");
        }
        if is_last {
            result.push_str("└─ ");
        } else {
            result.push_str("├─ ");
        }
        result
    };

    let rule_name = format!("{:?}", pair.as_rule()).to_lowercase();
    let text = pair.as_str();

    let inner_pairs: Vec<_> = pair.clone().into_inner().collect();

    if inner_pairs.is_empty() {
        // Folha: mostrar regra e conteúdo
        if text.is_empty() {
            println!("{}{}: \"\"", indent, rule_name);
        } else if text.len() < 50 && !text.contains('\n') {
            println!("{}{}: \"{}\"", indent, rule_name, text);
        } else {
            println!("{}{}", indent, rule_name);
        }
    } else {
        // Nó interno: mostrar regra e processar filhos
        println!("{}{}", indent, rule_name);

        for (i, inner_pair) in inner_pairs.iter().enumerate() {
            let is_last_child = i == inner_pairs.len() - 1;
            print_tree(inner_pair, depth + 1, is_last_child);
        }
    }
}



#[derive(Parser)]
#[grammar = "ts.pest"]
pub struct TSParser;

// Parser para AST
pub fn parse_ts(input: &str) -> Result<Program, pest::error::Error<Rule>> {
    let pairs = TSParser::parse(Rule::ts, input)?;
    let mut program = Program::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::ts => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::class_declaration => {
                            program.add_class(parse_class_declaration(inner_pair)?);
                        }
                        Rule::EOI => {} // Fim do input
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(program)
}

fn parse_class_declaration(pair: pest::iterators::Pair<Rule>) -> Result<ClassDeclaration, pest::error::Error<Rule>> {
    let mut class = ClassDeclaration::new(String::new());

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => {
                let name = inner_pair.as_str().to_string();
                class = ClassDeclaration::new(name);
            }
            Rule::class_body => {
                for body_pair in inner_pair.into_inner() {
                    match body_pair.as_rule() {
                        Rule::property_declaration => {
                            class.add_property(parse_property_declaration(body_pair)?);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(class)
}

fn parse_property_declaration(pair: pest::iterators::Pair<Rule>) -> Result<PropertyDeclaration, pest::error::Error<Rule>> {
    let mut name = String::new();
    let mut type_annotation = TypeAnnotation::Number; // default

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => {
                name = inner_pair.as_str().to_string();
            }
            Rule::type_annotation => {
                for type_pair in inner_pair.into_inner() {
                    match type_pair.as_rule() {
                        Rule::primitive_type => {
                            type_annotation = TypeAnnotation::from(type_pair.as_str());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(PropertyDeclaration::new(name, type_annotation))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use pest::Parser;
    use crate::{parse_ts, print_tree, Rule, TSParser};

    #[test]
    fn test_parse_tree() {
        let input = match fs::read_to_string("eg/ts.ts") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Erro ao ler arquivo ts.ts: {}", e);
                return;
            }
        };

        match TSParser::parse(Rule::ts, &input) {
            Ok(pairs) => {
                println!("✅ Parse tree bem-sucedido!");
                println!();

                // Vamos mostrar a árvore de parse no formato intuitivo
                for pair in pairs {
                    print_tree(&pair, 0, true);
                }
            }
            Err(e) => {
                println!("❌ Erro no parse tree:");
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_ast() {
        let input = match fs::read_to_string("eg/ts.ts") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Erro ao ler arquivo ts.ts: {}", e);
                return;
            }
        };

        match parse_ts(&input) {
            Ok(program) => {
                println!("✅ AST gerada com sucesso!");
                println!();
                println!("AST:");
                println!("{:#?}", program);

                // Verificar se a AST está correta
                assert_eq!(program.classes.len(), 1);
                let cat_class = &program.classes[0];
                assert_eq!(cat_class.name, "Cat");
                assert_eq!(cat_class.properties.len(), 2);

                let energy_prop = &cat_class.properties[0];
                assert_eq!(energy_prop.name, "energy");
                assert_eq!(energy_prop.type_annotation, crate::TypeAnnotation::Number);

                let breath_prop = &cat_class.properties[1];
                assert_eq!(breath_prop.name, "breath");
                assert_eq!(breath_prop.type_annotation, crate::TypeAnnotation::Number);

                println!("✅ Todas as verificações da AST passaram!");
            }
            Err(e) => {
                println!("❌ Erro na geração da AST:");
                println!("{}", e);
            }
        }
    }

    #[test]
    fn test_multiple_classes() {
        let input = match fs::read_to_string("eg/example2.ts") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Erro ao ler arquivo example2.ts: {}", e);
                return;
            }
        };

        match parse_ts(&input) {
            Ok(program) => {
                println!("✅ AST com múltiplas classes gerada com sucesso!");
                println!();
                println!("AST:");
                println!("{:#?}", program);

                // Verificar se a AST está correta
                assert_eq!(program.classes.len(), 2);

                let dog_class = &program.classes[0];
                assert_eq!(dog_class.name, "Dog");
                assert_eq!(dog_class.properties.len(), 3);
                assert_eq!(dog_class.properties[0].name, "name");
                assert_eq!(dog_class.properties[0].type_annotation, crate::TypeAnnotation::String);
                assert_eq!(dog_class.properties[1].name, "age");
                assert_eq!(dog_class.properties[1].type_annotation, crate::TypeAnnotation::Number);
                assert_eq!(dog_class.properties[2].name, "isGoodBoy");
                assert_eq!(dog_class.properties[2].type_annotation, crate::TypeAnnotation::Boolean);

                let house_class = &program.classes[1];
                assert_eq!(house_class.name, "House");
                assert_eq!(house_class.properties.len(), 2);
                assert_eq!(house_class.properties[0].name, "address");
                assert_eq!(house_class.properties[0].type_annotation, crate::TypeAnnotation::String);
                assert_eq!(house_class.properties[1].name, "rooms");
                assert_eq!(house_class.properties[1].type_annotation, crate::TypeAnnotation::Number);

                println!("✅ Todas as verificações para múltiplas classes passaram!");
            }
            Err(e) => {
                println!("❌ Erro na geração da AST:");
                println!("{}", e);
            }
        }
    }
}
