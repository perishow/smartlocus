import { format } from "date-fns";

/**
 * Formata um Date para o formato NaiveDateTime esperado pelo backend Rust
 * (chrono), SEM sufixo de timezone — ex.: "2026-06-10T14:30:00".
 * NÃO usar toISOString(), pois acrescenta o "Z".
 */
export function paraNaiveDateTime(date: Date = new Date()): string {
  return format(date, "yyyy-MM-dd'T'HH:mm:ss");
}
