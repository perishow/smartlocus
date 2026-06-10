# Smartlocus

Smartlocus é uma solução de gerenciamento de almoxarifado, implementada como projeto final da cadeira de "Projeto de Engenharia da Computação".

## Requisitos

- Docker Engine
- Rust + Cargo: pode ser instalado como instruído no site oficial: <https://rust-lang.org/tools/install/>

## O sistema funciona em 3 partes

- Um banco de dados MariaDB
- Um backend em Rust
- Um frontend em alguma coisa que ainda vai ser implementado

### Banco de dados

O banco de dados MariaDB é instanciado automaticamente ao iniciar o backend e já vem com alguns dados instanciados porém, qualquer dado inserido posteriormente ao banco não é persistente e será deletado caso o banco seja reiniciado.
Para garantir a persistência dos dados novos, o arquivo "database/init.sql" deve ser substituído por um novo dump do banco. Esse dump pode ser gerado utilizando o comando:

```bash
docker compose exec mariadb mariadb-dump -u root -psenha smartlocus > init.sql
```

### Backend 

O backend foi codado em rust e utiliza o cargo como iniciador padrão. Com o cargo previamente instalado, basta entrar no diretório "/back-end" e utilizar o comando:


```bash 
cargo run 
```

O backend iniciará o banco de dados e estabelecerá a conexão após algumas tentativas, após isso ele está pronto para receber requisições.

### ESPAÇO PARA EXPLICAR O FRONT SE QUISER DPS

## Rotas do backend

Abaixo estão todas as rotas do backend, qual método utilizar e qual o formato do corpo de cada uma:

### Autenticação:

#### `POST` /api/v1/auth/register
Registra um novo usuário no sistema.

Corpo da requisição:

```json
{
  "nome": "Leozinho Ruindade Pura",
  "email": "leozinho244@gmail.com",
  "senha": "senha_super_segura123",
  "perfil": "Operador"
}
```

#### `POST` /api/v1/auth/login
Checa se o usuário já possui cadastro.

Corpo da requisição:

```json
{
  "nome": "Peri de Lima",
  "senha": "123456"
}
```

#### `POST` /api/v1/auth/delete
Deleta o registro do usuário.

Corpo da requisição:

```json
{
  "id": 1
}
```

### Item

#### `GET` /api/v1/item/get-all
Coleta todos os itens registrados e suas respectivas informações.

#### `GET` /api/v1/item/get-all-quantidade-critica
Coleta todos os itens registrados que estão com a quantidade menor do que a quantidade mínima registrada.

#### `POST` /api/v1/item/register-item
Registra um novo item no sistema.

Corpo da requisição:

```json
{
  "nome": "nome do produto",
  "categoria": "categoria do produto",
  "quantidade_atual": 0,
  "quantidade_minima": 3,
  "localizacao": "localizacao do produto"
}
```

#### `POST` /api/v1/item/delete-item
deleta o item do sistema.

Corpo da requisição:

```json
{
  "id": 1
}
```

#### `POST` /api/v1/item/adicionar-quantidade
Adiciona uma quantidade especificada ao item previamente registrado.

Corpo da requisição: 

```json
{
  "id_item": 1,
  "quantidade": 200,
  "data_movimentacao": "2026-06-09T14:30:00",
  "observacao": "observação opcional",
  "responsavel_id": 67
}
```

#### `POST` /api/v1/item/subtrair-quantidade
Subtrai uma quantidade especificada do item previamente registrado.

Corpo da requisição:

```json
{
  "id_item": 1,
  "quantidade": 200,
  "data_movimentacao": "2026-06-09T14:30:00",
  "observacao": "observação opcional",
  "responsavel_id": 67
}
```

### Movimentacao do estoque:

#### `GET` /api/v1/movimentacao/get-all
Coleta todas as movimentacoes do sistema.

