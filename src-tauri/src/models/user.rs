use serde::{Deserialize, Serialize};

/// Role do usuário no sistema — Guard Pattern com enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Role {
    Admin,
    Funcionario,
}

impl Role {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "admin" => Ok(Role::Admin),
            "funcionario" => Ok(Role::Funcionario),
            _ => Err(format!("Role inválida: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::Funcionario => "funcionario",
        }
    }
}

/// Entidade User — controle de acesso ao sistema
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub login: String,
    #[serde(skip_serializing)]
    pub senha_hash: String,
    pub role: String,
    pub funcionario_id: Option<i64>,
    pub primeiro_acesso: bool,
    pub ativo: bool,
    pub created_at: String,
    // Campo joined para exibição
    pub funcionario_nome: Option<String>,
}

/// Sessão ativa do usuário — mantida em memória no AppState
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub user_id: i64,
    pub login: String,
    pub role: Role,
    pub primeiro_acesso: bool,
    pub funcionario_id: Option<i64>,
}

/// DTO para login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub senha: String,
}

/// Resposta do login
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub session: Session,
    pub primeiro_acesso: bool,
}

/// DTO para troca de senha
#[derive(Debug, Deserialize)]
pub struct TrocarSenhaRequest {
    pub senha_atual: String,
    pub nova_senha: String,
}

/// DTO para criação de usuário (admin only)
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub login: String,
    pub role: String,
    pub funcionario_id: Option<i64>,
}

/// Resposta da criação de usuário (inclui senha temporária)
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user: User,
    pub senha_temporaria: String,
}

/// Resposta do reset de senha (inclui nova senha temporária)
#[derive(Debug, Serialize)]
pub struct ResetSenhaResponse {
    pub senha_temporaria: String,
}

/// DTO para atualização de usuário (admin only)
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: i64,
    pub login: String,
    pub role: String,
    pub funcionario_id: Option<i64>,
}
