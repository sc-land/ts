/// Representa uma anotação de tipo TypeScript
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeAnnotation {
    Number,
    String,
    Boolean,
}

impl TypeAnnotation {
    /// Converte uma string em TypeAnnotation
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "number" => Ok(TypeAnnotation::Number),
            "string" => Ok(TypeAnnotation::String),
            "boolean" => Ok(TypeAnnotation::Boolean),
            _ => Err(format!("Tipo não suportado: {}", s)),
        }
    }

    /// Converte TypeAnnotation para string
    pub fn as_str(&self) -> &'static str {
        match self {
            TypeAnnotation::Number => "number",
            TypeAnnotation::String => "string",
            TypeAnnotation::Boolean => "boolean",
        }
    }

    /// Verifica se é um tipo primitivo
    pub fn is_primitive(&self) -> bool {
        match self {
            TypeAnnotation::Number | TypeAnnotation::String | TypeAnnotation::Boolean => true,
        }
    }
}

impl From<&str> for TypeAnnotation {
    fn from(s: &str) -> Self {
        match s {
            "number" => TypeAnnotation::Number,
            "string" => TypeAnnotation::String,
            "boolean" => TypeAnnotation::Boolean,
            _ => panic!("Tipo não suportado: {}", s),
        }
    }
}

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
