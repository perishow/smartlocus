use sqlx::MySqlPool;
use super::repository::{ UsuariosRepository, Usuario };

pub struct UsuariosService {
    pool: MySqlPool,
}

impl UsuariosService {

    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn listar_usuarios(&self) -> Result <Vec<Usuario>, sqlx::Error> {
        let repository = UsuariosRepository::new(self.pool.clone());

        let usuarios = repository.get_usuarios().await?;

        Ok(usuarios)
    }
}
