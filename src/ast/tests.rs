use crate::ast::{Program, ClassDeclaration, PropertyDeclaration, TypeAnnotation};

#[test]
fn test_ast_methods() {
    // Criar programa vazio
    let mut program = Program::new();
    assert!(program.is_empty());
    assert_eq!(program.class_count(), 0);

    // Criar classe Cat
    let mut cat_class = ClassDeclaration::new("Cat".to_string());
    assert_eq!(cat_class.name, "Cat");
    assert!(!cat_class.has_properties());
    assert_eq!(cat_class.property_count(), 0);

    // Adicionar propriedades
    let energy_prop = PropertyDeclaration::new("energy".to_string(), TypeAnnotation::Number);
    let breath_prop = PropertyDeclaration::new("breath".to_string(), TypeAnnotation::Number);

    assert!(energy_prop.is_type(&TypeAnnotation::Number));
    assert_eq!(energy_prop.type_as_string(), "number");

    cat_class.add_property(energy_prop);
    cat_class.add_property(breath_prop);

    assert!(cat_class.has_properties());
    assert_eq!(cat_class.property_count(), 2);

    // Buscar propriedade
    assert!(cat_class.find_property("energy").is_some());
    assert!(cat_class.find_property("speed").is_none());

    // Adicionar classe ao programa
    program.add_class(cat_class);
    assert!(!program.is_empty());
    assert_eq!(program.class_count(), 1);

    // Criar classe Dog com diferentes tipos
    let mut dog_class = ClassDeclaration::new("Dog".to_string());

    let name_prop = PropertyDeclaration::new("name".to_string(), TypeAnnotation::String);
    let age_prop = PropertyDeclaration::new("age".to_string(), TypeAnnotation::Number);
    let is_good_prop = PropertyDeclaration::new("isGoodBoy".to_string(), TypeAnnotation::Boolean);

    assert_eq!(name_prop.type_as_string(), "string");
    assert_eq!(age_prop.type_as_string(), "number");
    assert_eq!(is_good_prop.type_as_string(), "boolean");

    dog_class.add_property(name_prop);
    dog_class.add_property(age_prop);
    dog_class.add_property(is_good_prop);

    program.add_class(dog_class);

    assert_eq!(program.class_count(), 2);

    println!("✅ Todos os métodos da AST funcionaram corretamente!");
}

#[test]
fn test_type_annotation_methods() {
    // Teste TypeAnnotation::from_str
    assert_eq!(TypeAnnotation::from_str("number").unwrap(), TypeAnnotation::Number);
    assert_eq!(TypeAnnotation::from_str("string").unwrap(), TypeAnnotation::String);
    assert_eq!(TypeAnnotation::from_str("boolean").unwrap(), TypeAnnotation::Boolean);
    assert!(TypeAnnotation::from_str("invalid").is_err());

    // Teste as_str
    assert_eq!(TypeAnnotation::Number.as_str(), "number");
    assert_eq!(TypeAnnotation::String.as_str(), "string");
    assert_eq!(TypeAnnotation::Boolean.as_str(), "boolean");

    // Teste is_primitive
    assert!(TypeAnnotation::Number.is_primitive());
    assert!(TypeAnnotation::String.is_primitive());
    assert!(TypeAnnotation::Boolean.is_primitive());

    // Teste Display trait
    assert_eq!(format!("{}", TypeAnnotation::Number), "number");
    assert_eq!(format!("{}", TypeAnnotation::String), "string");
    assert_eq!(format!("{}", TypeAnnotation::Boolean), "boolean");

    println!("✅ Todos os métodos de TypeAnnotation funcionaram corretamente!");
}
