use serde::{Deserialize, Serialize};

/// Entidade Funcionário — executor dos serviços
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Funcionario {
    pub id: i64,
    pub nome: String,
    pub cargo: Option<String>,
    pub telefone: Option<String>,
    pub ativo: bool,
    pub created_at: String,
}

/// DTO para criação de funcionário
#[derive(Debug, Deserialize)]
pub struct CreateFuncionario {
    pub nome: String,
    pub cargo: Option<String>,
    pub telefone: Option<String>,
}

/// DTO para atualização de funcionário
#[derive(Debug, Deserialize)]
pub struct UpdateFuncionario {
    pub id: i64,
    pub nome: String,
    pub cargo: Option<String>,
    pub telefone: Option<String>,
    pub ativo: bool,
}

/// Resposta ao criar funcionário — inclui credenciais do usuário auto-criado
#[derive(Debug, Serialize)]
pub struct CreateFuncionarioResponse {
    pub funcionario: Funcionario,
    pub login: String,
    pub senha_temporaria: String,
}
