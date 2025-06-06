pub mod dsl;
// Re-exportar os tipos principais da AST
pub use dsl::ast::{Program, Klass, Property, Metadata};

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::dsl::parser::tree::Tree;
    use crate::Metadata;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("eg/ts.ts").expect("Failed to read file");
        let tree = Tree::parse_input(input.clone()).unwrap();

        // Validar estrutura do programa
        assert_eq!(tree.program.klasses.len(), 1);

        let klass = &tree.program.klasses[0];
        assert_eq!(klass.name, "Cat");
        assert_eq!(klass.properties.len(), 2);

        // Validar propriedade "energy"
        let energy_prop = &klass.properties[0];
        assert_eq!(energy_prop.name, "energy");
        assert_eq!(energy_prop.metadata, Metadata::Number);

        // Validar propriedade "breath"
        let breath_prop = &klass.properties[1];
        assert_eq!(breath_prop.name, "breath");
        assert_eq!(breath_prop.metadata, Metadata::Number);

        println!("✅ Todas as validações passaram!");
        println!("Classe: {}", klass.name);
        for prop in &klass.properties {
            println!("  - {}: {:?}", prop.name, prop.metadata);
        }
    }

    #[test]
    fn test_multiple_classes() {
        let input = fs::read_to_string("ts.dsl").expect("Failed to read ts.dsl file");
        let tree = Tree::parse_input(input).unwrap();

        // Validar que tem 4 classes
        assert_eq!(tree.program.klasses.len(), 4);

        // Validar Animal
        let animal = &tree.program.klasses[0];
        assert_eq!(animal.name, "Animal");
        assert_eq!(animal.properties.len(), 3);
        assert_eq!(animal.properties[0].name, "name");
        assert_eq!(animal.properties[0].metadata, Metadata::String);

        // Validar Cat
        let cat = &tree.program.klasses[1];
        assert_eq!(cat.name, "Cat");
        assert_eq!(cat.properties.len(), 3);
        assert_eq!(cat.properties[2].name, "sleeping");
        assert_eq!(cat.properties[2].metadata, Metadata::Boolean);

        // Validar Person
        let person = &tree.program.klasses[3];
        assert_eq!(person.name, "Person");
        assert_eq!(person.properties.len(), 4);

        println!("✅ Validação de múltiplas classes passou!");
        println!("Total de classes: {}", tree.program.klasses.len());
        for klass in &tree.program.klasses {
            println!("  - {}: {} propriedades", klass.name, klass.properties.len());
        }
    }
}
