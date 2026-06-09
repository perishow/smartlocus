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

docker compose exec mariadb mariadb-dump -u root -psenha smartlocus > dump_2026-06-09_17-03-58.sql
