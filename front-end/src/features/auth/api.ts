import { api } from "@/lib/api";
import type { Usuario } from "@/types";

export interface LoginPayload {
  nome: string;
  senha: string;
}

export async function login(payload: LoginPayload): Promise<Usuario> {
  const { data } = await api.post<Usuario>("/auth/login", payload);
  return data;
}
