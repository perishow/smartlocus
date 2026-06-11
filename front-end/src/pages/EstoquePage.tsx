import { useEffect, useMemo, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Search, Loader2, ChevronLeft, ChevronRight } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { StatusBadge } from "@/components/StatusBadge";
import { NovoItemDialog } from "@/features/itens/NovoItemDialog";
import { ItemDrawer } from "@/features/itens/ItemDrawer";
import { getAllItens } from "@/features/itens/api";
import { useAuth } from "@/context/AuthContext";
import { isCritico, type Item } from "@/types";

const POR_PAGINA = 6;
const TODAS = "__todas__";

export function EstoquePage() {
  const { isOperador } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const [busca, setBusca] = useState("");
  const [categoria, setCategoria] = useState<string>(TODAS);
  const [apenasCriticos, setApenasCriticos] = useState(false);
  const [pagina, setPagina] = useState(1);
  const [itemSelecionado, setItemSelecionado] = useState<Item | null>(null);
  const [drawerAberto, setDrawerAberto] = useState(false);

  const { data: itens, isLoading, isError } = useQuery({
    queryKey: ["itens"],
    queryFn: getAllItens,
  });

  const categorias = useMemo(
    () => Array.from(new Set((itens ?? []).map((i) => i.categoria))).sort(),
    [itens]
  );

  const filtrados = useMemo(() => {
    return (itens ?? []).filter((item) => {
      const correspondeBusca = item.nome.toLowerCase().includes(busca.toLowerCase());
      const correspondeCategoria = categoria === TODAS || item.categoria === categoria;
      const correspondeCritico = !apenasCriticos || isCritico(item);
      return correspondeBusca && correspondeCategoria && correspondeCritico;
    });
  }, [itens, busca, categoria, apenasCriticos]);

  const totalPaginas = Math.max(1, Math.ceil(filtrados.length / POR_PAGINA));
  const paginaAtual = Math.min(pagina, totalPaginas);
  const visiveis = filtrados.slice(
    (paginaAtual - 1) * POR_PAGINA,
    paginaAtual * POR_PAGINA
  );

  // Volta à primeira página sempre que os filtros mudam.
  useEffect(() => setPagina(1), [busca, categoria, apenasCriticos]);

  // Deep-link vindo do Dashboard ("Ver"): ?item=ID abre o drawer.
  useEffect(() => {
    const idParam = searchParams.get("item");
    if (idParam && itens) {
      const alvo = itens.find((i) => i.id === Number(idParam));
      if (alvo) {
        abrirDrawer(alvo);
        searchParams.delete("item");
        setSearchParams(searchParams, { replace: true });
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [itens, searchParams]);

  function abrirDrawer(item: Item) {
    setItemSelecionado(item);
    setDrawerAberto(true);
  }

  // Mantém o item do drawer sincronizado com os dados recém-carregados.
  const itemAtual = useMemo(
    () => itens?.find((i) => i.id === itemSelecionado?.id) ?? itemSelecionado,
    [itens, itemSelecionado]
  );

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Controle de Almoxarifado</h1>
      </div>

      {/* Barra de ferramentas */}
      <div className="flex flex-wrap items-center gap-3">
        <div className="relative min-w-[200px] flex-1">
          <Search className="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            className="pl-9"
            placeholder="Buscar item…"
            value={busca}
            onChange={(e) => setBusca(e.target.value)}
          />
        </div>

        <Select value={categoria} onValueChange={setCategoria}>
          <SelectTrigger className="w-[200px]">
            <SelectValue placeholder="Filtrar por Categoria" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={TODAS}>Todas as Categorias</SelectItem>
            {categorias.map((c) => (
              <SelectItem key={c} value={c}>
                {c}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <label className="flex items-center gap-2 text-sm">
          <Switch checked={apenasCriticos} onCheckedChange={setApenasCriticos} />
          Mostrar apenas Críticos
        </label>

        {isOperador && <NovoItemDialog />}
      </div>

      {/* Tabela */}
      <div className="rounded-xl border border-border bg-card">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-16">ID</TableHead>
              <TableHead>Item</TableHead>
              <TableHead className="w-20">Qtd</TableHead>
              <TableHead className="w-20">Min</TableHead>
              <TableHead className="w-28">Status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading ? (
              <TableRow>
                <TableCell colSpan={5} className="py-10 text-center text-muted-foreground">
                  <Loader2 className="mx-auto h-5 w-5 animate-spin" />
                </TableCell>
              </TableRow>
            ) : isError ? (
              <TableRow>
                <TableCell colSpan={5} className="py-10 text-center text-destructive">
                  Erro ao carregar itens. Verifique se o backend está rodando.
                </TableCell>
              </TableRow>
            ) : visiveis.length === 0 ? (
              <TableRow>
                <TableCell colSpan={5} className="py-10 text-center text-muted-foreground">
                  Nenhum item encontrado.
                </TableCell>
              </TableRow>
            ) : (
              visiveis.map((item) => (
                <TableRow
                  key={item.id}
                  className="cursor-pointer"
                  onClick={() => abrirDrawer(item)}
                >
                  <TableCell className="font-mono text-muted-foreground">
                    {String(item.id).padStart(3, "0")}
                  </TableCell>
                  <TableCell className="font-medium">{item.nome}</TableCell>
                  <TableCell>{item.quantidade_atual}</TableCell>
                  <TableCell>{item.quantidade_minima}</TableCell>
                  <TableCell>
                    <StatusBadge item={item} />
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Paginação */}
      <div className="flex items-center justify-between text-sm text-muted-foreground">
        <Button
          variant="ghost"
          size="sm"
          disabled={paginaAtual <= 1}
          onClick={() => setPagina((p) => p - 1)}
        >
          <ChevronLeft className="h-4 w-4" />
          Anterior
        </Button>
        <span>
          Pág {paginaAtual}/{totalPaginas}
        </span>
        <Button
          variant="ghost"
          size="sm"
          disabled={paginaAtual >= totalPaginas}
          onClick={() => setPagina((p) => p + 1)}
        >
          Próxima
          <ChevronRight className="h-4 w-4" />
        </Button>
      </div>

      <ItemDrawer item={itemAtual} open={drawerAberto} onOpenChange={setDrawerAberto} />
    </div>
  );
}
