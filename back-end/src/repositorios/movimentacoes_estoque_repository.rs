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
    pub observacao: Option<String>,
    pub responsavel_id: i32,
}

/// Movimentação enriquecida com o nome do responsável (JOIN com Usuarios).
/// Usada no histórico por item exibido no drawer do frontend (RF-004).
#[derive(Debug, FromRow, Serialize)]
pub struct MovimentacaoComResponsavel {
    pub id: i32,
    pub item_id: i32,
    pub tipo: String,
    pub quantidade: i32,
    pub data_movimentacao: NaiveDateTime,
    pub observacao: Option<String>,
    pub responsavel_id: i32,
    pub responsavel_nome: String,
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

    pub async fn get_by_item_id(
        &self,
        item_id: i32,
    ) -> Result<Vec<MovimentacaoComResponsavel>, sqlx::Error> {
        let movimentacao_vec = sqlx::query_as::<_, MovimentacaoComResponsavel>(
            "SELECT m.id, m.item_id, m.tipo, m.quantidade, m.data_movimentacao, \
             m.observacao, m.responsavel_id, u.nome AS responsavel_nome \
             FROM Movimentacoes_Estoque m \
             JOIN Usuarios u ON u.id = m.responsavel_id \
             WHERE m.item_id = ? \
             ORDER BY m.data_movimentacao DESC",
        )
        .bind(item_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(movimentacao_vec)
    }
}
