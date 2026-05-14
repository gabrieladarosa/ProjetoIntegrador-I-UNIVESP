use rusqlite::{params, Connection};

use crate::error::AppError;
use crate::models::user::User;

/// Repository responsável pelo acesso a dados de User
/// Princípio: Single Responsibility — apenas SQL, sem lógica de negócio

pub fn insert(
    conn: &Connection,
    login: &str,
    senha_hash: &str,
    role: &str,
    funcionario_id: Option<i64>,
) -> Result<User, AppError> {
    conn.execute(
        "INSERT INTO users (login, senha_hash, role, funcionario_id)
         VALUES (?1, ?2, ?3, ?4)",
        params![login, senha_hash, role, funcionario_id],
    )?;

    let id = conn.last_insert_rowid();
    find_by_id(conn, id)
}

pub fn find_by_login(conn: &Connection, login: &str) -> Result<User, AppError> {
    conn.query_row(
        "SELECT u.id, u.login, u.senha_hash, u.role, u.funcionario_id,
                u.primeiro_acesso, u.ativo, u.created_at,
                f.nome as funcionario_nome
         FROM users u
         LEFT JOIN funcionarios f ON u.funcionario_id = f.id
         WHERE u.login = ?1",
        params![login],
        |row| {
            Ok(User {
                id: row.get(0)?,
                login: row.get(1)?,
                senha_hash: row.get(2)?,
                role: row.get(3)?,
                funcionario_id: row.get(4)?,
                primeiro_acesso: row.get(5)?,
                ativo: row.get(6)?,
                created_at: row.get(7)?,
                funcionario_nome: row.get(8)?,
            })
        },
    )
    .map_err(|_| AppError::NotFound("Usuário não encontrado".into()))
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<User, AppError> {
    conn.query_row(
        "SELECT u.id, u.login, u.senha_hash, u.role, u.funcionario_id,
                u.primeiro_acesso, u.ativo, u.created_at,
                f.nome as funcionario_nome
         FROM users u
         LEFT JOIN funcionarios f ON u.funcionario_id = f.id
         WHERE u.id = ?1",
        params![id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                login: row.get(1)?,
                senha_hash: row.get(2)?,
                role: row.get(3)?,
                funcionario_id: row.get(4)?,
                primeiro_acesso: row.get(5)?,
                ativo: row.get(6)?,
                created_at: row.get(7)?,
                funcionario_nome: row.get(8)?,
            })
        },
    )
    .map_err(|_| AppError::NotFound("Usuário não encontrado".into()))
}

pub fn update_senha(conn: &Connection, id: i64, senha_hash: &str) -> Result<(), AppError> {
    let rows = conn.execute(
        "UPDATE users SET senha_hash = ?1 WHERE id = ?2",
        params![senha_hash, id],
    )?;

    if rows == 0 {
        return Err(AppError::NotFound("Usuário não encontrado".into()));
    }

    Ok(())
}

pub fn update_primeiro_acesso(conn: &Connection, id: i64, valor: bool) -> Result<(), AppError> {
    let rows = conn.execute(
        "UPDATE users SET primeiro_acesso = ?1 WHERE id = ?2",
        params![valor, id],
    )?;

    if rows == 0 {
        return Err(AppError::NotFound("Usuário não encontrado".into()));
    }

    Ok(())
}

pub fn list(conn: &Connection) -> Result<Vec<User>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT u.id, u.login, u.senha_hash, u.role, u.funcionario_id,
                u.primeiro_acesso, u.ativo, u.created_at,
                f.nome as funcionario_nome
         FROM users u
         LEFT JOIN funcionarios f ON u.funcionario_id = f.id
         ORDER BY u.login ASC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            login: row.get(1)?,
            senha_hash: row.get(2)?,
            role: row.get(3)?,
            funcionario_id: row.get(4)?,
            primeiro_acesso: row.get(5)?,
            ativo: row.get(6)?,
            created_at: row.get(7)?,
            funcionario_nome: row.get(8)?,
        })
    })?;

    let mut users = Vec::new();
    for row in rows {
        users.push(row?);
    }

    Ok(users)
}

/// Atualiza o status ativo/inativo do usuário
pub fn update_ativo(conn: &Connection, id: i64, ativo: bool) -> Result<(), AppError> {
    let rows = conn.execute(
        "UPDATE users SET ativo = ?1 WHERE id = ?2",
        params![ativo, id],
    )?;

    if rows == 0 {
        return Err(AppError::NotFound("Usuário não encontrado".into()));
    }

    Ok(())
}

/// Verifica se já existe um usuário com o login informado
pub fn login_exists(conn: &Connection, login: &str) -> Result<bool, AppError> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM users WHERE login = ?1",
        params![login],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn update(
    conn: &Connection,
    id: i64,
    login: &str,
    role: &str,
    funcionario_id: Option<i64>,
) -> Result<(), AppError> {
    let rows = conn.execute(
        "UPDATE users SET login = ?1, role = ?2, funcionario_id = ?3 WHERE id = ?4",
        params![login, role, funcionario_id, id],
    )?;

    if rows == 0 {
        return Err(AppError::NotFound("Usuário não encontrado".into()));
    }

    Ok(())
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let rows = conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;

    if rows == 0 {
        return Err(AppError::NotFound("Usuário não encontrado".into()));
    }

    Ok(())
}
