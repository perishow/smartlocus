import { Navigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { NovoUsuarioDialog } from "@/features/usuarios/NovoUsuarioDialog";
import { listarUsuarios } from "@/features/usuarios/api";
import { useAuth } from "@/context/AuthContext";

export function UsuariosPage() {
  const { isOperador } = useAuth();

  const { data: usuarios, isLoading, isError } = useQuery({
    queryKey: ["usuarios"],
    queryFn: listarUsuarios,
    enabled: isOperador,
  });

  // Consultores não têm acesso a esta página.
  if (!isOperador) return <Navigate to="/dashboard" replace />;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Usuários</h1>
          <p className="text-sm text-muted-foreground">
            Gerencie os Operadores e Consultores do sistema.
          </p>
        </div>
        <NovoUsuarioDialog />
      </div>

      <div className="rounded-xl border border-border bg-card">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-16">ID</TableHead>
              <TableHead>Nome</TableHead>
              <TableHead>Email</TableHead>
              <TableHead className="w-32">Perfil</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading ? (
              <TableRow>
                <TableCell colSpan={4} className="py-10 text-center text-muted-foreground">
                  <Loader2 className="mx-auto h-5 w-5 animate-spin" />
                </TableCell>
              </TableRow>
            ) : isError ? (
              <TableRow>
                <TableCell colSpan={4} className="py-10 text-center text-destructive">
                  Erro ao carregar usuários. Verifique se o backend está rodando.
                </TableCell>
              </TableRow>
            ) : (usuarios?.length ?? 0) === 0 ? (
              <TableRow>
                <TableCell colSpan={4} className="py-10 text-center text-muted-foreground">
                  Nenhum usuário cadastrado.
                </TableCell>
              </TableRow>
            ) : (
              usuarios!.map((u) => (
                <TableRow key={u.id}>
                  <TableCell className="font-mono text-muted-foreground">
                    {String(u.id).padStart(3, "0")}
                  </TableCell>
                  <TableCell className="font-medium">{u.nome}</TableCell>
                  <TableCell className="text-muted-foreground">{u.email}</TableCell>
                  <TableCell>
                    <Badge variant={u.perfil === "Operador" ? "default" : "outline"}>
                      {u.perfil}
                    </Badge>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
