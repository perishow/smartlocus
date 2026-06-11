import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ArrowDownToLine, ArrowUpFromLine, Loader2, MapPin } from "lucide-react";
import { toast } from "sonner";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Badge } from "@/components/ui/badge";
import { StatusBadge } from "@/components/StatusBadge";
import { useAuth } from "@/context/AuthContext";
import { adicionarQuantidade, subtrairQuantidade } from "@/features/itens/api";
import { getMovimentacoesPorItem } from "@/features/movimentacao/api";
import { getApiErrorMessage } from "@/lib/api";
import { paraNaiveDateTime } from "@/lib/datas";
import { isCritico, type Item } from "@/types";

type Acao = "Entrada" | "Saída";

interface ItemDrawerProps {
  item: Item | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ItemDrawer({ item, open, onOpenChange }: ItemDrawerProps) {
  const { usuario } = useAuth();
  const queryClient = useQueryClient();
  const [acao, setAcao] = useState<Acao>("Entrada");
  const [quantidade, setQuantidade] = useState("");
  const [observacao, setObservacao] = useState("");

  // Reinicia o formulário quando o item selecionado muda.
  useEffect(() => {
    setAcao("Entrada");
    setQuantidade("");
    setObservacao("");
  }, [item?.id]);

  const historicoQuery = useQuery({
    queryKey: ["movimentacoes", item?.id],
    queryFn: () => getMovimentacoesPorItem(item!.id),
    enabled: open && !!item,
  });

  const mutation = useMutation({
    mutationFn: (variaveis: { acao: Acao }) => {
      const payload = {
        id_item: item!.id,
        quantidade: Number(quantidade),
        data_movimentacao: paraNaiveDateTime(),
        observacao: observacao.trim() || null,
        responsavel_id: usuario!.id,
      };
      return variaveis.acao === "Entrada"
        ? adicionarQuantidade(payload)
        : subtrairQuantidade(payload);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["itens"] });
      queryClient.invalidateQueries({ queryKey: ["itens-criticos"] });
      queryClient.invalidateQueries({ queryKey: ["movimentacoes", item?.id] });
      queryClient.invalidateQueries({ queryKey: ["movimentacoes-hoje"] });
      toast.success("Movimentação registrada!");
      setQuantidade("");
      setObservacao("");
    },
    onError: (error) => toast.error(getApiErrorMessage(error, "Erro ao movimentar estoque")),
  });

  function handleConfirmar() {
    const qtd = Number(quantidade);
    if (!qtd || qtd <= 0) {
      toast.error("Informe uma quantidade válida (maior que zero).");
      return;
    }
    if (acao === "Saída" && item && qtd > item.quantidade_atual) {
      toast.error("Quantidade de saída maior que o estoque atual.");
      return;
    }
    mutation.mutate({ acao });
  }

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent>
        {item && (
          <>
            <SheetHeader>
              <div className="flex items-center justify-between pr-8">
                <SheetTitle>{item.nome}</SheetTitle>
                <StatusBadge item={item} />
              </div>
              <SheetDescription className="flex items-center gap-1">
                <MapPin className="h-3.5 w-3.5" />
                {item.localizacao ?? "Sem localização"}
              </SheetDescription>
            </SheetHeader>

            <div className="grid grid-cols-2 gap-3">
              <div className="rounded-lg border border-border p-3">
                <p className="text-xs text-muted-foreground">Qtd. Atual</p>
                <p className={isCritico(item) ? "text-xl font-bold text-destructive" : "text-xl font-bold"}>
                  {item.quantidade_atual}
                </p>
              </div>
              <div className="rounded-lg border border-border p-3">
                <p className="text-xs text-muted-foreground">Qtd. Mínima</p>
                <p className="text-xl font-bold">{item.quantidade_minima}</p>
              </div>
            </div>

            {/* Ação de movimentação */}
            <div className="space-y-3 rounded-lg border border-border p-4">
              <div className="grid grid-cols-2 gap-2">
                <Button
                  type="button"
                  variant={acao === "Saída" ? "destructive" : "outline"}
                  onClick={() => setAcao("Saída")}
                >
                  <ArrowUpFromLine className="h-4 w-4" />
                  Saída
                </Button>
                <Button
                  type="button"
                  variant={acao === "Entrada" ? "success" : "outline"}
                  onClick={() => setAcao("Entrada")}
                >
                  <ArrowDownToLine className="h-4 w-4" />
                  Entrada
                </Button>
              </div>

              <div className="space-y-1.5">
                <Label htmlFor="quantidade">Quantidade</Label>
                <Input
                  id="quantidade"
                  type="number"
                  min={1}
                  value={quantidade}
                  onChange={(e) => setQuantidade(e.target.value)}
                  placeholder="0"
                />
              </div>

              <div className="space-y-1.5">
                <Label htmlFor="observacao">Observação</Label>
                <Input
                  id="observacao"
                  value={observacao}
                  onChange={(e) => setObservacao(e.target.value)}
                  placeholder="Opcional"
                />
              </div>

              <Button className="w-full" onClick={handleConfirmar} disabled={mutation.isPending}>
                {mutation.isPending && <Loader2 className="h-4 w-4 animate-spin" />}
                Confirmar {acao}
              </Button>
            </div>

            {/* Histórico recente */}
            <div className="space-y-2">
              <p className="text-sm font-semibold">Histórico Recente</p>
              {historicoQuery.isLoading ? (
                <p className="text-sm text-muted-foreground">Carregando…</p>
              ) : historicoQuery.data && historicoQuery.data.length > 0 ? (
                <ul className="space-y-2">
                  {historicoQuery.data.slice(0, 8).map((m) => (
                    <li
                      key={m.id}
                      className="flex items-center justify-between rounded-md border border-border px-3 py-2 text-sm"
                    >
                      <div>
                        <Badge variant={m.tipo === "Entrada" ? "success" : "destructive"}>
                          {m.tipo === "Entrada" ? "IN" : "OUT"} {m.quantidade}
                        </Badge>
                        <p className="mt-1 text-xs text-muted-foreground">
                          {m.responsavel_nome} ·{" "}
                          {m.data_movimentacao.replace("T", " ").slice(0, 16)}
                        </p>
                      </div>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-muted-foreground">Sem movimentações registradas.</p>
              )}
            </div>
          </>
        )}
      </SheetContent>
    </Sheet>
  );
}
