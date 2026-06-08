use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sqlx::MySqlPool;

use crate::{
    movimentacao::service::MovimentacaoService,
    repositorios::movimentacoes_estoque_repository::MovimentacaoEstoque,
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

pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/get-all", get(get_all_handler))
        .with_state(pool)
}
