use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, FromRow, Serialize)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub perfil: String,
}

#[derive(Clone)]
pub struct UsuariosRepository {
    pool: MySqlPool,
}
impl UsuariosRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_usuarios(&self) -> Result<Vec<Usuario>, sqlx::Error> {
        let usuarios = sqlx::query_as::<_, Usuario>("SELECT * FROM Usuarios")
            .fetch_all(&self.pool)
            .await?;

        Ok(usuarios)
    }

    pub async fn get_usuario_by_id(&self, id: i32) -> Result<Option<Usuario>, sqlx::Error> {
        // Usamos query_as para mapear o resultado diretamente para a struct Usuario
        let usuario = sqlx::query_as::<_, Usuario>("SELECT * FROM Usuarios WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(usuario)
    }

    pub async fn get_usuario_by_nome(&self, nome: String) -> Result<Option<Usuario>, sqlx::Error> {
        let usuario = sqlx::query_as::<_, Usuario>("SELECT * FROM Usuarios WHERE nome = ?")
            .bind(nome)
            .fetch_optional(&self.pool)
            .await?;

        Ok(usuario)
    }

    pub async fn insert_usuario(
        &self,
        nome: &str,
        email: &str,
        senha: &str,
        perfil: &str,
    ) -> Result<u64, sqlx::Error> {
        // Usamos sqlx::query (sem o _as) porque não estamos esperando retornar linhas,
        // mas sim executar uma ação de modificação no banco.
        let result =
            sqlx::query("INSERT INTO Usuarios (nome, email, senha, perfil) VALUES (?, ?, ?, ?)")
                .bind(nome)
                .bind(email)
                .bind(senha)
                .bind(perfil) // O sqlx sabe como converter isso graças ao #[derive(Type)]
                .execute(&self.pool)
                .await?;

        // O MySQL/MariaDB retorna o ID gerado pelo auto_increment.
        // É muito útil retornar esse valor caso precise dele logo após a inserção.
        Ok(result.last_insert_id())
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM Usuarios WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
