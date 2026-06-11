import { NavLink } from "react-router-dom";
import { LayoutDashboard, Boxes, Users, LogOut } from "lucide-react";
import { Logo } from "@/components/Logo";
import { useAuth } from "@/context/AuthContext";
import { cn } from "@/lib/utils";

export function Sidebar() {
  const { logout, isOperador } = useAuth();

  const links = [
    { to: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
    { to: "/estoque", label: "Estoque", icon: Boxes },
    // Gestão de usuários é exclusiva do Operador.
    ...(isOperador ? [{ to: "/usuarios", label: "Usuários", icon: Users }] : []),
  ];

  return (
    <aside className="flex h-full w-[200px] shrink-0 flex-col border-r border-border bg-card">
      <div className="flex h-16 items-center px-5">
        <Logo />
      </div>
      <nav className="flex flex-1 flex-col gap-1 px-3 py-4">
        {links.map(({ to, label, icon: Icon }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              cn(
                "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                isActive
                  ? "bg-primary/10 text-primary"
                  : "text-muted-foreground hover:bg-accent hover:text-foreground"
              )
            }
          >
            <Icon className="h-4 w-4" />
            {label}
          </NavLink>
        ))}
      </nav>
      <button
        onClick={logout}
        className="m-3 flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-destructive/10 hover:text-destructive"
      >
        <LogOut className="h-4 w-4" />
        Sair
      </button>
    </aside>
  );
}
