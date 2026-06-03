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
        nome: String,
        categoria: String,
        quantidade_atual: i32,
        quantidade_minima: i32,
        localizacao: String,
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

    pub async fn get_item_by_id(&self, id: u64) -> Result<Option<Item>, sqlx::Error> {
        let item = sqlx::query_as::<_, Item>("SELECT * FROM Itens WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(item)
    }

    pub async fn get_item_id_by_nome(&self, nome: String) -> Result<Option<u64>, sqlx::Error> {
        let id = sqlx::query_scalar("SELECT id FROM Itens WHERE nome = ?")
            .bind(nome)
            .fetch_optional(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn update_item_quantidade(
        &self,
        id: u64,
        nova_quantidade: i32,
    ) -> Result<u64, sqlx::Error> {
        let resultado = sqlx::query("UPDATE Itens SET quantidade_atual = ? WHERE id = ?")
            .bind(nova_quantidade)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(resultado.rows_affected())
    }
}
