use rusqlite::Connection;

use crate::error::AppError;

/// Executa as migrações do banco de dados
pub fn run(conn: &Connection) -> Result<(), AppError> {
    // 1. Criação das tabelas base (se não existirem)
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS embarcacoes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            identificacao TEXT UNIQUE NOT NULL,
            modelo TEXT,
            tipo TEXT,
            comprimento REAL,
            ano_fabricacao INTEGER,
            cliente_responsavel TEXT,
            status TEXT NOT NULL DEFAULT 'ativa',
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        );

        CREATE TABLE IF NOT EXISTS funcionarios (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            cargo TEXT,
            telefone TEXT,
            ativo INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        );

        CREATE TABLE IF NOT EXISTS servicos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            embarcacao_id INTEGER NOT NULL,
            funcionario_id INTEGER NOT NULL,
            descricao TEXT NOT NULL,
            data_execucao TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pendente',
            observacao TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (embarcacao_id) REFERENCES embarcacoes(id),
            FOREIGN KEY (funcionario_id) REFERENCES funcionarios(id)
        );

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            login TEXT UNIQUE NOT NULL,
            senha_hash TEXT NOT NULL,
            role TEXT NOT NULL CHECK(role IN ('admin', 'funcionario')),
            funcionario_id INTEGER,
            primeiro_acesso INTEGER NOT NULL DEFAULT 1,
            ativo INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (funcionario_id) REFERENCES funcionarios(id)
        );

        -- Seed do admin inicial (senha: admin123)
        INSERT OR IGNORE INTO users (login, senha_hash, role, primeiro_acesso)
        VALUES ('admin', '$2b$10$TJzLBiGKL6yUTnaz0MUp0OrZdJHgp6UAkySQxtSFXCnZRXIbUcbEK', 'admin', 1);

        -- Garantir que o hash quebrado gerado anteriormente seja corrigido
        UPDATE users 
        SET senha_hash = '$2b$10$TJzLBiGKL6yUTnaz0MUp0OrZdJHgp6UAkySQxtSFXCnZRXIbUcbEK'
        WHERE login = 'admin' AND senha_hash LIKE '$2b$10$C78BfFvQ5nI4mZ0Pz%';
        "
    )?;

    // 2. Migrations Incrementais (tratando erros se a coluna já existir)
    // Nota: Em SQLite, não há IF NOT EXISTS para colunas no ALTER TABLE.
    // Usamos um bloco catch-all simples para esta refatoração.
    
    let _ = conn.execute("ALTER TABLE embarcacoes ADD COLUMN funcionario_id INTEGER NULL REFERENCES funcionarios(id)", []);
    let _ = conn.execute("ALTER TABLE servicos ADD COLUMN created_by_user_id INTEGER NULL REFERENCES users(id)", []);
    let _ = conn.execute("ALTER TABLE servicos ADD COLUMN updated_by_user_id INTEGER NULL REFERENCES users(id)", []);
    let _ = conn.execute("ALTER TABLE servicos ADD COLUMN created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))", []);
    let _ = conn.execute("ALTER TABLE servicos ADD COLUMN updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))", []);
    let _ = conn.execute("ALTER TABLE embarcacoes ADD COLUMN created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))", []);
    let _ = conn.execute("ALTER TABLE embarcacoes ADD COLUMN updated_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))", []);
    let _ = conn.execute("ALTER TABLE users ADD COLUMN created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))", []);
    
    conn.execute_batch(
        "
        -- Migração de Status: Pendente -> Em Execução
        UPDATE servicos SET status = 'em_execucao' WHERE status = 'pendente';

        -- Índices para performance
        CREATE INDEX IF NOT EXISTS idx_servicos_funcionario ON servicos(funcionario_id);
        CREATE INDEX IF NOT EXISTS idx_servicos_embarcacao ON servicos(embarcacao_id);
        CREATE INDEX IF NOT EXISTS idx_embarcacoes_funcionario ON embarcacoes(funcionario_id);
        CREATE INDEX IF NOT EXISTS idx_users_funcionario ON users(funcionario_id);
        "
    )?;

    Ok(())
}
