use rusqlite::Connection;

use crate::error::AppError;
use crate::auth::hasher;
use crate::models::funcionario::{CreateFuncionario, CreateFuncionarioResponse, Funcionario, UpdateFuncionario};
use crate::repositories::{funcionario_repository, user_repository};

/// Service Layer — regras de negócio de Funcionário

pub fn criar(conn: &Connection, data: CreateFuncionario) -> Result<CreateFuncionarioResponse, AppError> {
    if data.nome.trim().is_empty() {
        return Err(AppError::Validation("Nome do funcionário é obrigatório".into()));
    }

    let funcionario = funcionario_repository::insert(conn, &data)?;

    // Gerar login a partir do nome (minúsculo, sem espaços, sem acentos)
    let login_base = data.nome
        .trim()
        .to_lowercase()
        .split_whitespace()
        .next()
        .unwrap_or("func")
        .to_string();

    // Garantir unicidade do login
    let mut login = login_base.clone();
    let mut counter = 1;
    while user_repository::login_exists(conn, &login)? {
        login = format!("{}{}", login_base, counter);
        counter += 1;
    }

    // Gerar senha temporária e criar usuário
    let senha_temporaria = hasher::generate_temp_password();
    let hash = hasher::hash_password(&senha_temporaria)?;

    user_repository::insert(conn, &login, &hash, "funcionario", Some(funcionario.id))?;

    Ok(CreateFuncionarioResponse {
        funcionario,
        login,
        senha_temporaria,
    })
}

pub fn atualizar(conn: &Connection, data: UpdateFuncionario) -> Result<Funcionario, AppError> {
    if data.nome.trim().is_empty() {
        return Err(AppError::Validation("Nome do funcionário é obrigatório".into()));
    }

    funcionario_repository::update(conn, &data)
}

pub fn listar(conn: &Connection) -> Result<Vec<Funcionario>, AppError> {
    funcionario_repository::list(conn)
}

pub fn listar_ativos(conn: &Connection) -> Result<Vec<Funcionario>, AppError> {
    funcionario_repository::list_ativos(conn)
}

pub fn buscar(conn: &Connection, termo: String) -> Result<Vec<Funcionario>, AppError> {
    if termo.trim().is_empty() {
        return funcionario_repository::list(conn);
    }
    funcionario_repository::search(conn, &termo)
}
