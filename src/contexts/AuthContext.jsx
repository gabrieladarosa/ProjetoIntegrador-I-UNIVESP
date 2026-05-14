import { createContext, useContext, useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { notifications } from "@mantine/notifications";

const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  const checkSession = useCallback(async () => {
    try {
      const session = await invoke("cmd_sessao_atual");
      setUser(session);
    } catch (error) {
      console.error("Erro ao verificar sessão:", error);
      setUser(null);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    checkSession();
  }, [checkSession]);

  const login = async (login, senha) => {
    try {
      const response = await invoke("cmd_login", { data: { login, senha } });
      setUser(response.session);
      return response;
    } catch (error) {
      notifications.show({
        title: "Erro de login",
        message: error.toString(),
        color: "red",
      });
      throw error;
    }
  };

  const logout = async () => {
    try {
      await invoke("cmd_logout");
      setUser(null);
      notifications.show({
        title: "Sessão encerrada",
        message: "Até logo!",
        color: "blue",
      });
    } catch (error) {
      console.error("Erro ao fazer logout:", error);
    }
  };

  const trocarSenha = async (senhaAtual, novaSenha) => {
    try {
      await invoke("cmd_trocar_senha", { data: { senha_atual: senhaAtual, nova_senha: novaSenha } });
      // Atualizar estado local após troca
      const session = await invoke("cmd_sessao_atual");
      setUser(session);
      notifications.show({
        title: "Sucesso",
        message: "Senha alterada com sucesso!",
        color: "green",
      });
    } catch (error) {
      notifications.show({
        title: "Erro ao trocar senha",
        message: error.toString(),
        color: "red",
      });
      throw error;
    }
  };

  const value = {
    user,
    loading,
    isAuthenticated: !!user,
    isFirstAccess: user?.primeiro_acesso || false,
    isAdmin: user?.role === "Admin",
    funcionarioId: user?.funcionario_id || null,
    login,
    logout,
    trocarSenha,
    checkSession,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth deve ser usado dentro de um AuthProvider");
  }
  return context;
}
