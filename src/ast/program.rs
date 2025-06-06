use super::ClassDeclaration;

/// Representa um programa TypeScript completo
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub classes: Vec<ClassDeclaration>,
}

impl Program {
    /// Cria um novo programa vazio
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    /// Adiciona uma classe ao programa
    pub fn add_class(&mut self, class: ClassDeclaration) {
        self.classes.push(class);
    }

    /// Retorna o número de classes no programa
    pub fn class_count(&self) -> usize {
        self.classes.len()
    }

    /// Verifica se o programa está vazio
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
