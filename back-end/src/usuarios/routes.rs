use super::service::UsuariosService;
use crate::repositorios::usuarios_repository::Usuario;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
struct LoginRequest {
    nome: String,
    senha: String,
}

async fn listar_usuarios_handler(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Usuario>>, (StatusCode, String)> {
    let usuario_service: UsuariosService = UsuariosService::new(pool);

    match usuario_service.listar_usuarios().await {
        Ok(usuarios) => Ok(Json(usuarios)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro no banco de dados: {}", e),
        )),
    }
}

async fn coletar_usuario_por_id_handler(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<Usuario>, (StatusCode, String)> {
    let usuario_service = UsuariosService::new(pool);
    match usuario_service.coletar_usuario_por_id(id).await {
        Ok(usuario) => Ok(Json(usuario)),
        Err(e) => Err((
            (StatusCode::NOT_FOUND),
            "Usuário não encontrado".to_string(),
        )),
    }
}

async fn login_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Usuario>, (StatusCode, String)> {
    let usuario_service = UsuariosService::new(pool);
    match usuario_service
        .validar_usuario_e_senha(payload.nome, payload.senha)
        .await
    {
        Ok(usuario) => Ok(Json(usuario)),
        Err(mensagem) => {
            if mensagem == "Usuário ou senha incorretos" {
                Err((StatusCode::UNAUTHORIZED, mensagem))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, mensagem))
            }
        }
    }
}

pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/get-users", get(listar_usuarios_handler))
        .route("/get-user/:id", get(coletar_usuario_por_id_handler))
        .route("/login", post(login_handler))
        .with_state(pool)
}
