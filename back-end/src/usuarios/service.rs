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

    pub async fn coletar_usuario_por_id(&self, id: i32) -> Result<Usuario ,sqlx::Error> {
        let repository = UsuariosRepository::new(self.pool.clone());
        let resultado_query: Option<Usuario> = repository.get_usuario_by_id(id).await?;
        
        match resultado_query {
            Some(usuario) => Ok(usuario),
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn validar_usuario_e_senha(&self, nome: String, senha_fornecida: String) -> Result<Usuario, String> {
        
        println!("Tentativa de login!");

        let repository = UsuariosRepository::new(self.pool.clone());
        let resultado_query = repository.get_usuario_by_nome(nome).await;
        let usuario = match resultado_query {
            Ok(Some(usuario)) => usuario,
            Ok(None) => return Err("Usuário ou senha incorretos".to_string()),
            Err(_) => return Err("Erro interno no servidor".to_string()),
        };
        
        if usuario.senha == senha_fornecida {
            Ok(usuario)
        } else {
            Err("Usuario ou senha incorretos".to_string())
        }
    }
}
