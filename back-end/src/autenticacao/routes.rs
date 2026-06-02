use super::service::AuthService;
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

#[derive(Deserialize)]
struct RegisterRequest {
    nome: String,
    email: String,
    senha: String,
    perfil: String,
}

async fn login_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Usuario>, (StatusCode, String)> {
    let auth_service = AuthService::new(pool);

    match auth_service
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
        .route("/login", post(login_handler))
        .with_state(pool)
}
