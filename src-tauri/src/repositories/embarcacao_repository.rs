use rusqlite::{params, Connection};

use crate::error::AppError;
use crate::models::embarcacao::{CreateEmbarcacao, Embarcacao, UpdateEmbarcacao};

/// Repository responsável pelo acesso a dados de Embarcação
/// Princípio: Single Responsibility — apenas SQL, sem lógica de negócio

pub fn insert(conn: &Connection, data: &CreateEmbarcacao) -> Result<Embarcacao, AppError> {
    conn.execute(
        "INSERT INTO embarcacoes (nome, identificacao, modelo, tipo, comprimento, ano_fabricacao, cliente_responsavel, funcionario_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            data.nome,
            data.identificacao,
            data.modelo,
            data.tipo,
            data.comprimento,
            data.ano_fabricacao,
            data.cliente_responsavel,
            data.funcionario_id,
        ],
    )?;

    let id = conn.last_insert_rowid();
    find_by_id(conn, id)
}

pub fn update(conn: &Connection, data: &UpdateEmbarcacao) -> Result<Embarcacao, AppError> {
    let rows = conn.execute(
        "UPDATE embarcacoes SET nome = ?1, identificacao = ?2, modelo = ?3, tipo = ?4,
         comprimento = ?5, ano_fabricacao = ?6, cliente_responsavel = ?7, status = ?8,
         funcionario_id = ?9, updated_at = datetime('now', 'localtime')
         WHERE id = ?10",
        params![
            data.nome,
            data.identificacao,
            data.modelo,
            data.tipo,
            data.comprimento,
            data.ano_fabricacao,
            data.cliente_responsavel,
            data.status,
            data.funcionario_id,
            data.id,
        ],
    )?;

    if rows == 0 {
        return Err(AppError::NotFound("Embarcação não encontrada".into()));
    }

    find_by_id(conn, data.id)
}

pub fn list(conn: &Connection) -> Result<Vec<Embarcacao>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT e.id, e.nome, e.identificacao, e.modelo, e.tipo, e.comprimento, e.ano_fabricacao,
                e.cliente_responsavel, e.status, e.created_at, e.updated_at,
                e.funcionario_id, f.nome as funcionario_nome
         FROM embarcacoes e
         LEFT JOIN funcionarios f ON e.funcionario_id = f.id
         ORDER BY e.nome ASC"
    )?;

    map_rows(&mut stmt, params![])
}

pub fn list_by_funcionario(conn: &Connection, funcionario_id: i64) -> Result<Vec<Embarcacao>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT e.id, e.nome, e.identificacao, e.modelo, e.tipo, e.comprimento, e.ano_fabricacao,
                e.cliente_responsavel, e.status, e.created_at, e.updated_at,
                e.funcionario_id, f.nome as funcionario_nome
         FROM embarcacoes e
         LEFT JOIN funcionarios f ON e.funcionario_id = f.id
         WHERE e.funcionario_id = ?1
         ORDER BY e.nome ASC"
    )?;

    map_rows(&mut stmt, params![funcionario_id])
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<Embarcacao, AppError> {
    conn.query_row(
        "SELECT e.id, e.nome, e.identificacao, e.modelo, e.tipo, e.comprimento, e.ano_fabricacao,
                e.cliente_responsavel, e.status, e.created_at, e.updated_at,
                e.funcionario_id, f.nome as funcionario_nome
         FROM embarcacoes e
         LEFT JOIN funcionarios f ON e.funcionario_id = f.id
         WHERE e.id = ?1",
        params![id],
        |row| {
            Ok(Embarcacao {
                id: row.get(0)?,
                nome: row.get(1)?,
                identificacao: row.get(2)?,
                modelo: row.get(3)?,
                tipo: row.get(4)?,
                comprimento: row.get(5)?,
                ano_fabricacao: row.get(6)?,
                cliente_responsavel: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                funcionario_id: row.get(11)?,
                funcionario_nome: row.get(12)?,
            })
        },
    )
    .map_err(|_| AppError::NotFound("Embarcação não encontrada".into()))
}

pub fn search(conn: &Connection, termo: &str) -> Result<Vec<Embarcacao>, AppError> {
    let termo_like = format!("%{}%", termo);
    let mut stmt = conn.prepare(
        "SELECT e.id, e.nome, e.identificacao, e.modelo, e.tipo, e.comprimento, e.ano_fabricacao,
                e.cliente_responsavel, e.status, e.created_at, e.updated_at,
                e.funcionario_id, f.nome as funcionario_nome
         FROM embarcacoes e
         LEFT JOIN funcionarios f ON e.funcionario_id = f.id
         WHERE e.nome LIKE ?1 OR e.identificacao LIKE ?1 OR e.cliente_responsavel LIKE ?1
         ORDER BY e.nome ASC"
    )?;

    map_rows(&mut stmt, params![termo_like])
}

fn map_rows(stmt: &mut rusqlite::Statement, params: impl rusqlite::Params) -> Result<Vec<Embarcacao>, AppError> {
    let rows = stmt.query_map(params, |row| {
        Ok(Embarcacao {
            id: row.get(0)?,
            nome: row.get(1)?,
            identificacao: row.get(2)?,
            modelo: row.get(3)?,
            tipo: row.get(4)?,
            comprimento: row.get(5)?,
            ano_fabricacao: row.get(6)?,
            cliente_responsavel: row.get(7)?,
            status: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
            funcionario_id: row.get(11)?,
            funcionario_nome: row.get(12)?,
        })
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}
