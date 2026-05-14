use tauri::State;

use crate::state::AppState;
use crate::models::embarcacao::{CreateEmbarcacao, Embarcacao, UpdateEmbarcacao};
use crate::services::embarcacao_service;
use crate::auth::guard;

/// Commands Tauri — interface entre frontend e backend
/// Princípio: Interface Segregation — cada command é específico para sua operação

#[tauri::command]
pub fn criar_embarcacao(
    state: State<'_, AppState>,
    data: CreateEmbarcacao,
) -> Result<Embarcacao, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;
    guard::require_admin(session).map_err(String::from)?;

    embarcacao_service::criar(&conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn atualizar_embarcacao(
    state: State<'_, AppState>,
    data: UpdateEmbarcacao,
) -> Result<Embarcacao, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;
    guard::require_admin(session).map_err(String::from)?;

    embarcacao_service::atualizar(&conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn listar_embarcacoes(
    state: State<'_, AppState>,
) -> Result<Vec<Embarcacao>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    embarcacao_service::listar(&conn, session).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn buscar_embarcacoes(
    state: State<'_, AppState>,
    termo: String,
) -> Result<Vec<Embarcacao>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    embarcacao_service::buscar(&conn, session, termo).map_err(|e| e.to_string())
}
