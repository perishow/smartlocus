use crate::repositorios::itens_repository::ItensRepository;
use sqlx::MySqlPool;

pub struct ItemService {
    pool: MySqlPool,
}
impl ItemService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn inserir_novo_item(
        &self,
        nome: String,
        categoria: String,
        quantidade_atual: i32,
        quantidade_minima: i32,
        localizacao: String,
    ) -> Result<u64, String> {
        let repositorio = ItensRepository::new(self.pool.clone());
        // checa se já existe o item no repositorio.

        let id_item = repositorio
            .insert_item(
                nome,
                categoria,
                quantidade_atual,
                quantidade_minima,
                localizacao,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(id_item)
    }

    pub async fn adicionar_quantidade_item(
        &self,
        quant_somar: i32,
        id: u64,
    ) -> Result<u64, String> {
        if quant_somar < 0 {
            return Err(format!(
                "Erro: A quantidade a ser somada não pode ser negativa! quantidade inserida: {}",
                quant_somar
            ));
        }
        let repositorio = ItensRepository::new(self.pool.clone());
        let item = repositorio
            .get_item_by_id(id)
            .await
            .map_err(|e| e.to_string())?;

        let quantidade_atual = match item {
            Some(item) => item.quantidade_atual,
            None => return Err(format!("Erro: Item com ID {} não foi encontrado", id)),
        };
        let nova_quantidade = quantidade_atual + quant_somar;
        let result = repositorio
            .update_item_quantidade(id, nova_quantidade)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    pub async fn subtrair_quantidade_item(
        &self,
        quant_subtrair: i32,
        id: u64,
    ) -> Result<u64, String> {
        if quant_subtrair < 0 {
            return Err(format!(
                "Erro: A quantidade a ser removida do estoque não pode ser negativa! Valor inserido: {}",
                quant_subtrair
            ));
        }
        let repositorio = ItensRepository::new(self.pool.clone());
        let item = repositorio
            .get_item_by_id(id)
            .await
            .map_err(|e| e.to_string())?;

        let quantidade_atual = match item {
            Some(item) => item.quantidade_atual,
            None => return Err(format!("Erro: Item com ID {} não foi encontrado", id)),
        };
        let nova_quantidade = quantidade_atual - quant_subtrair;
        if nova_quantidade < 0 {
            return Err(format!(
                "Erro: A quantidade final não pode ser negativa [ quantidade final = {}]",
                nova_quantidade
            ));
        }

        let result = repositorio
            .update_item_quantidade(id, nova_quantidade)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result)
    }
}
