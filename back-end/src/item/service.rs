use crate::repositorios::{
    itens_repository::{Item, ItensRepository},
    movimentacoes_estoque_repository::MovimentacaoEstoqueRepository,
};
use chrono::NaiveDateTime;
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
            .insert(
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

    pub async fn deletar_item(&self, id: i32) -> Result<u64, String> {
        let repositorio = ItensRepository::new(self.pool.clone());
        let id_item = repositorio.delete(id).await.map_err(|e| e.to_string())?;

        Ok(id_item)
    }

    pub async fn get_all_items(&self) -> Result<Vec<Item>, String> {
        let repositorio = ItensRepository::new(self.pool.clone());
        let item_vec = repositorio.get_all().await.map_err(|e| e.to_string())?;
        Ok(item_vec)
    }

    pub async fn get_all_quantidade_critica(&self) -> Result<Vec<Item>, String> {
        let repositorio = ItensRepository::new(self.pool.clone());
        let item_vec = repositorio
            .get_all_quantidade_critica()
            .await
            .map_err(|e| e.to_string())?;
        Ok(item_vec)
    }

    pub async fn adicionar_quantidade(
        &self,
        id_item: i32,
        quant_somar: i32,
        data_movimentacao: NaiveDateTime,
        observacao: Option<String>,
        responsavel_id: i32,
    ) -> Result<i32, String> {
        if quant_somar < 0 {
            return Err(format!(
                "Erro: A quantidade a ser somada não pode ser negativa! quantidade inserida: {}",
                quant_somar
            ));
        }
        let repositorio_item = ItensRepository::new(self.pool.clone());
        let repositorio_movimentacoes = MovimentacaoEstoqueRepository::new(self.pool.clone());
        let item = repositorio_item
            .get(id_item)
            .await
            .map_err(|e| e.to_string())?;

        let quantidade_atual = match item {
            Some(item) => item.quantidade_atual,
            None => return Err(format!("Erro: Item com ID {} não foi encontrado", id_item)),
        };
        let nova_quantidade = quantidade_atual + quant_somar;
        let result_item = repositorio_item
            .update_quantidade(id_item, nova_quantidade)
            .await
            .map_err(|e| e.to_string())?;

        //registro da movimentação:
        let resultado_movimentacao = repositorio_movimentacoes
            .insert(
                id_item,
                String::from("Entrada"),
                quant_somar,
                data_movimentacao,
                observacao,
                responsavel_id,
            )
            .await;
        match resultado_movimentacao {
            Ok(id) => {
                println!("Movimentacao Criada com sucesso, id = {}", id);
            }
            Err(mensagem) => return Err(format!("Erro ao criar movimentação: {}", mensagem)),
        }

        Ok(result_item as i32)
    }

    pub async fn subtrair_quantidade(
        &self,
        id_item: i32,
        quant_subtrair: i32,
        data_movimentacao: NaiveDateTime,
        observacao: Option<String>,
        responsavel_id: i32,
    ) -> Result<u64, String> {
        if quant_subtrair < 0 {
            return Err(format!(
                "Erro: A quantidade a ser removida do estoque não pode ser negativa! Valor inserido: {}",
                quant_subtrair
            ));
        }
        let repositorio_item = ItensRepository::new(self.pool.clone());
        let repositorio_movimentacoes = MovimentacaoEstoqueRepository::new(self.pool.clone());

        let item = repositorio_item
            .get(id_item)
            .await
            .map_err(|e| e.to_string())?;

        let quantidade_atual = match item {
            Some(item) => item.quantidade_atual,
            None => return Err(format!("Erro: Item com ID {} não foi encontrado", id_item)),
        };
        let nova_quantidade = quantidade_atual - quant_subtrair;
        if nova_quantidade < 0 {
            return Err(format!(
                "Erro: A quantidade final não pode ser negativa [ quantidade final = {}]",
                nova_quantidade
            ));
        }
        let result = repositorio_item
            .update_quantidade(id_item, nova_quantidade)
            .await
            .map_err(|e| e.to_string())?;

        //registro da movimentação:
        let resultado_movimentacao = repositorio_movimentacoes
            .insert(
                id_item,
                String::from("Saída"),
                quant_subtrair,
                data_movimentacao,
                observacao,
                responsavel_id,
            )
            .await;
        match resultado_movimentacao {
            Ok(id) => {
                println!("Movimentacao Criada com sucesso, id = {}", id);
            }
            Err(mensagem) => return Err(format!("Erro ao criar movimentação: {}", mensagem)),
        }

        Ok(result)
    }
}
