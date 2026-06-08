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

    pub async fn insert(
        &self,
        item_id: i32,
        tipo: String,
        quantidade: i32,
        data_movimentacao: NaiveDateTime,
        observacao: Option<String>,
        responsavel_id: i32
    ) -> Result<i32, sqlx::Error> {
        let resultado = sqlx::query(
            "INSERT INTO Movimentacoes_Estoque (item_id, tipo, quantidade, data_movimentacao, observacao, responsavel_id) VALUES (?, ?, ?, ?, ?, ?)"
        ) 
        .bind(item_id)
        .bind(tipo)
        .bind(quantidade)
        .bind(data_movimentacao)
        .bind(observacao)
        .bind(responsavel_id)
        .execute(&self.pool)
        .await?;

        Ok(resultado.last_insert_id() as i32)
    }

    pub async fn delete_movimentacao_estoque(
        &self,
        id: i32
    ) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query("DELETE FROM Movimentacoes_Estoque WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(resultado.rows_affected())
    }
 
    pub async fn get_movimentacao_estoque(&self, id: i32) -> Result<Option<MovimentacaoEstoque>, sqlx::Error> {
        let movimentacao = sqlx::query_as::<_, MovimentacaoEstoque>("SELECT * FROM Movimentacoes_Estoque WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(movimentacao)
    }

    pub async fn get_all(&self) -> Result<Vec<MovimentacaoEstoque>, sqlx::Error> {
        let movimentacao_vec = sqlx::query_as::<_,MovimentacaoEstoque>(
            "SELECT * FROM Movimentacoes_Estoque" 
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(movimentacao_vec)
    }
}
