use sqlx::{MySqlPool, FromRow};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct Produto {
    pub id_produto: i32,
    pub nome_produto: String,
    pub descricao: Option<String>, 
    pub id_tipo: i32,
    pub tipo_rastreamento: String, 
    pub data_cadastro: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct EstoqueQuantizavel {
    pub id_estoque: i32,
    pub id_produto: i32,
    pub quantidade: i32,
    pub ultima_compra: DateTime<Utc>,
}

pub async fn get_all_produtos(pool: &MySqlPool) -> Result<Vec<Produto>, sqlx::Error> {
    let produtos = sqlx::query_as::<_, Produto>("SELECT * FROM Produtos")
        .fetch_all(pool)
        .await?;

    Ok(produtos)
}

pub async fn get_estoque_quantizavel(pool: &MySqlPool) -> Result<Vec<EstoqueQuantizavel>, sqlx::Error> {
    let estoques = sqlx::query_as::<_, EstoqueQuantizavel>("SELECT * FROM Estoque_Quantizavel")
        .fetch_all(pool)
        .await?;

    Ok(estoques)
}
