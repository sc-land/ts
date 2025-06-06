# TypeScript Parser DSL

Um parser TypeScript mínimo implementado em Rust usando Pest, com AST correspondente e CLI interativo.

## 🎯 Características

- **Gramática Pest**: Define sintaxe TypeScript básica (classes com propriedades tipadas)
- **AST Modular**: Estrutura organizada em módulos separados
- **CLI Interativo**: Interface de linha de comando com saída formatada e JSON
- **Suporte a Comentários**: Parser aceita comentários de linha (`//`)
- **Testes Abrangentes**: Validação completa da estrutura AST

- ✅ Parsing de classes simples
- ✅ Propriedades com tipos primitivos (`number`, `string`, `boolean`)
- ✅ Múltiplas classes no mesmo arquivo
- ✅ Geração de AST (Abstract Syntax Tree) modular
- ✅ Suporte a whitespace e formatação
- ✅ Métodos auxiliares para análise da AST
- ✅ Estatísticas do programa (contagem de tipos, classes, etc.)
- ✅ Estrutura modular da AST com arquivos separados

## Estrutura do Projeto

```
├── src/
│   ├── lib.rs          # Parser e funções principais
│   ├── main.rs         # Executável CLI
│   ├── ts.pest         # Gramática Pest
│   └── ast/            # Módulos da AST
│       ├── mod.rs                  # Módulo principal da AST
│       ├── program.rs              # Estrutura Program
│       ├── class_declaration.rs    # Estrutura ClassDeclaration
│       ├── property_declaration.rs # Estrutura PropertyDeclaration
│       ├── type_annotation.rs      # Enum TypeAnnotation
│       ├── utils.rs               # Utilitários de análise
│       └── tests.rs               # Testes dos métodos da AST
├── eg/
│   ├── ts.ts           # Exemplo inicial (classe Cat)
│   ├── example2.ts     # Exemplo com múltiplas classes
│   └── complex.ts      # Exemplo complexo com 3 classes
└── Cargo.toml          # Configuração do projeto
```

## Uso

### Como executável:

```bash
cargo run eg/ts.ts
cargo run eg/example2.ts
```

### Como biblioteca:

```rust
use ts::{parse_ts, Program};

let input = r#"
class Cat {
    energy: number;
    breath: number;
}
"#;

match parse_ts(input) {
    Ok(program) => {
        println!("Classes encontradas: {}", program.classes.len());
        for class in program.classes {
            println!("- {}: {} propriedades", class.name, class.properties.len());
        }
    }
    Err(e) => eprintln!("Erro: {}", e),
}
```

## Testes

```bash
cargo test                    # Todos os testes
cargo test test_ast           # Teste da AST básica
cargo test test_multiple_classes # Teste com múltiplas classes
```

## Gramática Atual (Pest)

```pest
// Entrada principal
ts = { SOI ~ (class_declaration)* ~ EOI }

// Declaração de classe
class_declaration = { "class" ~ identifier ~ "{" ~ class_body ~ "}" }

// Corpo da classe
class_body = { (property_declaration)* }

// Declaração de propriedade
property_declaration = { identifier ~ ":" ~ type_annotation ~ ";" }

// Anotação de tipo
type_annotation = { primitive_type }

// Tipos primitivos
primitive_type = { "number" | "string" | "boolean" }

// Identificador
identifier = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }

// Whitespace (espaços, tabs, newlines)
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
```

## AST (Abstract Syntax Tree)

```rust
pub struct Program {
    pub classes: Vec<ClassDeclaration>,
}

pub struct ClassDeclaration {
    pub name: String,
    pub properties: Vec<PropertyDeclaration>,
}

pub struct PropertyDeclaration {
    pub name: String,
    pub type_annotation: TypeAnnotation,
}

pub enum TypeAnnotation {
    Number,
    String,
    Boolean,
}
```

## Exemplos Suportados

### Classe simples:
```typescript
class Cat {
    energy: number;
    breath: number;
}
```

### Múltiplas classes:
```typescript
class Dog {
    name: string;
    age: number;
    isGoodBoy: boolean;
}

class House {
    address: string;
    rooms: number;
}
```

## Próximos Passos

- [ ] Métodos de classe
- [ ] Construtores
- [ ] Tipos de união (`string | number`)
- [ ] Arrays (`number[]`)
- [ ] Interfaces
- [ ] Imports/exports
- [ ] Herança de classes
- [ ] Modificadores de acesso (`public`, `private`, `protected`)
- [ ] Tipos genéricos
- [ ] Comentários

## Dependências

- `pest` - Parser generator
- `pest_derive` - Macros para Pest
- `clap` - Command line parsing

## Licença

Este projeto é de domínio público para fins educacionais.

## Módulos da AST

### `ast::Program`
- `new()` - Cria programa vazio
- `add_class()` - Adiciona classe
- `class_count()` - Conta classes
- `is_empty()` - Verifica se vazio

### `ast::ClassDeclaration`
- `new(name)` - Cria nova classe
- `add_property()` - Adiciona propriedade
- `property_count()` - Conta propriedades
- `has_properties()` - Verifica se tem propriedades
- `find_property(name)` - Busca propriedade por nome

### `ast::PropertyDeclaration`
- `new(name, type)` - Cria nova propriedade
- `is_type()` - Verifica tipo
- `type_as_string()` - Tipo como string

### `ast::TypeAnnotation`
- `from_str()` - Converte de string (com Result)
- `as_str()` - Converte para string
- `is_primitive()` - Verifica se é primitivo
- Implementa `Display`, `Hash`, `Eq`

### `ast::utils`
- `analyze_program()` - Gera estatísticas
- `ProgramStats` - Estrutura com estatísticas
- `print_summary()` - Mostra resumo estatístico
