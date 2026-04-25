use rusqlite::{Connection, params};

use crate::auth::guard;
use crate::auth::hasher;
use crate::error::AppError;
use crate::models::user::{
    CreateUser, CreateUserResponse, LoginRequest, LoginResponse,
    ResetSenhaResponse, Role, Session, TrocarSenhaRequest, User,
};
use crate::repositories::user_repository;

/// Service Layer — regras de negócio de Autenticação
/// Concentra invariantes de auth: hash, verificação, fluxo de primeiro acesso, RBAC

/// Realiza login: busca user, verifica hash, cria Session
pub fn login(conn: &Connection, data: LoginRequest) -> Result<LoginResponse, AppError> {
    // Buscar usuário pelo login
    let user = user_repository::find_by_login(conn, &data.login)
        .map_err(|_| AppError::Unauthorized("Login ou senha inválidos".into()))?;

    // Verificar se o usuário está ativo
    if !user.ativo {
        return Err(AppError::Unauthorized("Usuário desativado. Contate o administrador.".into()));
    }

    // Verificar senha
    let senha_valida = hasher::verify_password(&data.senha, &user.senha_hash)?;
    if !senha_valida {
        return Err(AppError::Unauthorized("Login ou senha inválidos".into()));
    }

    // Criar sessão
    let role = Role::from_str(&user.role)
        .map_err(|e| AppError::Database(e))?;

    let session = Session {
        user_id: user.id,
        login: user.login,
        role,
        primeiro_acesso: user.primeiro_acesso,
    };

    Ok(LoginResponse {
        primeiro_acesso: user.primeiro_acesso,
        session,
    })
}

/// Troca de senha — valida senha atual, hash nova, marca primeiro_acesso=0
pub fn trocar_senha(
    conn: &Connection,
    session: &Session,
    data: TrocarSenhaRequest,
) -> Result<(), AppError> {
    // Validar nova senha
    if data.nova_senha.len() < 4 {
        return Err(AppError::Validation("Nova senha deve ter no mínimo 4 caracteres".into()));
    }

    // Buscar user para verificar senha atual
    let user = user_repository::find_by_id(conn, session.user_id)?;

    // Verificar senha atual
    let senha_valida = hasher::verify_password(&data.senha_atual, &user.senha_hash)?;
    if !senha_valida {
        return Err(AppError::Validation("Senha atual incorreta".into()));
    }

    // Hash da nova senha
    let novo_hash = hasher::hash_password(&data.nova_senha)?;

    // Atualizar senha
    user_repository::update_senha(conn, session.user_id, &novo_hash)?;

    // Marcar primeiro_acesso como false
    user_repository::update_primeiro_acesso(conn, session.user_id, false)?;

    Ok(())
}

/// Reset de senha — somente admin pode executar
/// Gera nova senha temporária e marca primeiro_acesso=1
pub fn resetar_senha(
    conn: &Connection,
    session: &Session,
    user_id: i64,
) -> Result<ResetSenhaResponse, AppError> {
    // Guard: somente admin
    guard::require_admin(session)?;

    // Não permitir auto-reset (admin pode trocar senha dele via trocar_senha)
    if session.user_id == user_id {
        return Err(AppError::Validation(
            "Use 'Trocar Senha' para alterar sua própria senha".into()
        ));
    }

    // Gerar senha temporária
    let temp_password = hasher::generate_temp_password();
    let hash = hasher::hash_password(&temp_password)?;

    // Atualizar no banco
    user_repository::update_senha(conn, user_id, &hash)?;
    user_repository::update_primeiro_acesso(conn, user_id, true)?;

    Ok(ResetSenhaResponse {
        senha_temporaria: temp_password,
    })
}

/// Criar novo usuário — somente admin
pub fn criar_usuario(
    conn: &Connection,
    session: &Session,
    data: CreateUser,
) -> Result<CreateUserResponse, AppError> {
    // Guard: somente admin
    guard::require_admin(session)?;

    // Validações
    if data.login.trim().is_empty() {
        return Err(AppError::Validation("Login é obrigatório".into()));
    }

    // Validar role
    let _ = Role::from_str(&data.role)
        .map_err(|_| AppError::Validation("Role deve ser 'admin' ou 'funcionario'".into()))?;

    // Verificar se login já existe
    if user_repository::login_exists(conn, &data.login)? {
        return Err(AppError::Validation(format!("Login '{}' já está em uso", data.login)));
    }

    // Gerar senha temporária
    let temp_password = hasher::generate_temp_password();
    let hash = hasher::hash_password(&temp_password)?;

    // Criar usuário
    let user = user_repository::insert(
        conn,
        &data.login,
        &hash,
        &data.role,
        data.funcionario_id,
    )?;

    Ok(CreateUserResponse {
        user,
        senha_temporaria: temp_password,
    })
}

/// Listar todos os usuários — somente admin
pub fn listar_usuarios(
    conn: &Connection,
    session: &Session,
) -> Result<Vec<User>, AppError> {
    guard::require_admin(session)?;
    user_repository::list(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrations::run(&conn).unwrap();
        conn
    }

    #[test]
    fn test_login_admin_seed() {
        let conn = setup_db();
        let result = login(&conn, LoginRequest {
            login: "admin".into(),
            senha: "admin123".into(),
        });
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.primeiro_acesso);
        assert_eq!(resp.session.role, Role::Admin);
    }

    #[test]
    fn test_login_senha_errada() {
        let conn = setup_db();
        let result = login(&conn, LoginRequest {
            login: "admin".into(),
            senha: "errada".into(),
        });
        assert!(matches!(result, Err(AppError::Unauthorized(_))));
    }

    #[test]
    fn test_criar_usuario_e_login() {
        let conn = setup_db();
        // Login como admin para criar usuario
        let admin_session = login(&conn, LoginRequest {
            login: "admin".into(),
            senha: "admin123".into(),
        }).unwrap().session;

        // Trocar senha do admin (sair do primeiro_acesso)
        trocar_senha(&conn, &admin_session, TrocarSenhaRequest {
            senha_atual: "admin123".into(),
            nova_senha: "nova123".into(),
        }).unwrap();

        // Criar novo usuário
        let create_resp = criar_usuario(&conn, &admin_session, CreateUser {
            login: "joao".into(),
            role: "funcionario".into(),
            funcionario_id: None,
        }).unwrap();

        // Login com o novo usuário
        let result = login(&conn, LoginRequest {
            login: "joao".into(),
            senha: create_resp.senha_temporaria.clone(),
        });
        assert!(result.is_ok());
        assert!(result.unwrap().primeiro_acesso);
    }

    #[test]
    fn test_resetar_senha() {
        let conn = setup_db();
        let admin_session = login(&conn, LoginRequest {
            login: "admin".into(),
            senha: "admin123".into(),
        }).unwrap().session;

        // Criar funcionário
        let create_resp = criar_usuario(&conn, &admin_session, CreateUser {
            login: "maria".into(),
            role: "funcionario".into(),
            funcionario_id: None,
        }).unwrap();

        // Resetar senha
        let reset_resp = resetar_senha(&conn, &admin_session, create_resp.user.id).unwrap();

        // Login com nova temp
        let result = login(&conn, LoginRequest {
            login: "maria".into(),
            senha: reset_resp.senha_temporaria,
        });
        assert!(result.is_ok());
        assert!(result.unwrap().primeiro_acesso);
    }

    #[test]
    fn test_funcionario_nao_pode_criar_usuario() {
        let conn = setup_db();
        let admin_session = login(&conn, LoginRequest {
            login: "admin".into(),
            senha: "admin123".into(),
        }).unwrap().session;

        let create_resp = criar_usuario(&conn, &admin_session, CreateUser {
            login: "func1".into(),
            role: "funcionario".into(),
            funcionario_id: None,
        }).unwrap();

        // Login como funcionário
        let func_session = login(&conn, LoginRequest {
            login: "func1".into(),
            senha: create_resp.senha_temporaria,
        }).unwrap().session;

        // Tentar criar usuário
        let result = criar_usuario(&conn, &func_session, CreateUser {
            login: "outro".into(),
            role: "funcionario".into(),
            funcionario_id: None,
        });
        assert!(matches!(result, Err(AppError::Forbidden(_))));
    }
}
pub fn gerar_usuarios_para_funcionarios(
    conn: &Connection,
) -> Result<i32, AppError> {
    let mut stmt = conn.prepare("
        SELECT f.id, f.nome
        FROM funcionarios f
        LEFT JOIN users u ON u.funcionario_id = f.id
        WHERE u.id IS NULL
    ")?;

    let funcionarios = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut count = 0;

    for func in funcionarios {
        let (id, _nome) = func?;

        let login = format!("func{}", id);

        // senha padrão simples (depois pode melhorar)
        let senha_hash = hasher::hash_password("123456").unwrap();
        conn.execute(
            "INSERT INTO users (login, senha_hash, role, funcionario_id, ativo)
             VALUES (?1, ?2, 'funcionario', ?3, 1)",
            params![login, senha_hash, id],
        )?;

        count += 1;
    }

    Ok(count)
}
