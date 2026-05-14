use tauri::State;

use crate::state::AppState;
use crate::models::funcionario::{CreateFuncionario, CreateFuncionarioResponse, Funcionario, UpdateFuncionario};
use crate::services::funcionario_service;
use crate::auth::guard;

#[tauri::command]
pub fn criar_funcionario(
    state: State<'_, AppState>,
    data: CreateFuncionario,
) -> Result<CreateFuncionarioResponse, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;
    guard::require_admin(session).map_err(String::from)?;

    funcionario_service::criar(&conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn atualizar_funcionario(
    state: State<'_, AppState>,
    data: UpdateFuncionario,
) -> Result<Funcionario, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;
    guard::require_admin(session).map_err(String::from)?;

    funcionario_service::atualizar(&conn, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn listar_funcionarios(
    state: State<'_, AppState>,
) -> Result<Vec<Funcionario>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let _session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    funcionario_service::listar(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn listar_funcionarios_ativos(
    state: State<'_, AppState>,
) -> Result<Vec<Funcionario>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let _session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    funcionario_service::listar_ativos(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn buscar_funcionarios(
    state: State<'_, AppState>,
    termo: String,
) -> Result<Vec<Funcionario>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let _session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    funcionario_service::buscar(&conn, termo).map_err(|e| e.to_string())
}
