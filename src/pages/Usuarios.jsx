import { useState, useEffect } from "react";
import { 
  Title, 
  Text, 
  Paper, 
  Table, 
  Button, 
  Group, 
  ActionIcon, 
  Badge, 
  Modal, 
  TextInput, 
  Select, 
  Stack,
  Loader,
  Center,
  CopyButton,
  Divider,
  Menu
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { 
  IconUserPlus, 
  IconRefresh, 
  IconCheck, 
  IconShield,
  IconLock,
  IconPower,
  IconDotsVertical,
  IconEdit,
  IconTrash
} from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";
import { notifications } from "@mantine/notifications";

export default function Usuarios() {
  const [users, setUsers] = useState([]);
  const [funcionarios, setFuncionarios] = useState([]);
  const [loading, setLoading] = useState(true);
  const [createOpened, setCreateOpened] = useState(false);
  const [editOpened, setEditOpened] = useState(false);
  const [resetModalData, setResetModalData] = useState(null);
  const [userEditando, setUserEditando] = useState(null);
  // Estado para confirmação de ações destrutivas
  const [confirmAction, setConfirmAction] = useState(null);

  const createForm = useForm({
    initialValues: {
      login: "",
      role: "funcionario",
      funcionario_id: null,
    },
    validate: {
      login: (value) => (value.length >= 3 ? null : "Login deve ter no mínimo 3 caracteres"),
      role: (value) => (value ? null : "Selecione uma role"),
    },
  });

  const editForm = useForm({
    initialValues: {
      login: "",
      role: "",
      funcionario_id: null,
    },
    validate: {
      login: (value) => (value.length >= 3 ? null : "Login deve ter no mínimo 3 caracteres"),
      role: (value) => (value ? null : "Selecione uma role"),
    },
  });

  const loadData = async () => {
    setLoading(true);
    try {
      const [usersList, funcList] = await Promise.all([
        invoke("cmd_listar_usuarios"),
        invoke("listar_funcionarios_ativos"),
      ]);
      setUsers(usersList);
      setFuncionarios(funcList.map(f => ({ value: f.id.toString(), label: f.nome })));
    } catch (error) {
      notifications.show({ title: "Erro ao carregar dados", message: error.toString(), color: "red" });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const handleCreate = async (values) => {
    try {
      const resp = await invoke("cmd_criar_usuario", { 
        data: { 
          ...values, 
          funcionario_id: values.funcionario_id ? parseInt(values.funcionario_id) : null 
        } 
      });
      
      setCreateOpened(false);
      createForm.reset();
      loadData();
      
      setResetModalData({
        login: resp.user.login,
        senha_temporaria: resp.senha_temporaria,
        title: "Usuário Criado"
      });
    } catch (error) {
      notifications.show({ title: "Erro ao criar usuário", message: error.toString(), color: "red" });
    }
  };

  const handleToggleAtivo = (userId, login, atualAtivo) => {
    const acao = atualAtivo ? "desativar" : "ativar";
    setConfirmAction({
      title: `${atualAtivo ? "Desativar" : "Ativar"} Usuário`,
      message: `Deseja realmente ${acao} o usuário ${login}?`,
      color: atualAtivo ? "red" : "green",
      confirmLabel: atualAtivo ? "Desativar" : "Ativar",
      onConfirm: async () => {
        try {
          await invoke("cmd_ativar_desativar_usuario", { userId, ativo: !atualAtivo });
          notifications.show({ title: "Sucesso", message: `Usuário ${login} ${atualAtivo ? "desativado" : "ativado"}`, color: "green" });
          loadData();
        } catch (error) {
          notifications.show({ title: "Erro", message: error.toString(), color: "red" });
        }
      }
    });
  };

  const handleResetPassword = (userId, login) => {
    setConfirmAction({
      title: "Resetar Senha",
      message: `Deseja realmente resetar a senha do usuário ${login}?`,
      color: "orange",
      confirmLabel: "Resetar Senha",
      onConfirm: async () => {
        try {
          const resp = await invoke("cmd_resetar_senha", { userId });
          setResetModalData({
            login,
            senha_temporaria: resp.senha_temporaria,
            title: "Senha Resetada"
          });
        } catch (error) {
          notifications.show({ title: "Erro ao resetar senha", message: error.toString(), color: "red" });
        }
      }
    });
  };

  const openEdit = (user) => {
    setUserEditando(user);
    editForm.setValues({
      login: user.login,
      role: user.role.toLowerCase(),
      funcionario_id: user.funcionario_id ? user.funcionario_id.toString() : null,
    });
    setEditOpened(true);
  };

  const handleEdit = async (values) => {
    try {
      await invoke("cmd_editar_usuario", {
        data: {
          id: userEditando.id,
          login: values.login,
          role: values.role,
          funcionario_id: values.funcionario_id ? parseInt(values.funcionario_id) : null
        }
      });
      
      setEditOpened(false);
      notifications.show({ title: "Sucesso", message: "Usuário atualizado", color: "green" });
      loadData();
    } catch (error) {
      notifications.show({ title: "Erro ao atualizar usuário", message: error.toString(), color: "red" });
    }
  };

  const handleDelete = (userId, login) => {
    setConfirmAction({
      title: "Excluir Permanentemente",
      message: `Deseja realmente EXCLUIR o usuário ${login}? Esta ação não pode ser desfeita.`,
      color: "red",
      confirmLabel: "Excluir",
      onConfirm: async () => {
        try {
          await invoke("cmd_excluir_usuario", { userId });
          notifications.show({ title: "Sucesso", message: `Usuário ${login} excluído`, color: "green" });
          loadData();
        } catch (error) {
          notifications.show({ title: "Erro ao excluir", message: error.toString(), color: "red" });
        }
      }
    });
  };

  if (loading && users.length === 0) {
    return <Center style={{ height: "400px" }}><Loader size="xl" /></Center>;
  }

  return (
    <Stack>
      <Group justify="space-between" align="flex-end">
        <div>
          <Title order={2}>Gestão de Usuários</Title>
          <Text c="dimmed" size="sm">Controle de acesso ao sistema</Text>
        </div>
        <Button radius="md" leftSection={<IconUserPlus size={16}/>} onClick={() => setCreateOpened(true)}>
          Novo Usuário
        </Button>
      </Group>

      <Paper withBorder p="md" radius="md" mt="md">
        <Table verticalSpacing="sm" highlightOnHover>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Login</Table.Th>
              <Table.Th>Role</Table.Th>
              <Table.Th>Funcionário Vinculado</Table.Th>
              <Table.Th>Situação</Table.Th>
              <Table.Th>Ações</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {users.map((u) => (
              <Table.Tr key={u.id} style={{ opacity: u.ativo ? 1 : 0.6 }}>
                <Table.Td>
                  <Group gap="sm">
                    <IconShield size={16} color={u.role === "Admin" ? "var(--mantine-color-blue-6)" : "gray"} />
                    <Text fw={500}>{u.login}</Text>
                  </Group>
                </Table.Td>
                <Table.Td>
                  <Badge color={u.role === "Admin" ? "blue" : "gray"} variant="light" radius="sm">
                    {u.role}
                  </Badge>
                </Table.Td>
                <Table.Td>{u.funcionario_nome || "-"}</Table.Td>
                <Table.Td>
                  <Group gap={4}>
                    {u.ativo ? (
                      <Badge color="green" variant="dot">Ativo</Badge>
                    ) : (
                      <Badge color="red" variant="dot">Inativo</Badge>
                    )}
                    {u.primeiro_acesso && <Badge color="orange" size="xs">Pendente Senha</Badge>}
                  </Group>
                </Table.Td>
                <Table.Td>
                  <Menu shadow="md" width={200} position="bottom-end">
                    <Menu.Target>
                      <ActionIcon variant="subtle" color="gray">
                        <IconDotsVertical size={18} />
                      </ActionIcon>
                    </Menu.Target>

                    <Menu.Dropdown>
                      <Menu.Label>Conta</Menu.Label>
                      <Menu.Item 
                        leftSection={<IconEdit size={14} />} 
                        onClick={() => openEdit(u)}
                        disabled={u.login === "admin"}
                      >
                        Editar Usuário
                      </Menu.Item>
                      
                      <Menu.Item 
                        leftSection={<IconRefresh size={14} />} 
                        onClick={() => handleResetPassword(u.id, u.login)}
                        disabled={u.login === "admin" || !u.ativo}
                        color="orange"
                      >
                        Resetar Senha
                      </Menu.Item>

                      <Menu.Divider />
                      
                      <Menu.Label>Status</Menu.Label>
                      <Menu.Item
                        leftSection={<IconPower size={14} />}
                        color={u.ativo ? "red" : "green"}
                        onClick={() => handleToggleAtivo(u.id, u.login, u.ativo)}
                        disabled={u.login === "admin"}
                      >
                        {u.ativo ? "Desativar" : "Ativar"}
                      </Menu.Item>

                      <Menu.Item
                        leftSection={<IconTrash size={14} />}
                        color="red"
                        onClick={() => handleDelete(u.id, u.login)}
                        disabled={u.login === "admin"}
                      >
                        Excluir Permanentemente
                      </Menu.Item>
                    </Menu.Dropdown>
                  </Menu>
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      </Paper>

      {/* Modal Criar Usuário */}
      <Modal opened={createOpened} onClose={() => setCreateOpened(false)} title="Novo Usuário" centered radius="md">
        <form onSubmit={createForm.onSubmit(handleCreate)}>
          <Stack>
            <TextInput label="Login" placeholder="ex: joao.silva" required {...createForm.getInputProps("login")} />
            <Select 
              label="Role" placeholder="Selecione..." 
              data={[
                { value: "admin", label: "Administrador (Acesso Total)" },
                { value: "funcionario", label: "Funcionário (Acesso Restrito)" }
              ]}
              required {...createForm.getInputProps("role")}
            />
            <Select label="Funcionário Vinculado" placeholder="Selecione (opcional)..." data={funcionarios} clearable {...createForm.getInputProps("funcionario_id")} />
            <Button type="submit" fullWidth mt="md" radius="md">Criar Usuário</Button>
          </Stack>
        </form>
      </Modal>

      {/* Modal Editar Usuário */}
      <Modal opened={editOpened} onClose={() => setEditOpened(false)} title="Editar Usuário" centered radius="md">
        <form onSubmit={editForm.onSubmit(handleEdit)}>
          <Stack>
            <TextInput label="Login" required {...editForm.getInputProps("login")} />
            <Select 
              label="Role" placeholder="Selecione..." 
              data={[
                { value: "admin", label: "Administrador (Acesso Total)" },
                { value: "funcionario", label: "Funcionário (Acesso Restrito)" }
              ]}
              required {...editForm.getInputProps("role")}
            />
            <Select label="Funcionário Vinculado" placeholder="Selecione (opcional)..." data={funcionarios} clearable {...editForm.getInputProps("funcionario_id")} />
            <Button type="submit" fullWidth mt="md" radius="md">Salvar Alterações</Button>
          </Stack>
        </form>
      </Modal>

      {/* Modal Senha Temporária */}
      <Modal opened={!!resetModalData} onClose={() => setResetModalData(null)} title={resetModalData?.title} centered radius="md" padding="xl">
        <Stack align="center" gap="xs">
          <IconLock size={48} color="var(--mantine-color-orange-6)" />
          <Text ta="center" size="sm" c="dimmed" mb="md">
            Forneça as credenciais abaixo ao usuário <b>{resetModalData?.login}</b>. Ele deverá alterar a senha no primeiro acesso.
          </Text>
          <Paper withBorder p="md" w="100%" radius="md" style={{ backgroundColor: "var(--app-bg)" }}>
            <Stack gap="xs">
              <div><Text size="xs" fw={700} tt="uppercase" c="dimmed">Login</Text><Text fw={600}>{resetModalData?.login}</Text></div>
              <Divider />
              <div>
                <Text size="xs" fw={700} tt="uppercase" c="dimmed">Senha Temporária</Text>
                <Group justify="space-between">
                  <Text fw={700} size="xl" c="blue" style={{ letterSpacing: 2 }}>{resetModalData?.senha_temporaria}</Text>
                  <CopyButton value={resetModalData?.senha_temporaria}>
                    {({ copied, copy }) => (
                      <ActionIcon color={copied ? "teal" : "blue"} variant="light" onClick={copy}>
                        {copied ? <IconCheck size={16} /> : <IconPower size={16} />}
                      </ActionIcon>
                    )}
                  </CopyButton>
                </Group>
              </div>
            </Stack>
          </Paper>
          <Button fullWidth mt="xl" onClick={() => setResetModalData(null)} radius="md">Entendido</Button>
        </Stack>
      </Modal>

      {/* Modal de Confirmação Genérico */}
      <Modal
        opened={!!confirmAction}
        onClose={() => setConfirmAction(null)}
        title={confirmAction?.title}
        centered
        radius="md"
        size="sm"
      >
        <Stack>
          <Text size="sm">{confirmAction?.message}</Text>
          <Group justify="flex-end" mt="md">
            <Button variant="default" onClick={() => setConfirmAction(null)} radius="md">
              Cancelar
            </Button>
            <Button
              color={confirmAction?.color || "red"}
              radius="md"
              onClick={async () => {
                setConfirmAction(null);
                await confirmAction?.onConfirm();
              }}
            >
              {confirmAction?.confirmLabel || "Confirmar"}
            </Button>
          </Group>
        </Stack>
      </Modal>
    </Stack>
  );
}
