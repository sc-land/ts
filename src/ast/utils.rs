use super::{Program, TypeAnnotation};
use std::collections::HashMap;

/// Coleta estatísticas sobre um programa
pub fn analyze_program(program: &Program) -> ProgramStats {
    let mut stats = ProgramStats::new();

    for class in &program.classes {
        stats.class_count += 1;
        stats.total_properties += class.properties.len();

        for property in &class.properties {
            *stats.type_counts.entry(property.type_annotation.clone()).or_insert(0) += 1;
        }
    }

    stats
}

/// Estrutura com estatísticas do programa
#[derive(Debug)]
pub struct ProgramStats {
    pub class_count: usize,
    pub total_properties: usize,
    pub type_counts: HashMap<TypeAnnotation, usize>,
}

impl ProgramStats {
    pub fn new() -> Self {
        Self {
            class_count: 0,
            total_properties: 0,
            type_counts: HashMap::new(),
        }
    }

    pub fn print_summary(&self) {
        println!("📊 Estatísticas do Programa:");
        println!("  Classes: {}", self.class_count);
        println!("  Propriedades totais: {}", self.total_properties);
        println!("  Distribuição de tipos:");

        for (type_annotation, count) in &self.type_counts {
            println!("    • {}: {}", type_annotation, count);
        }
    }
}

impl Default for ProgramStats {
    fn default() -> Self {
        Self::new()
    }
}
