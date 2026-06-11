import { createContext, useContext, useEffect, useState, type ReactNode } from "react";
import type { Usuario } from "@/types";

const STORAGE_KEY = "smartlocus.usuario";

interface AuthContextValue {
  usuario: Usuario | null;
  login: (usuario: Usuario, persistir: boolean) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

function carregarUsuario(): Usuario | null {
  const raw =
    localStorage.getItem(STORAGE_KEY) ?? sessionStorage.getItem(STORAGE_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as Usuario;
  } catch {
    return null;
  }
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [usuario, setUsuario] = useState<Usuario | null>(() => carregarUsuario());

  useEffect(() => {
    if (!usuario) {
      localStorage.removeItem(STORAGE_KEY);
      sessionStorage.removeItem(STORAGE_KEY);
    }
  }, [usuario]);

  function login(novoUsuario: Usuario, persistir: boolean) {
    const store = persistir ? localStorage : sessionStorage;
    store.setItem(STORAGE_KEY, JSON.stringify(novoUsuario));
    // Garante que não fique duplicado no outro storage.
    (persistir ? sessionStorage : localStorage).removeItem(STORAGE_KEY);
    setUsuario(novoUsuario);
  }

  function logout() {
    setUsuario(null);
  }

  return (
    <AuthContext.Provider value={{ usuario, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth deve ser usado dentro de <AuthProvider>");
  return ctx;
}
