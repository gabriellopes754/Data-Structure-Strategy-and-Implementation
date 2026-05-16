mod produto;
mod catalogo;
mod grafo;

use produto::Produto;
use catalogo::Catalogo;
use grafo::Grafo;

fn main() {
  
    let mut catalogo = Catalogo::novo();
    let mut grafo = Grafo::novo();

   
    let p1 = Produto::novo(1, "Notebook", "Dell", "Eletrônicos");
    let p2 = Produto::novo(2, "Mouse", "Logitech", "Eletrônicos");
    let p3 = Produto::novo(3, "Teclado", "Redragon", "Eletrônicos");

    
    catalogo.adicionar_produto(p1.clone());
    catalogo.adicionar_produto(p2.clone());
    catalogo.adicionar_produto(p3.clone());

    
    grafo.adicionar_relacao(1, 2);
    grafo.adicionar_relacao(1, 3);

    
    println!("Busca por nome:");
    if let Some(produtos) = catalogo.buscar_por_nome("Notebook") {
        for produto in produtos {
            println!("Encontrado: {}", produto.nome);
            println!("Marca: {}", produto.marca);
            println!("Categoria: {}", produto.categoria);

          
            println!("Recomendações:");
            if let Some(recs) = grafo.obter_recomendacoes(produto.id) {
                for id in recs {
                    if let Some(rec) = catalogo.buscar_por_id(*id) {
                        println!("- {}", rec.nome);
                    }
                }
            }
        }
    }
}