use tauri::State;

use crate::state::AppState;
use crate::models::servico::{CreateServico, Servico, UpdateServico};
use crate::services::servico_service;
use crate::auth::guard;

#[tauri::command]
pub fn criar_servico(
    state: State<'_, AppState>,
    data: CreateServico,
) -> Result<Servico, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    servico_service::criar(&conn, session, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn atualizar_servico(
    state: State<'_, AppState>,
    data: UpdateServico,
) -> Result<Servico, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    servico_service::atualizar(&conn, session, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn listar_servicos(
    state: State<'_, AppState>,
) -> Result<Vec<Servico>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    servico_service::listar(&conn, session).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn listar_servicos_por_embarcacao(
    state: State<'_, AppState>,
    embarcacao_id: i64,
) -> Result<Vec<Servico>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    servico_service::listar_por_embarcacao(&conn, session, embarcacao_id).map_err(|e| e.to_string())
}
