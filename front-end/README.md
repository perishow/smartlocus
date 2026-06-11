# SmartLocus — Frontend

Interface web do MVP de almoxarifado do SmartLocus, em **React + TypeScript + Vite +
Tailwind CSS + shadcn/ui** (componentes Radix). Consome o backend Rust (Axum) em
`http://localhost:3000`.

## Pré-requisitos

- Node.js 18+ e npm
- Backend rodando (`cargo run` na pasta `../back-end`) — ele também sobe o MariaDB via Docker.

## Como rodar

```bash
npm install
npm run dev      # Vite em http://localhost:5173
```

Em desenvolvimento, o Vite faz proxy de `/api` → `http://localhost:3000` (ver
`vite.config.ts`), então **não há problema de CORS** localmente. A base da API é configurável
pela variável `VITE_API_BASE_URL` no arquivo `.env` (padrão `/api/v1`).

### Credenciais de teste (semeadas em `database/init.sql`)

- Usuário: **Peri de Lima** · Senha: **123456** (perfil Operador)

## Scripts

| Comando         | Descrição                                  |
|-----------------|--------------------------------------------|
| `npm run dev`   | Servidor de desenvolvimento                |
| `npm run build` | Type-check (`tsc -b`) + build de produção  |
| `npm run preview` | Pré-visualiza o build de produção        |

## Estrutura

```
src/
  pages/            LoginPage, DashboardPage, EstoquePage
  features/         auth, itens (api + NovoItemDialog + ItemDrawer), movimentacao
  components/ui/    componentes shadcn/ui (button, table, switch, dialog, sheet, select, ...)
  components/layout/ Sidebar, Header, AppLayout
  context/          AuthContext (sessão em localStorage/sessionStorage)
  lib/              api (axios), utils (cn), datas (NaiveDateTime)
  types/            Item, Usuario, Movimentacao + helper isCritico()
```

## Telas

- **Login** — autentica via `POST /api/v1/auth/login`.
- **Dashboard** — saudação + data, métricas (Materiais Críticos, Total de Itens,
  Movimentações Hoje) e tabela de alertas críticos com botão **Ver** (abre o item no Estoque).
- **Estoque** — tabela com busca, filtro de categoria, toggle "Mostrar apenas Críticos",
  paginação e badges OK/CRÍTICO. Clicar numa linha abre o **drawer** com Entrada/Saída,
  observação e histórico de movimentações do item. Botão **Novo Item** cadastra insumos.

## Observações técnicas

- O badge **CRÍTICO** segue o RF-002: `quantidade_atual <= quantidade_minima`.
- Datas de movimentação são enviadas como `NaiveDateTime` **sem** sufixo de timezone
  (ex.: `2026-06-10T14:30:00`) — ver `src/lib/datas.ts`.
- A sessão é apenas client-side (sem JWT no MVP); o `responsavel_id` das movimentações vem do
  usuário logado.
