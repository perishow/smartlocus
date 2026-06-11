import { useState, type FormEvent } from "react";
import { useNavigate } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { User, Lock, Loader2 } from "lucide-react";
import { toast } from "sonner";
import { Logo } from "@/components/Logo";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useAuth } from "@/context/AuthContext";
import { login as loginRequest } from "@/features/auth/api";
import { getApiErrorMessage } from "@/lib/api";

export function LoginPage() {
  const navigate = useNavigate();
  const { login } = useAuth();
  const [nome, setNome] = useState("");
  const [senha, setSenha] = useState("");
  const [manterConectado, setManterConectado] = useState(true);

  const mutation = useMutation({
    mutationFn: loginRequest,
    onSuccess: (usuario) => {
      login(usuario, manterConectado);
      toast.success(`Bem-vindo, ${usuario.nome.split(" ")[0]}!`);
      navigate("/dashboard", { replace: true });
    },
    onError: (error) => {
      toast.error(getApiErrorMessage(error, "Usuário ou senha incorretos"));
    },
  });

  function handleSubmit(e: FormEvent) {
    e.preventDefault();
    if (!nome.trim() || !senha) return;
    mutation.mutate({ nome: nome.trim(), senha });
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-gradient-to-br from-slate-100 to-slate-200 px-4">
      <div className="w-full max-w-sm">
        <div className="mb-8 flex justify-center">
          <Logo className="scale-125" />
        </div>

        <div className="rounded-2xl border border-border bg-card p-8 shadow-lg">
          <h1 className="mb-6 text-xl font-semibold">Acessar Sistema</h1>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="relative">
              <User className="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                className="pl-9"
                placeholder="Usuário"
                value={nome}
                onChange={(e) => setNome(e.target.value)}
                autoFocus
                autoComplete="username"
              />
            </div>

            <div className="relative">
              <Lock className="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                className="pl-9"
                type="password"
                placeholder="Senha"
                value={senha}
                onChange={(e) => setSenha(e.target.value)}
                autoComplete="current-password"
              />
            </div>

            <label className="flex items-center gap-2 text-sm text-muted-foreground">
              <input
                type="checkbox"
                className="h-4 w-4 rounded border-input accent-[#2563eb]"
                checked={manterConectado}
                onChange={(e) => setManterConectado(e.target.checked)}
              />
              Manter conectado?
            </label>

            <Button type="submit" className="w-full" disabled={mutation.isPending}>
              {mutation.isPending && <Loader2 className="h-4 w-4 animate-spin" />}
              ENTRAR
            </Button>

            <div className="text-center">
              <button
                type="button"
                className="text-sm text-primary hover:underline"
                onClick={() => toast.info("Contate o administrador do almoxarifado.")}
              >
                Esqueci minha senha?
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
