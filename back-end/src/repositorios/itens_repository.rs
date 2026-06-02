use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, FromRow, Serialize)]
pub struct Item {
    pub id: i32,
    pub nome: String,
    pub categoria: String,
    pub quantidade_atual: i32,
    pub quantidade_minima: i32,
    pub localizacao: String,
}

#[derive(Clone)]
pub struct ItensRepository {
    pool: MySqlPool,
}
impl ItensRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn insert_item(
        &self,
        nome: &str,
        categoria: &str,
        quantidade_atual: i32,
        quantidade_minima: i32,
        localizacao: &str,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO Itens (nome, categoria, quantidade_atual, quantidade_minima, localizacao) VALUES  (?, ?, ?, ?, ?)"
        )
        .bind(nome)
        .bind(categoria)
        .bind(quantidade_atual)
        .bind(quantidade_minima)
        .bind(localizacao)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id())
    }

    pub async fn delete_item(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM Itens WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_item_by_id(&self, id: i32) -> Result<Option<Item>, sqlx::Error> {
        let item = sqlx::query_as::<_, Item>("SELECT * FROM Itens WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(item)
    }
}
