import { useAuth } from "@/context/AuthContext";

export function Header() {
  const { usuario } = useAuth();
  const iniciais = usuario?.nome
    ?.split(" ")
    .slice(0, 2)
    .map((p) => p[0])
    .join("")
    .toUpperCase();

  return (
    <header className="flex h-16 shrink-0 items-center justify-end border-b border-border bg-card px-6">
      <div className="flex items-center gap-3">
        <div className="text-right">
          <p className="text-sm font-medium leading-tight">{usuario?.nome}</p>
          <p className="text-xs text-muted-foreground">{usuario?.perfil}</p>
        </div>
        <div className="flex h-9 w-9 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary">
          {iniciais}
        </div>
      </div>
    </header>
  );
}
