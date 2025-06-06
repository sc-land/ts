use super::TypeAnnotation;

/// Representa uma declaração de propriedade em uma classe
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclaration {
    pub name: String,
    pub type_annotation: TypeAnnotation,
}

impl PropertyDeclaration {
    /// Cria uma nova declaração de propriedade
    pub fn new(name: String, type_annotation: TypeAnnotation) -> Self {
        Self {
            name,
            type_annotation,
        }
    }

    /// Verifica se a propriedade é de um tipo específico
    pub fn is_type(&self, type_annotation: &TypeAnnotation) -> bool {
        &self.type_annotation == type_annotation
    }

    /// Retorna uma representação em string do tipo
    pub fn type_as_string(&self) -> &'static str {
        match self.type_annotation {
            TypeAnnotation::Number => "number",
            TypeAnnotation::String => "string",
            TypeAnnotation::Boolean => "boolean",
        }
    }
}
