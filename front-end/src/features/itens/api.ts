import { api } from "@/lib/api";
import type { Item } from "@/types";

export async function getAllItens(): Promise<Item[]> {
  const { data } = await api.get<Item[]>("/item/get-all");
  return data;
}

export async function getItensCriticos(): Promise<Item[]> {
  const { data } = await api.get<Item[]>("/item/get-all-quantidade-critica");
  return data;
}

export interface NovoItemPayload {
  nome: string;
  categoria: string;
  quantidade_atual: number;
  quantidade_minima: number;
  localizacao: string;
}

export async function registrarItem(payload: NovoItemPayload): Promise<number> {
  const { data } = await api.post<number>("/item/register-item", payload);
  return data;
}

export async function deletarItem(id: number): Promise<number> {
  const { data } = await api.post<number>("/item/delete-item", { id });
  return data;
}

export interface MovimentarPayload {
  id_item: number;
  quantidade: number;
  data_movimentacao: string;
  observacao?: string | null;
  responsavel_id: number;
}

export async function adicionarQuantidade(payload: MovimentarPayload): Promise<number> {
  const { data } = await api.post<number>("/item/adicionar-quantidade", payload);
  return data;
}

export async function subtrairQuantidade(payload: MovimentarPayload): Promise<number> {
  const { data } = await api.post<number>("/item/subtrair-quantidade", payload);
  return data;
}
