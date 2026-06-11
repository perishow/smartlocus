use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sqlx::MySqlPool;

use crate::{
    movimentacao::service::MovimentacaoService,
    repositorios::movimentacoes_estoque_repository::{
        MovimentacaoComResponsavel, MovimentacaoEstoque,
    },
};

pub async fn get_all_handler(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<MovimentacaoEstoque>>, (StatusCode, String)> {
    let movimentacao_service = MovimentacaoService::new(pool);
    match movimentacao_service.coletar_todas_movimentacoes().await {
        Ok(vec) => Ok(Json(vec)),
        Err(mensagem) => Err((StatusCode::INTERNAL_SERVER_ERROR, mensagem)),
    }
}

pub async fn get_by_item_handler(
    State(pool): State<MySqlPool>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<MovimentacaoComResponsavel>>, (StatusCode, String)> {
    let movimentacao_service = MovimentacaoService::new(pool);
    match movimentacao_service
        .coletar_movimentacoes_por_item(item_id)
        .await
    {
        Ok(vec) => Ok(Json(vec)),
        Err(mensagem) => Err((StatusCode::INTERNAL_SERVER_ERROR, mensagem)),
    }
}

pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/get-all", get(get_all_handler))
        .route("/get-by-item/:item_id", get(get_by_item_handler))
        .with_state(pool)
}
