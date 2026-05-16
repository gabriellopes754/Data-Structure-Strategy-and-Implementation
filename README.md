# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este projeto implementa um sistema de busca e recomendação de produtos para a MegaStore utilizando Rust, com foco em desempenho, organização e escalabilidade.

A solução foi desenvolvida para resolver problemas de lentidão e imprecisão em sistemas tradicionais de busca, utilizando tabelas hash (HashMap) para indexação eficiente e grafos para recomendação de produtos relacionados.

## Objetivos do Sistema
- Indexar produtos de forma rápida e eficiente
- Realizar buscas por nome, marca e categoria
- Melhorar a precisão dos resultados
- Implementar recomendações inteligentes
- Garantir escalabilidade para grandes volumes de dados

---

## Funcionalidades
- Cadastro de produtos
- Busca por nome
- Busca por marca
- Busca por categoria
- Recomendação de produtos relacionados
- Testes unitários e de integração

---

## Tecnologias Utilizadas
- Rust (Edition 2021)
- HashMap (`std::collections::HashMap`)
- Grafos (`HashMap<u32, Vec<u32>>`)
- Cargo
- Cargo Test
- GitHub

---

## Estrutura do Projeto
```txt
megastore-search/
│── Cargo.toml
│── README.md
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── produto.rs
│   ├── catalogo.rs
│   └── grafo.rs
│
└── tests/
    └── sistema_tests.rs
```

---

## Como Executar o Sistema

### Pré-requisitos:
- Rust instalado
- Cargo instalado

### Comando:
```bash
cargo run
```

### Saída esperada:
```txt
Busca por nome:
Encontrado: Notebook
Marca: Dell
Categoria: Eletrônicos
Recomendações:
- Mouse
- Teclado
```

---

## Como Executar os Testes

### Comando:
```bash
cargo test
```

### Testes incluídos:
- Busca por nome
- Busca por marca
- Busca por categoria
- Produto inexistente
- Recomendação por grafo

---

## Exemplos de Uso

### Buscar por nome:
```rust
catalogo.buscar_por_nome("Notebook");
```

### Buscar por marca:
```rust
catalogo.buscar_por_marca("Dell");
```

### Buscar por categoria:
```rust
catalogo.buscar_por_categoria("Eletrônicos");
```

### Obter recomendações:
```rust
grafo.obter_recomendacoes(1);
```

---

## Arquitetura do Sistema

### `produto.rs`
Responsável pela estrutura de dados do produto:
- ID
- Nome
- Marca
- Categoria

### `catalogo.rs`
Responsável por:
- Armazenamento de produtos
- Indexação
- Busca por filtros

### `grafo.rs`
Responsável por:
- Relações entre produtos
- Sistema de recomendação

### `main.rs`
Executa o sistema principal

### `tests/`
Responsável por validação e confiabilidade

---

## Algoritmos e Estruturas de Dados Utilizados

### Tabelas Hash (HashMap)
Utilizadas para armazenamento e busca rápida.

### Vantagens:
- Complexidade média O(1)
- Alta performance
- Escalabilidade

### Grafos
Utilizados para representar relações entre produtos.

### Exemplo:
Notebook → Mouse, Teclado

---

## Considerações sobre Desempenho e Escalabilidade

### Desempenho:
- Busca linear tradicional: O(n)
- Busca com HashMap: O(1)

### Benefícios:
- Redução no tempo de resposta
- Melhor experiência do usuário
- Menor uso de processamento

### Escalabilidade:
A estrutura permite expansão para milhões de produtos mantendo desempenho eficiente.


## Licença
Este projeto utiliza a licença MIT para fins acadêmicos e educacionais.

