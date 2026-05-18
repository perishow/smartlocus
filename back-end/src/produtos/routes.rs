use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use sqlx::MySqlPool;
use super::service::ProdutoService;
use super::repository::{ Produto, EstoqueQuantizavel };

// 1. Nossa função "Handler" que responde à requisição web
// Ela recebe o State(pool) injetado pelo Axum magicamente
async fn listar_produtos_handler(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Produto>>, (StatusCode, String)> {
    
    // Instanciamos o serviço
    let produto_service = ProdutoService::new(pool);

    // Chamamos a lógica de negócio
    match produto_service.listar_produtos().await {
        Ok(produtos) => Ok(Json(produtos)), // Devolve Status 200 com o JSON
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro no banco de dados: {}", e)
        )),
    }
}

async fn listar_nome_produtos_handler(
    State(pool): State<MySqlPool>,
    ) -> Result<Json<Vec<String>>,(StatusCode, String)> {

    let produto_service = ProdutoService::new(pool);

    match produto_service.listar_nome_produtos().await {
        Ok(nomes_produtos) => Ok(Json(nomes_produtos)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro no banco de dados: {}", e)
        )),
    }
}


async fn listar_estoque_quantizavel_handler(
    State(pool): State<MySqlPool>,
    ) -> Result<Json<Vec<EstoqueQuantizavel>>,(StatusCode, String)> {
    let produto_service = ProdutoService::new(pool);
    
    match produto_service.listar_estoque_quantizavel().await {
        Ok(estoque_quantizavel) => Ok(Json(estoque_quantizavel)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro no banco de dados: {}", e)
        )),
    }
}

// 2. Exportamos a configuração do Router deste módulo
pub fn router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(listar_produtos_handler)) // Rota GET "/"
        .route("/nomes", get(listar_nome_produtos_handler))
        .route("/estoque-quantizavel", get(listar_estoque_quantizavel_handler))
        .with_state(pool) // Passamos o pool de conexões para dentro do Axum
}
