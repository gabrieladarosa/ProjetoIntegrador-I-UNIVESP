use rusqlite::{params, Connection};

use crate::error::AppError;
use crate::models::servico::{CreateServico, Servico, UpdateServico};

/// Repository responsável pelo acesso a dados de Serviço
/// Nota: nunca deleta serviços fisicamente (INV03)

pub fn insert(conn: &Connection, data: &CreateServico, created_by_user_id: i64) -> Result<Servico, AppError> {
    conn.execute(
        "INSERT INTO servicos (embarcacao_id, funcionario_id, descricao, data_execucao, status, observacao, created_by_user_id)
         VALUES (?1, ?2, ?3, ?4, 'em_execucao', ?5, ?6)",
        params![
            data.embarcacao_id,
            data.funcionario_id,
            data.descricao,
            data.data_execucao,
            data.observacao,
            created_by_user_id,
        ],
    )?;

    let id = conn.last_insert_rowid();
    find_by_id(conn, id)
}

pub fn update(conn: &Connection, data: &UpdateServico, updated_by_user_id: i64) -> Result<Servico, AppError> {
    // Atualização genérica de campos mutáveis
    let mut stmt = conn.prepare(
        "UPDATE servicos SET 
            descricao = COALESCE(?1, descricao),
            data_execucao = COALESCE(?2, data_execucao),
            status = COALESCE(?3, status),
            observacao = COALESCE(?4, observacao),
            funcionario_id = COALESCE(?5, funcionario_id),
            updated_by_user_id = ?6,
            updated_at = datetime('now', 'localtime')
         WHERE id = ?7"
    )?;

    let rows = stmt.execute(params![
        data.descricao,
        data.data_execucao,
        data.status,
        data.observacao,
        data.funcionario_id,
        updated_by_user_id,
        data.id
    ])?;

    if rows == 0 {
        return Err(AppError::NotFound("Serviço não encontrado".into()));
    }

    find_by_id(conn, data.id)
}

pub fn list_all(conn: &Connection) -> Result<Vec<Servico>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.embarcacao_id, s.funcionario_id, s.descricao, s.data_execucao,
                s.status, s.observacao, s.created_at, s.updated_at,
                s.created_by_user_id, s.updated_by_user_id,
                e.nome as embarcacao_nome, f.nome as funcionario_nome
         FROM servicos s
         INNER JOIN embarcacoes e ON s.embarcacao_id = e.id
         INNER JOIN funcionarios f ON s.funcionario_id = f.id
         ORDER BY s.data_execucao DESC"
    )?;

    map_rows(&mut stmt, params![])
}

pub fn list_by_funcionario(conn: &Connection, funcionario_id: i64) -> Result<Vec<Servico>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.embarcacao_id, s.funcionario_id, s.descricao, s.data_execucao,
                s.status, s.observacao, s.created_at, s.updated_at,
                s.created_by_user_id, s.updated_by_user_id,
                e.nome as embarcacao_nome, f.nome as funcionario_nome
         FROM servicos s
         INNER JOIN embarcacoes e ON s.embarcacao_id = e.id
         INNER JOIN funcionarios f ON s.funcionario_id = f.id
         WHERE s.funcionario_id = ?1
         ORDER BY s.data_execucao DESC"
    )?;

    map_rows(&mut stmt, params![funcionario_id])
}

pub fn list_by_embarcacao(conn: &Connection, embarcacao_id: i64) -> Result<Vec<Servico>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.embarcacao_id, s.funcionario_id, s.descricao, s.data_execucao,
                s.status, s.observacao, s.created_at, s.updated_at,
                s.created_by_user_id, s.updated_by_user_id,
                e.nome as embarcacao_nome, f.nome as funcionario_nome
         FROM servicos s
         INNER JOIN embarcacoes e ON s.embarcacao_id = e.id
         INNER JOIN funcionarios f ON s.funcionario_id = f.id
         WHERE s.embarcacao_id = ?1
         ORDER BY s.data_execucao DESC"
    )?;

    map_rows(&mut stmt, params![embarcacao_id])
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<Servico, AppError> {
    conn.query_row(
        "SELECT s.id, s.embarcacao_id, s.funcionario_id, s.descricao, s.data_execucao,
                s.status, s.observacao, s.created_at, s.updated_at,
                s.created_by_user_id, s.updated_by_user_id,
                e.nome as embarcacao_nome, f.nome as funcionario_nome
         FROM servicos s
         INNER JOIN embarcacoes e ON s.embarcacao_id = e.id
         INNER JOIN funcionarios f ON s.funcionario_id = f.id
         WHERE s.id = ?1",
        params![id],
        |row| {
            Ok(Servico {
                id: row.get(0)?,
                embarcacao_id: row.get(1)?,
                funcionario_id: row.get(2)?,
                descricao: row.get(3)?,
                data_execucao: row.get(4)?,
                status: row.get(5)?,
                observacao: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                created_by_user_id: row.get(9)?,
                updated_by_user_id: row.get(10)?,
                embarcacao_nome: row.get(11)?,
                funcionario_nome: row.get(12)?,
            })
        },
    )
    .map_err(|_| AppError::NotFound("Serviço não encontrado".into()))
}

fn map_rows(stmt: &mut rusqlite::Statement, params: impl rusqlite::Params) -> Result<Vec<Servico>, AppError> {
    let rows = stmt.query_map(params, |row| {
        Ok(Servico {
            id: row.get(0)?,
            embarcacao_id: row.get(1)?,
            funcionario_id: row.get(2)?,
            descricao: row.get(3)?,
            data_execucao: row.get(4)?,
            status: row.get(5)?,
            observacao: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            created_by_user_id: row.get(9)?,
            updated_by_user_id: row.get(10)?,
            embarcacao_nome: row.get(11)?,
            funcionario_nome: row.get(12)?,
        })
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}
