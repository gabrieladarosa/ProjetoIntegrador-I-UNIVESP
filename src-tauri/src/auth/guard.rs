use crate::error::AppError;
use crate::models::user::{Role, Session};

/// Guard Pattern — controle de acesso reutilizável
/// Princípio: Open/Closed — adicionar novas roles não requer alterar guards existentes

/// Verifica se existe sessão ativa, retornando referência à sessão
pub fn require_authenticated(session: &Option<Session>) -> Result<&Session, AppError> {
    session
        .as_ref()
        .ok_or_else(|| AppError::Unauthorized("Sessão não encontrada. Faça login.".into()))
}

/// Verifica se a sessão possui a role exigida
pub fn require_role(session: &Session, required: &Role) -> Result<(), AppError> {
    if session.role != *required {
        return Err(AppError::Forbidden(format!(
            "Acesso negado. Role necessária: {}",
            required.as_str()
        )));
    }
    Ok(())
}

/// Convenience: exige role admin
pub fn require_admin(session: &Session) -> Result<(), AppError> {
    require_role(session, &Role::Admin)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session_admin() -> Session {
        Session {
            user_id: 1,
            login: "admin".into(),
            role: Role::Admin,
            primeiro_acesso: false,
            funcionario_id: None,
        }
    }

    fn session_func() -> Session {
        Session {
            user_id: 2,
            login: "joao".into(),
            role: Role::Funcionario,
            primeiro_acesso: false,
            funcionario_id: Some(10),
        }
    }

    #[test]
    fn test_require_authenticated_none() {
        let none: Option<Session> = None;
        assert!(require_authenticated(&none).is_err());
    }

    #[test]
    fn test_require_authenticated_some() {
        let some = Some(session_admin());
        assert!(require_authenticated(&some).is_ok());
    }

    #[test]
    fn test_require_admin_ok() {
        assert!(require_admin(&session_admin()).is_ok());
    }

    #[test]
    fn test_require_admin_forbidden() {
        assert!(require_admin(&session_func()).is_err());
    }
}
