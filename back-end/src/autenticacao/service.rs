use crate::repositorios::usuarios_repository::{Usuario, UsuariosRepository};
use sqlx::MySqlPool;

pub struct AuthService {
    pool: MySqlPool,
}
impl AuthService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn validar_usuario_e_senha(
        &self,
        nome: String,
        senha_fornecida: String,
    ) -> Result<Usuario, String> {
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

    pub async fn registrar_usuario(
        &self,
        nome: String,
        email: String,
        senha: String,
        perfil: String,
    ) -> Result<u64, String> {
        println!("Requisição de cadastro recebida!");

        let repository = UsuariosRepository::new(self.pool.clone());
        let id_usuario = repository
            .insert_usuario(&nome, &email, &senha, &perfil)
            .await
            .map_err(|e| e.to_string())?;
        println!("Cadastro concluído, id do usuário: {}", id_usuario);
        Ok(id_usuario)
    }
}
