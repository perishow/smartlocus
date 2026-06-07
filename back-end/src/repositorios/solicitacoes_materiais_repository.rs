use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(FromRow, Serialize)]
pub struct SolicitacaoMateriais {
    pub id: i32,
    pub item_id: i32,
    pub solicitante_id: i32,
    pub quantidade_solicitada: i32,
    pub status: String,
    pub data_solicitacao: NaiveDateTime,
}

pub struct SolicitacaoMateriaisRepository {
    pool: MySqlPool,
}
impl SolicitacaoMateriaisRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        item_id: i32,
        solicitante_id: i32,
        quantidade_solicitada: i32,
        status: String,
        data_solicitacao: NaiveDateTime,
    ) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query(
            "INSERT INTO Solicitacoes_Materiais (item_id, solicitante_id, quantidade_solicitada, status, data_solicitacao) VALUES (?, ?, ?, ?, ?)"
        )
            .bind(item_id)
            .bind(solicitante_id)
            .bind(quantidade_solicitada)
            .bind(status)
            .bind(data_solicitacao)
            .execute(&self.pool)
            .await?;
        Ok(resultado.last_insert_id())
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query("DELETE FROM Solicitacoes_Materiais WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(resultado.rows_affected())
    }

    pub async fn get(&self, id: i32) -> Result<Option<SolicitacaoMateriais>, sqlx::Error> {
        let solicitacao = sqlx::query_as::<_, SolicitacaoMateriais>(
            "SELECT * FROM Solicitacoes_Materiais WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(solicitacao)
    }

    pub async fn get_all(&self) -> Result<Vec<SolicitacaoMateriais>, sqlx::Error> {
        let solicitacoes =
            sqlx::query_as::<_, SolicitacaoMateriais>("SELECT * FROM Solicitacoes_Materiais")
                .fetch_all(&self.pool)
                .await?;
        Ok(solicitacoes)
    }
}
