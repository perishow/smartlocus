// Tipos espelhando o JSON (snake_case) retornado pelo backend Rust.

export type Perfil = "Operador" | "Consultor";

export interface Usuario {
  id: number;
  nome: string;
  email: string;
  perfil: Perfil;
}

export interface Item {
  id: number;
  nome: string;
  categoria: string;
  quantidade_atual: number;
  quantidade_minima: number;
  localizacao: string | null;
}

export type TipoMovimentacao = "Entrada" | "Saída";

export interface Movimentacao {
  id: number;
  item_id: number;
  tipo: TipoMovimentacao;
  quantidade: number;
  data_movimentacao: string;
  observacao: string | null;
  responsavel_id: number;
  responsavel_nome: string;
}

/** RF-002: um item é CRÍTICO quando a quantidade atual <= quantidade mínima. */
export function isCritico(item: Item): boolean {
  return item.quantidade_atual <= item.quantidade_minima;
}
