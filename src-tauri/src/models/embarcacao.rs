use serde::{Deserialize, Serialize};

/// Entidade Embarcação — ativo principal do sistema
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embarcacao {
    pub id: i64,
    pub nome: String,
    pub identificacao: String,
    pub modelo: Option<String>,
    pub tipo: Option<String>,
    pub comprimento: Option<f64>,
    pub ano_fabricacao: Option<i32>,
    pub cliente_responsavel: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    // Campos vinculados
    pub funcionario_id: Option<i64>,
    pub funcionario_nome: Option<String>,
}

/// DTO para criação de embarcação
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateEmbarcacao {
    pub nome: String,
    pub identificacao: String,
    pub modelo: Option<String>,
    pub tipo: Option<String>,
    pub comprimento: Option<f64>,
    pub ano_fabricacao: Option<i32>,
    pub cliente_responsavel: Option<String>,
    pub funcionario_id: Option<i64>,
}

/// DTO para atualização de embarcação
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateEmbarcacao {
    pub id: i64,
    pub nome: String,
    pub identificacao: String,
    pub modelo: Option<String>,
    pub tipo: Option<String>,
    pub comprimento: Option<f64>,
    pub ano_fabricacao: Option<i32>,
    pub cliente_responsavel: Option<String>,
    pub status: String,
    pub funcionario_id: Option<i64>,
}
