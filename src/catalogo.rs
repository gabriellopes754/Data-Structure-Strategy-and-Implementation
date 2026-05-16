use std::collections::HashMap;
use crate::produto::Produto;

pub struct Catalogo {
    pub produtos: HashMap<u32, Produto>,
}

impl Catalogo {
    pub fn novo() -> Self {
        Self {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto);
    }

    pub fn buscar_por_id(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<Vec<&Produto>> {
        let resultados: Vec<&Produto> = self
            .produtos
            .values()
            .filter(|p| p.nome == nome)
            .collect();

        if resultados.is_empty() {
            None
        } else {
            Some(resultados)
        }
    }

    pub fn buscar_por_marca(&self, marca: &str) -> Option<Vec<&Produto>> {
        let resultados: Vec<&Produto> = self
            .produtos
            .values()
            .filter(|p| p.marca == marca)
            .collect();

        if resultados.is_empty() {
            None
        } else {
            Some(resultados)
        }
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Option<Vec<&Produto>> {
        let resultados: Vec<&Produto> = self
            .produtos
            .values()
            .filter(|p| p.categoria == categoria)
            .collect();

        if resultados.is_empty() {
            None
        } else {
            Some(resultados)
        }
    }
}