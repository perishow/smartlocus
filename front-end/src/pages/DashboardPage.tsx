import { useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { format } from "date-fns";
import { ptBR } from "date-fns/locale";
import { AlertTriangle, Boxes, ArrowLeftRight, Eye } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { StatusBadge } from "@/components/StatusBadge";
import { useAuth } from "@/context/AuthContext";
import { getAllItens, getItensCriticos } from "@/features/itens/api";
import { getTodasMovimentacoes } from "@/features/movimentacao/api";

function saudacao(): string {
  const h = new Date().getHours();
  if (h < 12) return "Bom dia";
  if (h < 18) return "Boa tarde";
  return "Boa noite";
}

export function DashboardPage() {
  const navigate = useNavigate();
  const { usuario } = useAuth();

  const criticosQuery = useQuery({ queryKey: ["itens-criticos"], queryFn: getItensCriticos });
  const itensQuery = useQuery({ queryKey: ["itens"], queryFn: getAllItens });
  const movsQuery = useQuery({
    queryKey: ["movimentacoes-hoje"],
    queryFn: getTodasMovimentacoes,
  });

  const hoje = format(new Date(), "yyyy-MM-dd");
  const movimentacoesHoje =
    movsQuery.data?.filter((m) => m.data_movimentacao.slice(0, 10) === hoje).length ?? 0;

  const primeiroNome = usuario?.nome.split(" ")[0] ?? "";

  const metricas = [
    {
      label: "Materiais Críticos",
      valor: criticosQuery.data?.length ?? 0,
      icon: AlertTriangle,
      destaque: true,
    },
    {
      label: "Total de Itens",
      valor: itensQuery.data?.length ?? 0,
      icon: Boxes,
      destaque: false,
    },
    {
      label: "Movimentações Hoje",
      valor: movimentacoesHoje,
      icon: ArrowLeftRight,
      destaque: false,
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">
          {saudacao()}, {primeiroNome}!
        </h1>
        <p className="text-sm capitalize text-muted-foreground">
          {format(new Date(), "EEEE, dd 'de' MMMM 'de' yyyy", { locale: ptBR })}
        </p>
      </div>

      {/* Cards de métrica */}
      <div className="grid gap-4 sm:grid-cols-3">
        {metricas.map((m) => (
          <Card key={m.label}>
            <CardContent className="flex items-center justify-between p-6">
              <div>
                <p
                  className={
                    m.destaque
                      ? "text-4xl font-bold text-destructive"
                      : "text-4xl font-bold"
                  }
                >
                  {String(m.valor).padStart(2, "0")}
                </p>
                <p className="mt-1 text-sm text-muted-foreground">{m.label}</p>
              </div>
              <m.icon
                className={
                  m.destaque ? "h-8 w-8 text-destructive" : "h-8 w-8 text-primary"
                }
              />
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Tabela de alertas críticos */}
      <Card>
        <CardContent className="p-0">
          <div className="border-b border-border px-6 py-4">
            <h2 className="font-semibold">Materiais em Alerta Crítico</h2>
          </div>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Item</TableHead>
                <TableHead className="w-20">Qtd</TableHead>
                <TableHead className="w-20">Min</TableHead>
                <TableHead className="w-28">Status</TableHead>
                <TableHead className="w-20 text-right">Ação</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {criticosQuery.isLoading ? (
                <TableRow>
                  <TableCell colSpan={5} className="py-8 text-center text-muted-foreground">
                    Carregando…
                  </TableCell>
                </TableRow>
              ) : (criticosQuery.data?.length ?? 0) === 0 ? (
                <TableRow>
                  <TableCell colSpan={5} className="py-8 text-center text-muted-foreground">
                    Nenhum material em alerta crítico. 🎉
                  </TableCell>
                </TableRow>
              ) : (
                criticosQuery.data!.slice(0, 5).map((item) => (
                  <TableRow key={item.id}>
                    <TableCell className="font-medium">{item.nome}</TableCell>
                    <TableCell>{item.quantidade_atual}</TableCell>
                    <TableCell>{item.quantidade_minima}</TableCell>
                    <TableCell>
                      <StatusBadge item={item} />
                    </TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => navigate(`/estoque?item=${item.id}`)}
                      >
                        <Eye className="h-4 w-4" />
                        Ver
                      </Button>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
