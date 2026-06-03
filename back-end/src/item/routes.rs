use crate::repositorios::itens_repository::Item;

use super::service::ItemService;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
struct RegisterItemRequest {
    nome: String,
    categoria: String,
    quantidade_atual: i32,
    quantidade_minima: i32,
    localizacao: String,
}

async fn register_item_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterItemRequest>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let item_service = ItemService::new(pool);
    match item_service
        .inserir_novo_item(
            payload.nome,
            payload.categoria,
            payload.quantidade_atual,
            payload.quantidade_minima,
            payload.localizacao,
        )
        .await
    {
        Ok(item_id) => Ok(Json(item_id)),
        Err(mensagem) => Err((StatusCode::INTERNAL_SERVER_ERROR, mensagem)),
    }
}

pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/register-item", post(register_item_handler))
        .with_state(pool)
}
