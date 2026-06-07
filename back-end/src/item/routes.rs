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

#[derive(Deserialize)]
struct DeleteItemRequest {
    id: i32,
}

#[derive(Deserialize)]
struct UpdateQuantidadeRequest {
    id: i32,
    quantidade: i32,
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

async fn delete_item_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<DeleteItemRequest>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let item_service = ItemService::new(pool);
    match item_service.deletar_item(payload.id).await {
        Ok(id) => Ok(Json(id)),
        Err(mensagem) => Err((StatusCode::NOT_FOUND, mensagem)),
    }
}

async fn adicionar_quantidade_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<UpdateQuantidadeRequest>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let item_service = ItemService::new(pool);
    match item_service
        .adicionar_quantidade_item(payload.id, payload.quantidade)
        .await
    {
        Ok(id) => Ok(Json(id)),
        Err(mensagem) => Err((StatusCode::NOT_ACCEPTABLE, mensagem)),
    }
}

async fn subtrair_quantidade_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<UpdateQuantidadeRequest>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let item_service = ItemService::new(pool);
    match item_service
        .subtrair_quantidade_item(payload.id, payload.quantidade)
        .await
    {
        Ok(id) => Ok(Json(id)),
        Err(mensagem) => Err((StatusCode::NOT_ACCEPTABLE, mensagem)),
    }
}

pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/register-item", post(register_item_handler))
        .route("/delete-item", post(delete_item_handler))
        .route("/adicionar-quantidade", post(adicionar_quantidade_handler))
        .route("/subtrair-quantidade", post(subtrair_quantidade_handler))
        .with_state(pool)
}
