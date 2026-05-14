use rusqlite::Connection;

use crate::error::AppError;
use crate::models::servico::{CreateServico, Servico, UpdateServico};
use crate::models::user::{Session, Role};
use crate::repositories::{embarcacao_repository, funcionario_repository, servico_repository};

/// Service Layer — regras de negócio e invariantes de Serviço
/// Aqui ficam todas as invariantes críticas do domínio (INV01, INV02, INV03)

pub fn criar(conn: &Connection, session: &Session, mut data: CreateServico) -> Result<Servico, AppError> {
    // RBAC: Se funcionário, auto-assign funcionario_id da sessão
    if session.role == Role::Funcionario {
        if let Some(fid) = session.funcionario_id {
            data.funcionario_id = fid;
        } else {
            return Err(AppError::Forbidden("Usuário funcionário sem ID de funcionário vinculado".into()));
        }

        // Valida que a embarcação pertence ao funcionário (ou está livre se for o caso, 
        // mas o plano diz que funcionário só acessa vinculadas)
        let emb = embarcacao_repository::find_by_id(conn, data.embarcacao_id)?;
        if emb.funcionario_id != Some(data.funcionario_id) {
            return Err(AppError::Forbidden("Você não tem permissão para registrar serviços nesta embarcação".into()));
        }
    }

    // INV01 — serviço sem embarcação não existe
    embarcacao_repository::find_by_id(conn, data.embarcacao_id)
        .map_err(|_| AppError::Validation("Embarcação selecionada não existe".into()))?;

    // INV02 — serviço sem funcionário não existe
    let funcionario = funcionario_repository::find_by_id(conn, data.funcionario_id)
        .map_err(|_| AppError::Validation("Funcionário selecionado não existe".into()))?;

    // Validação adicional: funcionário deve estar ativo
    if !funcionario.ativo {
        return Err(AppError::Validation("Funcionário selecionado está inativo".into()));
    }

    // Validações de campos obrigatórios
    if data.descricao.trim().is_empty() {
        return Err(AppError::Validation("Descrição do serviço é obrigatória".into()));
    }
    if data.data_execucao.trim().is_empty() {
        return Err(AppError::Validation("Data de execução é obrigatória".into()));
    }

    servico_repository::insert(conn, &data, session.user_id)
}

pub fn atualizar(
    conn: &Connection,
    session: &Session,
    data: UpdateServico,
) -> Result<Servico, AppError> {
    let servico_atual = servico_repository::find_by_id(conn, data.id)?;

    // RBAC: Funcionário só edita próprios serviços não concluídos
    if session.role == Role::Funcionario {
        if servico_atual.funcionario_id != session.funcionario_id.unwrap_or(0) {
            return Err(AppError::Forbidden("Acesso negado. Você só pode editar seus próprios serviços.".into()));
        }

        if servico_atual.status == "concluido" {
            return Err(AppError::Forbidden("Serviços concluídos não podem ser editados por funcionários.".into()));
        }

        // Funcionário não pode mudar status para concluído (somente admin no plano original, 
        // mas vamos permitir mudar para 'em_execucao' se estiver em outro status se necessário)
        if let Some(ref st) = data.status {
            if st == "concluido" {
                return Err(AppError::Forbidden("Somente administradores podem concluir serviços.".into()));
            }
        }
        
        // Bloquear alteração de funcionário_id por funcionário
        if data.funcionario_id.is_some() && data.funcionario_id != session.funcionario_id {
            return Err(AppError::Forbidden("Você não pode reatribuir este serviço.".into()));
        }
    }

    // Validação de Status
    if let Some(ref st) = data.status {
        let status_validos = ["em_execucao", "concluido"]; // 'pendente' foi removido no plano
        if !status_validos.contains(&st.as_str()) {
            return Err(AppError::Validation(
                format!("Status inválido. Use: {}", status_validos.join(", "))
            ));
        }

        // INV03 — serviço concluído não pode ser reaberto (nem por admin, conforme plano/discussão)
        if servico_atual.status == "concluido" && st != "concluido" {
            return Err(AppError::Validation(
                "Serviço concluído não pode ter status alterado".into()
            ));
        }
    }

    servico_repository::update(conn, &data, session.user_id)
}

pub fn listar(conn: &Connection, session: &Session) -> Result<Vec<Servico>, AppError> {
    if session.role == Role::Admin {
        servico_repository::list_all(conn)
    } else {
        let fid = session.funcionario_id.ok_or_else(|| AppError::Forbidden("Usuário sem vínculo".into()))?;
        servico_repository::list_by_funcionario(conn, fid)
    }
}

pub fn listar_por_embarcacao(conn: &Connection, session: &Session, embarcacao_id: i64) -> Result<Vec<Servico>, AppError> {
    let emb = embarcacao_repository::find_by_id(conn, embarcacao_id)?;
    
    // RBAC: Funcionário só vê serviços de embarcações vinculadas
    if session.role == Role::Funcionario {
        if emb.funcionario_id != session.funcionario_id {
            return Err(AppError::Forbidden("Acesso negado a esta embarcação".into()));
        }
    }

    servico_repository::list_by_embarcacao(conn, embarcacao_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use crate::db::migrations;
    use crate::models::embarcacao::CreateEmbarcacao;
    use crate::models::funcionario::CreateFuncionario;
    use crate::models::user::{Role, Session};
    use crate::services::{embarcacao_service, funcionario_service};

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrations::run(&conn).unwrap();
        conn
    }

    fn admin_session() -> Session {
        Session {
            user_id: 1,
            login: "admin".into(),
            role: Role::Admin,
            primeiro_acesso: false,
            funcionario_id: None,
        }
    }

    fn func_session(fid: i64) -> Session {
        Session {
            user_id: 1, // Usar 1 (admin seed) para evitar erro de FK em testes simples
            login: "funcionario".into(),
            role: Role::Funcionario,
            primeiro_acesso: false,
            funcionario_id: Some(fid),
        }
    }

    #[test]
    fn test_criar_servico_valida_embarcacao_existente() {
        let conn = setup_db();
        let data = CreateServico {
            embarcacao_id: 1,
            funcionario_id: 1,
            descricao: "Teste".into(),
            data_execucao: "2024-01-01".into(),
            observacao: None,
        };
        
        let result = criar(&conn, &admin_session(), data);
        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[test]
    fn test_funcionario_auto_assign_e_vinculo_embarcacao() {
        let conn = setup_db();
        
        // Criar funcionário 1 e funcionário 2
        let f1 = funcionario_service::criar(&conn, CreateFuncionario { nome: "F1".into(), cargo: None, telefone: None }).unwrap();
        let f2 = funcionario_service::criar(&conn, CreateFuncionario { nome: "F2".into(), cargo: None, telefone: None }).unwrap();

        // Criar embarcação vinculada ao F1
        let emb = embarcacao_service::criar(&conn, CreateEmbarcacao {
            nome: "Emb 1".into(), identificacao: "ID1".into(),
            modelo: None, tipo: None, comprimento: None, ano_fabricacao: None, cliente_responsavel: None,
            funcionario_id: Some(f1.id),
        }).unwrap();

        let data = CreateServico {
            embarcacao_id: emb.id,
            funcionario_id: 0, // Será ignorado
            descricao: "Serviço".into(),
            data_execucao: "2024-01-01".into(),
            observacao: None,
        };

        // F2 tenta registrar na embarcação do F1 -> Erro
        let res_f2 = criar(&conn, &func_session(f2.id), data.clone());
        assert!(matches!(res_f2, Err(AppError::Forbidden(_))));

        // F1 registra na própria -> OK
        let res_f1 = criar(&conn, &func_session(f1.id), data);
        assert!(res_f1.is_ok());
        assert_eq!(res_f1.unwrap().funcionario_id, f1.id);
    }
}
