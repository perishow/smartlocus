use sqlx::MySqlPool;
use super::repository::{get_all_produtos, Produto, get_estoque_quantizavel, EstoqueQuantizavel};

// Nossa "Classe" de Serviço
pub struct ProdutoService {
    pool: MySqlPool,
}

impl ProdutoService {
    // O "Construtor" da nossa classe
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    // O método que a Rota vai chamar
    pub async fn listar_produtos(&self) -> Result<Vec<Produto>, sqlx::Error> {
        // É AQUI que entram as regras de negócio!
        // Por exemplo, você poderia verificar permissões, 
        // filtrar itens inativos, ou enviar um log de auditoria.
        
        println!("🔍 [Log do Serviço] Uma solicitação para listar produtos foi recebida.");

        // Como não temos regras complexas agora, apenas repassamos para o repositório
        let produtos = get_all_produtos(&self.pool).await?;

        // Se precisasse transformar os dados antes de devolver, faria aqui.
        Ok(produtos)
    }

    pub async fn listar_nome_produtos(&self) -> Result<Vec<String>, sqlx::Error> {
        println!("🔍 [Log do Serviço] Uma solicitação para listar nomes dos produtos foi recebida");

        let produtos: Vec<Produto> = get_all_produtos(&self.pool).await?;

        let nome_produtos: Vec<String> = produtos
            .into_iter()
            .map(|p| p.nome_produto)
            .collect();

        Ok(nome_produtos)
    }

    pub async fn listar_estoque_quantizavel(&self) -> Result<Vec<EstoqueQuantizavel>, sqlx::Error> {
        println!("🔍 [Log do Serviço] Uma solicitação para listar o estoque quantizavel foi recebida");

        let estoques: Vec<EstoqueQuantizavel> = get_estoque_quantizavel(&self.pool).await?;

        Ok(estoques)
    }
}
