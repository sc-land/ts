use super::PropertyDeclaration;

/// Representa uma declaração de classe TypeScript
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub name: String,
    pub properties: Vec<PropertyDeclaration>,
}

impl ClassDeclaration {
    /// Cria uma nova declaração de classe
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: Vec::new(),
        }
    }

    /// Adiciona uma propriedade à classe
    pub fn add_property(&mut self, property: PropertyDeclaration) {
        self.properties.push(property);
    }

    /// Retorna o número de propriedades na classe
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }

    /// Verifica se a classe tem propriedades
    pub fn has_properties(&self) -> bool {
        !self.properties.is_empty()
    }

    /// Busca uma propriedade pelo nome
    pub fn find_property(&self, name: &str) -> Option<&PropertyDeclaration> {
        self.properties.iter().find(|prop| prop.name == name)
    }
}
