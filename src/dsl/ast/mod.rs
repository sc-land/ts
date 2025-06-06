//! Módulo principal da AST (Abstract Syntax Tree)

pub mod program;
pub mod klass;
pub mod property;
pub mod metadata;


// Re-exportar os tipos principais
pub use program::Program;
pub use klass::Klass;
pub use property::Property;
pub use metadata::Metadata;
