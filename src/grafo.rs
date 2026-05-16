use std::collections::HashMap;

pub struct Grafo {
    pub recomendacoes: HashMap<u32, Vec<u32>>,
}

impl Grafo {
    pub fn novo() -> Self {
        Self {
            recomendacoes: HashMap::new(),
        }
    }

    pub fn adicionar_relacao(&mut self, origem: u32, destino: u32) {
        self.recomendacoes
            .entry(origem)
            .or_insert(Vec::new())
            .push(destino);
    }

    pub fn obter_recomendacoes(&self, produto_id: u32) -> Option<&Vec<u32>> {
        self.recomendacoes.get(&produto_id)
    }
}