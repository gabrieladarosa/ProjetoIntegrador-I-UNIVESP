use rusqlite::Connection;

use crate::error::AppError;
use crate::models::embarcacao::{CreateEmbarcacao, Embarcacao, UpdateEmbarcacao};
use crate::models::user::{Session, Role};
use crate::repositories::embarcacao_repository;

/// Service Layer — regras de negócio de Embarcação
/// Princípio: Open/Closed — novas validações são adicionadas aqui sem alterar o repository

pub fn criar(conn: &Connection, data: CreateEmbarcacao) -> Result<Embarcacao, AppError> {
    // Validações de negócio
    if data.nome.trim().is_empty() {
        return Err(AppError::Validation("Nome da embarcação é obrigatório".into()));
    }
    if data.identificacao.trim().is_empty() {
        return Err(AppError::Validation("Identificação da embarcação é obrigatória".into()));
    }

    embarcacao_repository::insert(conn, &data)
}

pub fn atualizar(conn: &Connection, data: UpdateEmbarcacao) -> Result<Embarcacao, AppError> {
    if data.nome.trim().is_empty() {
        return Err(AppError::Validation("Nome da embarcação é obrigatório".into()));
    }
    if data.identificacao.trim().is_empty() {
        return Err(AppError::Validation("Identificação da embarcação é obrigatória".into()));
    }

    let status_validos = ["ativa", "inativa", "em_manutencao"];
    if !status_validos.contains(&data.status.as_str()) {
        return Err(AppError::Validation(
            format!("Status inválido. Use: {}", status_validos.join(", "))
        ));
    }

    embarcacao_repository::update(conn, &data)
}

pub fn listar(conn: &Connection, session: &Session) -> Result<Vec<Embarcacao>, AppError> {
    if session.role == Role::Admin {
        embarcacao_repository::list(conn)
    } else {
        let fid = session.funcionario_id.ok_or_else(|| AppError::Forbidden("Usuário sem vínculo".into()))?;
        embarcacao_repository::list_by_funcionario(conn, fid)
    }
}

pub fn buscar(conn: &Connection, session: &Session, termo: String) -> Result<Vec<Embarcacao>, AppError> {
    if termo.trim().is_empty() {
        return listar(conn, session);
    }
    
    let resultados = embarcacao_repository::search(conn, &termo)?;
    
    // Filtro pós-busca para funcionários (simples, mas eficaz para este escopo)
    if session.role == Role::Funcionario {
        let fid = session.funcionario_id;
        return Ok(resultados.into_iter().filter(|e| e.funcionario_id == fid).collect());
    }
    
    Ok(resultados)
}
