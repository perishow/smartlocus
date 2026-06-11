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
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { registrarItem } from "@/features/itens/api";
import { getApiErrorMessage } from "@/lib/api";

const VAZIO = {
  nome: "",
  categoria: "",
  quantidade_atual: "0",
  quantidade_minima: "0",
  localizacao: "",
};

export function NovoItemDialog() {
  const queryClient = useQueryClient();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState(VAZIO);

  const mutation = useMutation({
    mutationFn: registrarItem,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["itens"] });
      queryClient.invalidateQueries({ queryKey: ["itens-criticos"] });
      toast.success("Item cadastrado com sucesso!");
      setForm(VAZIO);
      setOpen(false);
    },
    onError: (error) => toast.error(getApiErrorMessage(error, "Erro ao cadastrar item")),
  });

  function handleSubmit(e: FormEvent) {
    e.preventDefault();
    if (!form.nome.trim() || !form.categoria.trim()) {
      toast.error("Informe ao menos o nome e a categoria.");
      return;
    }
    mutation.mutate({
      nome: form.nome.trim(),
      categoria: form.categoria.trim(),
      quantidade_atual: Number(form.quantidade_atual) || 0,
      quantidade_minima: Number(form.quantidade_minima) || 0,
      localizacao: form.localizacao.trim(),
    });
  }

  function campo(key: keyof typeof VAZIO) {
    return {
      value: form[key],
      onChange: (e: React.ChangeEvent<HTMLInputElement>) =>
        setForm((f) => ({ ...f, [key]: e.target.value })),
    };
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button>
          <Plus className="h-4 w-4" />
          Novo Item
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Novo Item</DialogTitle>
          <DialogDescription>
            Cadastre um novo insumo no almoxarifado.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-1.5">
            <Label htmlFor="nome">Nome</Label>
            <Input id="nome" placeholder="Ex.: Papel A4" {...campo("nome")} autoFocus />
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="categoria">Categoria</Label>
            <Input id="categoria" placeholder="Ex.: Material de Escritório" {...campo("categoria")} />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1.5">
              <Label htmlFor="qtd">Quantidade Atual</Label>
              <Input id="qtd" type="number" min={0} {...campo("quantidade_atual")} />
            </div>
            <div className="space-y-1.5">
              <Label htmlFor="min">Quantidade Mínima</Label>
              <Input id="min" type="number" min={0} {...campo("quantidade_minima")} />
            </div>
          </div>
          <div className="space-y-1.5">
            <Label htmlFor="loc">Localização</Label>
            <Input id="loc" placeholder="Ex.: Prateleira A1" {...campo("localizacao")} />
          </div>

          <div className="flex justify-end gap-2 pt-2">
            <DialogClose asChild>
              <Button type="button" variant="outline">
                Cancelar
              </Button>
            </DialogClose>
            <Button type="submit" disabled={mutation.isPending}>
              {mutation.isPending && <Loader2 className="h-4 w-4 animate-spin" />}
              Cadastrar
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
