#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

impl Produto {
    pub fn novo(id: u32, nome: &str, marca: &str, categoria: &str) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            marca: marca.to_string(),
            categoria: categoria.to_string(),
        }
    }
}