import axios from "axios";

// Cliente HTTP central. baseURL vem do .env (em dev: /api/v1 via proxy do Vite).
export const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL ?? "/api/v1",
  headers: { "Content-Type": "application/json" },
});

/** Extrai uma mensagem de erro legível de uma falha do axios. */
export function getApiErrorMessage(error: unknown, fallback = "Erro inesperado"): string {
  if (axios.isAxiosError(error)) {
    const data = error.response?.data;
    if (typeof data === "string" && data.trim()) return data;
    if (data && typeof data === "object" && "message" in data) {
      return String((data as { message: unknown }).message);
    }
    return error.message || fallback;
  }
  return fallback;
}
