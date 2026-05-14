use serde::{Deserialize, Serialize};

/// Entidade Serviço — registro de atividades realizadas
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Servico {
    pub id: i64,
    pub embarcacao_id: i64,
    pub funcionario_id: i64,
    pub descricao: String,
    pub data_execucao: String,
    pub status: String,
    pub observacao: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    // Campos de auditoria
    pub created_by_user_id: Option<i64>,
    pub updated_by_user_id: Option<i64>,
    // Campos joined para exibição
    pub embarcacao_nome: Option<String>,
    pub funcionario_nome: Option<String>,
}

/// DTO para criação de serviço
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateServico {
    pub embarcacao_id: i64,
    pub funcionario_id: i64,
    pub descricao: String,
    pub data_execucao: String,
    pub observacao: Option<String>,
}

/// DTO para atualização de serviço (campos opcionais)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateServico {
    pub id: i64,
    pub descricao: Option<String>,
    pub data_execucao: Option<String>,
    pub status: Option<String>,
    pub observacao: Option<String>,
    pub funcionario_id: Option<i64>,
}
