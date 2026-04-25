mod auth;
mod commands;
mod db;
mod error;
mod models;
mod repositories;
mod services;

use tauri::Manager;
use std::sync::Mutex;

pub mod state {
    use rusqlite::Connection;
    use std::sync::Mutex;
    use crate::models::user::Session;

    /// Estado global da aplicação — Dependency Inversion via Tauri State
    pub struct AppState {
        pub db: Mutex<Connection>,
        pub session: Mutex<Option<Session>>,
    }
}

use commands::embarcacao_commands::*;
use commands::funcionario_commands::*;
use commands::servico_commands::*;
use commands::auth_commands::*;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Obter diretório de dados da aplicação
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Erro ao obter diretório de dados da aplicação");

            // Inicializar banco de dados
            let conn = db::initialize(app_data_dir)
                .expect("Erro ao inicializar banco de dados");

            // Registrar estado global
            app.manage(AppState {
                db: Mutex::new(conn),
                session: Mutex::new(None),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Autenticação
            cmd_login,
            cmd_logout,
            cmd_trocar_senha,
            cmd_resetar_senha,
            cmd_criar_usuario,
            cmd_listar_usuarios,
            cmd_gerar_usuarios_para_funcionarios,   
            cmd_sessao_atual,
            // Embarcação
            criar_embarcacao,
            atualizar_embarcacao,
            listar_embarcacoes,
            buscar_embarcacoes,
            // Funcionário
            criar_funcionario,
            atualizar_funcionario,
            listar_funcionarios,
            listar_funcionarios_ativos,
            buscar_funcionarios,
            // Serviço
            criar_servico,
            atualizar_status_servico,
            listar_servicos,
            listar_servicos_por_embarcacao,
        ])

        .run(tauri::generate_context!())
        .expect("Erro ao executar aplicação Tauri");
}
