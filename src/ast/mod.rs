//! Módulo principal da AST (Abstract Syntax Tree)

pub mod program;
pub mod class_declaration;
pub mod property_declaration;
pub mod type_annotation;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-exportar os tipos principais
pub use program::Program;
pub use class_declaration::ClassDeclaration;
pub use property_declaration::PropertyDeclaration;
pub use type_annotation::TypeAnnotation;
