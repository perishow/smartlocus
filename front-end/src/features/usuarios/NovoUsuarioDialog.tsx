import { useState, type FormEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Plus, Loader2 } from "lucide-react";
import { toast } from "sonner";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogTrigger,
  DialogClose,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAuth } from "@/context/AuthContext";
import { criarUsuario } from "@/features/usuarios/api";
import { getApiErrorMessage } from "@/lib/api";
import type { Perfil } from "@/types";

const VAZIO = { nome: "", email: "", senha: "", perfil: "Consultor" as Perfil };

export function NovoUsuarioDialog() {
  const { usuario } = useAuth();
  const queryClient = useQueryClient();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState(VAZIO);

  const mutation = useMutation({
    mutationFn: criarUsuario,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["usuarios"] });
      toast.success("Usuário criado com sucesso!");
      setForm(VAZIO);
      setOpen(false);
    },
    onError: (error) => toast.error(getApiErrorMessage(error, "Erro ao criar usuário")),
  });

  function handleSubmit(e: FormEvent) {
    e.preventDefault();
    if (!usuario) return;
    if (!form.nome.trim() || !form.email.trim() || !form.senha) {
      toast.error("Preencha nome, email e senha.");
      return;
    }
    mutation.mutate({
      solicitante_id: usuario.id,
      nome: form.nome.trim(),
      email: form.email.trim(),
      senha: form.senha,
      perfil: form.perfil,
    });
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>
          <Plus className="h-4 w-4" />
          Novo Usuário
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Novo Usuário</DialogTitle>
          <DialogDescription>
            Cadastre um novo Operador ou Consultor no sistema.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-1.5">
            <Label htmlFor="u-nome">Nome</Label>
            <Input
              id="u-nome"
              placeholder="Nome completo"
              value={form.nome}
              onChange={(e) => setForm((f) => ({ ...f, nome: e.target.value }))}
              autoFocus
            />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="u-email">Email</Label>
            <Input
              id="u-email"
              type="email"
              placeholder="email@exemplo.com"
              value={form.email}
              onChange={(e) => setForm((f) => ({ ...f, email: e.target.value }))}
            />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="u-senha">Senha</Label>
            <Input
              id="u-senha"
              type="password"
              placeholder="Senha de acesso"
              value={form.senha}
              onChange={(e) => setForm((f) => ({ ...f, senha: e.target.value }))}
              autoComplete="new-password"
            />
          </div>
          <div className="space-y-1.5">
            <Label>Perfil</Label>
            <Select
              value={form.perfil}
              onValueChange={(v) => setForm((f) => ({ ...f, perfil: v as Perfil }))}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="Consultor">Consultor (somente consulta)</SelectItem>
                <SelectItem value="Operador">Operador (controle total)</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="flex justify-end gap-2 pt-2">
            <DialogClose asChild>
              <Button type="button" variant="outline">
                Cancelar
              </Button>
            </DialogClose>
            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending && <Loader2 className="h-4 w-4 animate-spin" />}
              Criar
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
