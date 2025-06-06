use std::env;
use std::fs;
use std::process;
use ts::{parse_ts, TSParser, Rule};
use ts::ast::utils::analyze_program;
use pest::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <typescript_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let input = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Erro ao ler arquivo {}: {}", filename, e);
            process::exit(1);
        }
    };

    println!("📁 Arquivo: {}", filename);
    println!("📝 Conteúdo:");
    println!("{}", input);
    println!();    // Parse da gramática
    match TSParser::parse(Rule::ts, &input) {
        Ok(_pairs) => {
            println!("✅ Parse bem-sucedido!");            // Gerar AST
            match parse_ts(&input) {
                Ok(program) => {
                    println!("🌳 AST gerada:");
                    println!("{:#?}", program);
                    
                    // Análise estatística
                    let stats = analyze_program(&program);
                    println!();
                    stats.print_summary();
                    
                    // Resumo detalhado
                    println!("\n📋 Detalhes das Classes:");
                    for class in &program.classes {
                        println!("  🏛️  {}: {} propriedades", class.name, class.properties.len());
                        for prop in &class.properties {
                            println!("    • {}: {}", prop.name, prop.type_as_string());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Erro na geração da AST: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Erro no parse: {}", e);
            process::exit(1);
        }
    }
}
