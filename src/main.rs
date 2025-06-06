use std::fs;
use clap::{Arg, Command};
use ts::dsl::parser::tree::Tree;

fn main() {
    let matches = Command::new("TypeScript Parser")
        .version("0.1.0")
        .about("Parse TypeScript DSL files and display AST")
        .arg(
            Arg::new("file")
                .help("The TypeScript file to parse")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output as JSON")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();
    let output_json = matches.get_flag("json");

    match fs::read_to_string(file_path) {
        Ok(input) => {
            match Tree::parse_input(input) {
                Ok(tree) => {
                    if output_json {
                        match serde_json::to_string_pretty(&tree) {
                            Ok(json) => println!("{}", json),
                            Err(e) => eprintln!("❌ Erro ao serializar JSON: {}", e),
                        }
                    } else {
                        println!("🎯 Arquivo parseado com sucesso: {}", file_path);
                        println!("📊 Estatísticas:");
                        println!("  - Classes encontradas: {}", tree.program.klasses.len());

                        let total_props: usize = tree.program.klasses.iter()
                            .map(|k| k.properties.len())
                            .sum();
                        println!("  - Total de propriedades: {}", total_props);

                        println!("\n📋 Estrutura:");
                        for klass in &tree.program.klasses {
                            println!("  class {} {{", klass.name);
                            for prop in &klass.properties {
                                println!("    {}: {:?};", prop.name, prop.metadata);
                            }
                            println!("  }}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Erro ao fazer parsing: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Erro ao ler arquivo '{}': {}", file_path, e);
            std::process::exit(1);
        }
    }
}
