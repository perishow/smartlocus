use crate::repositorios::movimentacoes_estoque_repository::{
    MovimentacaoComResponsavel, MovimentacaoEstoque, MovimentacaoEstoqueRepository,
};
use sqlx::MySqlPool;

pub struct MovimentacaoService {
    pool: MySqlPool,
}
impl MovimentacaoService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    pub async fn coletar_todas_movimentacoes(self) -> Result<Vec<MovimentacaoEstoque>, String> {
        let repositorio = MovimentacaoEstoqueRepository::new(self.pool.clone());
        let movimentacao_vec = repositorio.get_all().await.map_err(|e| e.to_string())?;
        Ok(movimentacao_vec)
    }

    pub async fn coletar_movimentacoes_por_item(
        self,
        item_id: i32,
    ) -> Result<Vec<MovimentacaoComResponsavel>, String> {
        let repositorio = MovimentacaoEstoqueRepository::new(self.pool.clone());
        let movimentacao_vec = repositorio
            .get_by_item_id(item_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(movimentacao_vec)
    }
}
