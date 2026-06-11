-- ============================================================================
--  SmartLocus — Povoamento inicial do banco (MariaDB)
--  Executado automaticamente na PRIMEIRA inicialização do container.
--  Para reaplicar após mudanças: docker compose down -v && docker compose up -d
-- ============================================================================

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------- Usuarios ------------------------------------
DROP TABLE IF EXISTS `Usuarios`;
CREATE TABLE `Usuarios` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `senha` varchar(255) NOT NULL,
  `perfil` enum('Consultor','Operador') NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

INSERT INTO `Usuarios` (id, nome, email, senha, perfil) VALUES
(1, 'Peri de Lima',           'peri@smartlocus.com',     '123456',                 'Operador'),
(2, 'Leozinho Ruindade Pura', 'leozinho244@gmail.com',   'senha_super_segura123',  'Operador'),
(3, 'Carla Mendes',           'carla@smartlocus.com',    'consultor123',           'Consultor');

-- ----------------------------- Itens ---------------------------------------
DROP TABLE IF EXISTS `Itens`;
CREATE TABLE `Itens` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nome` varchar(255) NOT NULL,
  `categoria` varchar(100) NOT NULL,
  `quantidade_atual` int(11) DEFAULT 0,
  `quantidade_minima` int(11) DEFAULT 0,
  `localizacao` varchar(150) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

-- Itens com quantidade_atual <= quantidade_minima ficam CRÍTICOS (RF-002):
-- 2 (Caneta), 4 (Detergente), 6 (Papel Higiênico), 7 (Cartucho), 10 (Café).
INSERT INTO `Itens` (id, nome, categoria, quantidade_atual, quantidade_minima, localizacao) VALUES
(1,  'Papel A4 (Resma)',            'Material de Escritório', 50, 20, 'Prateleira A1'),
(2,  'Caneta Esferográfica Azul',   'Material de Escritório',  8, 15, 'Prateleira A2'),
(3,  'Marcador para Quadro Branco', 'Material de Escritório', 12,  5, 'Prateleira A1'),
(4,  'Detergente Neutro 5L',        'Material de Limpeza',     3, 10, 'Armário de Limpeza'),
(5,  'Vassoura',                    'Material de Limpeza',     6,  2, 'Armário de Limpeza'),
(6,  'Papel Higiênico (Fardo)',     'Material de Limpeza',     4,  4, 'Almoxarifado B'),
(7,  'Cartucho de Tinta HP 664',    'Informática',             2,  5, 'Prateleira C3'),
(8,  'Mouse USB',                   'Informática',            12,  4, 'Prateleira C1'),
(9,  'Cabo HDMI 2m',                'Informática',             7,  3, 'Prateleira C2'),
(10, 'Café em Pó (500g)',           'Copa',                    5,  8, 'Cozinha');

-- ----------------------- Movimentacoes_Estoque -----------------------------
DROP TABLE IF EXISTS `Movimentacoes_Estoque`;
CREATE TABLE `Movimentacoes_Estoque` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `item_id` int(11) NOT NULL,
  `tipo` enum('Entrada','Saída') NOT NULL,
  `quantidade` int(11) NOT NULL,
  `data_movimentacao` datetime NOT NULL,
  `observacao` text DEFAULT NULL,
  `responsavel_id` int(11) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `item_id` (`item_id`),
  KEY `responsavel_id` (`responsavel_id`),
  CONSTRAINT `fk_mov_item` FOREIGN KEY (`item_id`) REFERENCES `Itens` (`id`),
  CONSTRAINT `fk_mov_responsavel` FOREIGN KEY (`responsavel_id`) REFERENCES `Usuarios` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

INSERT INTO `Movimentacoes_Estoque` (item_id, tipo, quantidade, data_movimentacao, observacao, responsavel_id) VALUES
(1,  'Entrada', 60, '2026-05-16 09:00:00', 'Entrada inicial de papel A4',          1),
(1,  'Saída',   10, '2026-06-01 10:30:00', 'Distribuição para a secretaria',       1),
(2,  'Entrada', 30, '2026-05-16 09:10:00', 'Compra de canetas',                    2),
(2,  'Saída',   22, '2026-06-02 14:00:00', 'Distribuição para os setores',         1),
(4,  'Entrada', 12, '2026-05-20 08:00:00', 'Reposição de material de limpeza',     2),
(4,  'Saída',    9, '2026-06-05 11:15:00', 'Consumo semanal',                      2),
(7,  'Entrada',  6, '2026-05-22 08:30:00', 'Compra de cartuchos de tinta',         1),
(7,  'Saída',    4, '2026-06-08 16:00:00', 'Troca em impressoras',                 1),
(10, 'Entrada', 10, '2026-05-25 07:45:00', 'Abastecimento de café da copa',        2),
(10, 'Saída',    5, '2026-06-07 09:00:00', 'Consumo da copa',                      2);

-- --------------------- Solicitacoes_Materiais ------------------------------
-- (Reservas estão fora do escopo do MVP; tabela criada mas mantida vazia.)
DROP TABLE IF EXISTS `Solicitacoes_Materiais`;
CREATE TABLE `Solicitacoes_Materiais` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `item_id` int(11) NOT NULL,
  `solicitante_id` int(11) NOT NULL,
  `quantidade_solicitada` int(11) NOT NULL,
  `status` enum('Pendente','Aprovada','Rejeitada') DEFAULT 'Pendente',
  `data_solicitacao` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `item_id` (`item_id`),
  KEY `solicitante_id` (`solicitante_id`),
  CONSTRAINT `fk_sol_item` FOREIGN KEY (`item_id`) REFERENCES `Itens` (`id`),
  CONSTRAINT `fk_sol_solicitante` FOREIGN KEY (`solicitante_id`) REFERENCES `Usuarios` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

SET FOREIGN_KEY_CHECKS = 1;
