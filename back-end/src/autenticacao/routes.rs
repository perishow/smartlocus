use super::service::AuthService;
use crate::repositorios::usuarios_repository::Usuario;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
struct RegisterRequest {
    nome: String,
    email: String,
    senha: String,
    perfil: String,
}

#[derive(Deserialize)]
struct DeleteRequest {
    id: i32,
}

#[derive(Deserialize)]
struct LoginRequest {
    nome: String,
    senha: String,
}

async fn register_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let auth_service = AuthService::new(pool);
    let result = auth_service
        .registrar_usuario(payload.nome, payload.email, payload.senha, payload.perfil)
        .await;
    match result {
        Ok(id) => Ok(Json(id)),
        Err(mensagem) => Err((StatusCode::INTERNAL_SERVER_ERROR, mensagem)),
    }
}

async fn delete_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<DeleteRequest>,
) -> Result<Json<u64>, (StatusCode, String)> {
    let auth_service = AuthService::new(pool);
    let result = auth_service.deletar_usuario(payload.id).await;
    match result {
        Ok(id) => Ok(Json(id)),
        Err(mensagem) => Err((StatusCode::NOT_FOUND, mensagem)),
    }
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
        .route("/register", post(register_handler))
        .route("/delete", post(delete_handler))
        .with_state(pool)
}
