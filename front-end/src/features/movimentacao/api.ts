import { api } from "@/lib/api";
import type { Movimentacao } from "@/types";

export async function getMovimentacoesPorItem(itemId: number): Promise<Movimentacao[]> {
  const { data } = await api.get<Movimentacao[]>(`/movimentacao/get-by-item/${itemId}`);
  return data;
}

export async function getTodasMovimentacoes(): Promise<Movimentacao[]> {
  const { data } = await api.get<Movimentacao[]>("/movimentacao/get-all");
  return data;
}
