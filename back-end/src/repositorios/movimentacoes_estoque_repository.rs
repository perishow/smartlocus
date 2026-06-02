use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, FromRow, Serialize)]
pub struct MovimentacaoEstoque {
    pub id: i32,
    pub item_id: i32,
    pub tipo: String,
    pub quantidade: i32,
    pub data_movimentacao: NaiveDateTime,
    pub observacao: String,
    pub responsavel_id: i32,
}

pub struct MovimentacaoEstoqueRepository {
    pool: MySqlPool,
}
impl MovimentacaoEstoqueRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn insert_movimentacao_estoque(
        &self,
        item_id: i32,
        tipo: &str,
        quantidade: i32,
        data_movimentacao: NaiveDateTime,
        observacao: Option<&str>,
        responsavel_id: i32
    ) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query(
        )
    }
}
