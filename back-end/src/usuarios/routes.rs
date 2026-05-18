use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use sqlx::MySqlPool;
use super::service::UsuariosService;
use super::repository::Usuario;

async fn listar_usuarios_handler(State(pool): State<MySqlPool>) -> Result<Json<Vec<Usuario>>, (StatusCode, String)> {
    let usuario_service = UsuariosService::new(pool);

    match usuario_service.listar_usuarios().await {
        Ok(usuarios) => Ok(Json(usuarios)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro no banco de dados: {}", e)
        )),
    }
}

pub fn router(pool:MySqlPool) -> Router {
    Router::new()
        .route("/get-users", get(listar_usuarios_handler))
        .with_state(pool)
}
