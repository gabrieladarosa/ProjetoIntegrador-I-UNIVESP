use tauri::State;

use crate::auth::guard;
use crate::models::user::{
    CreateUser, CreateUserResponse, LoginRequest, LoginResponse,
    ResetSenhaResponse, Session, TrocarSenhaRequest, User,
};
use crate::services::auth_service;
use crate::state::AppState;

/// Commands de Autenticação — thin wrappers sobre auth_service
/// Princípio: Interface Segregation — cada command faz uma operação específica

#[tauri::command]
pub fn cmd_login(
    state: State<'_, AppState>,
    data: LoginRequest,
) -> Result<LoginResponse, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let response = auth_service::login(&conn, data).map_err(|e| e.to_string())?;

    // Armazenar sessão no estado global
    let mut session = state.session.lock().map_err(|e| e.to_string())?;
    *session = Some(response.session.clone());

    Ok(response)
}

#[tauri::command]
pub fn cmd_logout(state: State<'_, AppState>) -> Result<(), String> {
    let mut session = state.session.lock().map_err(|e| e.to_string())?;
    *session = None;
    Ok(())
}

#[tauri::command]
pub fn cmd_trocar_senha(
    state: State<'_, AppState>,
    data: TrocarSenhaRequest,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    auth_service::trocar_senha(&conn, session, data).map_err(|e| e.to_string())?;

    // Atualizar sessão para refletir primeiro_acesso = false
    drop(session_lock);
    let mut session_lock = state.session.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut s) = *session_lock {
        s.primeiro_acesso = false;
    }

    Ok(())
}

#[tauri::command]
pub fn cmd_resetar_senha(
    state: State<'_, AppState>,
    user_id: i64,
) -> Result<ResetSenhaResponse, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    auth_service::resetar_senha(&conn, session, user_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_criar_usuario(
    state: State<'_, AppState>,
    data: CreateUser,
) -> Result<CreateUserResponse, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    auth_service::criar_usuario(&conn, session, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_listar_usuarios(
    state: State<'_, AppState>,
) -> Result<Vec<User>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let session_lock = state.session.lock().map_err(|e| e.to_string())?;
    let session = guard::require_authenticated(&session_lock).map_err(String::from)?;

    auth_service::listar_usuarios(&conn, session).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_sessao_atual(
    state: State<'_, AppState>,
) -> Result<Option<Session>, String> {
    let session = state.session.lock().map_err(|e| e.to_string())?;
    Ok(session.clone())
}
#[tauri::command]
pub fn cmd_gerar_usuarios_para_funcionarios(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    match auth_service::gerar_usuarios_para_funcionarios(&conn) {
        Ok(count) => Ok(format!("{} usuários criados com sucesso", count)),
        Err(e) => Err(e.to_string()),
    }
}
