import { api } from "@/lib/api";
import type { Perfil, Usuario } from "@/types";

export async function listarUsuarios(): Promise<Usuario[]> {
  const { data } = await api.get<Usuario[]>("/auth/users");
  return data;
}

export interface CriarUsuarioPayload {
  /** ID do Operador que está criando o usuário (validado no backend). */
  solicitante_id: number;
  nome: string;
  email: string;
  senha: string;
  perfil: Perfil;
}

export async function criarUsuario(payload: CriarUsuarioPayload): Promise<number> {
  const { data } = await api.post<number>("/auth/create-user", payload);
  return data;
}
