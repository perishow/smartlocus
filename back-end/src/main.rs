use axum::Router;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::process::Command;
use tokio::net::TcpListener;

mod autenticacao;
mod repositorios;
mod usuarios;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Verificando/Iniciando banco de dados via docker...");

    // 1. Iniciamos o Docker apenas UMA vez
    let docker_status = Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg("../docker-compose.yaml")
        .arg("up")
        .arg("-d")
        .status()
        .expect("Falha crítica: O executável do Docker não foi encontrado no sistema.");

    if !docker_status.success() {
        eprintln!(
            "⚠️ Aviso: Houve um problema ao executar o docker compose. O banco pode não estar disponível."
        );
    }

    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não configurada");

    // 2. Loop de tentativas PARA A CONEXÃO COM O BANCO DE DADOS
    let mut pool = None;
    let max_tentativas = 10;

    for i in 1..=max_tentativas {
        println!("Tentativa {} de conectar ao MariaDB...", i);

        match MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
        {
            Ok(p) => {
                pool = Some(p);
                println!("✅ Conexão com o MariaDB estabelecida com sucesso!");
                break; // Deu certo, saímos do loop!
            }
            Err(e) => {
                eprintln!(
                    "⏳ Banco ainda não está pronto: {}. Aguardando 3 segundos...",
                    e
                );
                if i < max_tentativas {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                }
            }
        }
    }

    // 3. Verifica se conseguimos o "pool" após as tentativas
    let pool = pool.expect(
        "❌ Não foi possível conectar ao banco de dados após várias tentativas. Abortando...",
    );

    // 4. Juntamos as rotas e subimos o servidor
    let app = Router::new().nest("/api/v1", autenticacao::routes::router(pool.clone()));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Servidor rodando em http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
