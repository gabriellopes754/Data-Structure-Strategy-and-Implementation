use megastore_search::produto::Produto;
use megastore_search::catalogo::Catalogo;
use megastore_search::grafo::Grafo;

#[test]
fn teste_busca_por_nome() {
    let mut catalogo = Catalogo::novo();

    let produto = Produto::novo(1, "Notebook", "Dell", "Eletrônicos");
    catalogo.adicionar_produto(produto);

    let resultado = catalogo.buscar_por_nome("Notebook");

    assert!(resultado.is_some());

    let produtos = resultado.unwrap();
    assert_eq!(produtos.len(), 1);
    assert_eq!(produtos[0].nome, "Notebook");
}

#[test]
fn teste_busca_por_marca() {
    let mut catalogo = Catalogo::novo();

    let produto = Produto::novo(1, "Mouse", "Logitech", "Eletrônicos");
    catalogo.adicionar_produto(produto);

    let resultado = catalogo.buscar_por_marca("Logitech");

    assert!(resultado.is_some());

    let produtos = resultado.unwrap();
    assert_eq!(produtos.len(), 1);
    assert_eq!(produtos[0].marca, "Logitech");
}

#[test]
fn teste_busca_por_categoria() {
    let mut catalogo = Catalogo::novo();

    let produto = Produto::novo(1, "Teclado", "Redragon", "Periféricos");
    catalogo.adicionar_produto(produto);

    let resultado = catalogo.buscar_por_categoria("Periféricos");

    assert!(resultado.is_some());

    let produtos = resultado.unwrap();
    assert_eq!(produtos.len(), 1);
    assert_eq!(produtos[0].categoria, "Periféricos");
}

#[test]
fn teste_produto_inexistente() {
    let catalogo = Catalogo::novo();

    let resultado = catalogo.buscar_por_nome("Produto Fantasma");

    assert!(resultado.is_none());
}

#[test]
fn teste_recomendacao_grafo() {
    let mut grafo = Grafo::novo();

    grafo.adicionar_relacao(1, 2);
    grafo.adicionar_relacao(1, 3);

    let recomendacoes = grafo.obter_recomendacoes(1);

    assert!(recomendacoes.is_some());

    let recs = recomendacoes.unwrap();
    assert_eq!(recs.len(), 2);
    assert!(recs.contains(&2));
    assert!(recs.contains(&3));
}