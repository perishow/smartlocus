use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use axum::Router;

mod produtos;
mod usuarios;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "mysql://root:senha@localhost:3306/smartlocus";

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Conexão com o MariaDB estabelecida com sucesso!");

    // 1. Juntamos as rotas do nosso módulo "produtos"
    // O ".nest" faz com que todas as rotas daquele arquivo comecem com "/produtos"
    let app = Router::new()
        .nest("/api/v1/produtos", produtos::routes::router(pool.clone()))
        .nest("/api/v1/usuarios", usuarios::routes::router(pool.clone()));

    // 2. Definimos a porta em que o servidor vai rodar (ex: 3000)
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Servidor rodando em http://localhost:3000");

    // 3. Rodamos o servidor infinitamente!
    axum::serve(listener, app).await?;

    Ok(())
}
