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
  Tooltip,
  Divider
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { 
  IconUserPlus, 
  IconRefresh, 
  IconCopy, 
  IconCheck, 
  IconUsers,
  IconShield,
  IconLock
} from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";
import { notifications } from "@mantine/notifications";

export default function Usuarios() {
  const [users, setUsers] = useState([]);
  const [funcionarios, setFuncionarios] = useState([]);
  const [loading, setLoading] = useState(true);
  const [createOpened, setCreateOpened] = useState(false);
  const [resetModalData, setResetModalData] = useState(null);

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
      notifications.show({
        title: "Erro ao carregar dados",
        message: error.toString(),
        color: "red"
      });
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
      notifications.show({
        title: "Erro ao criar usuário",
        message: error.toString(),
        color: "red"
      });
    }
  };

  const handleResetPassword = async (userId, login) => {
    if (!confirm(`Deseja realmente resetar a senha do usuário ${login}?`)) return;
    
    try {
      const resp = await invoke("cmd_resetar_senha", { userId });
      setResetModalData({
        login,
        senha_temporaria: resp.senha_temporaria,
        title: "Senha Resetada"
      });
    } catch (error) {
      notifications.show({
        title: "Erro ao resetar senha",
        message: error.toString(),
        color: "red"
      });
    }
  };

  if (loading && users.length === 0) {
    return (
      <Center style={{ height: "400px" }}>
        <Loader size="xl" />
      </Center>
    );
  }

  return (
    <Stack>
      <Group justify="space-between" align="flex-end">
        <div>
          <Title order={2}>Gestão de Usuários</Title>
          <Text c="dimmed" size="sm">Controle de acesso ao sistema</Text>
        </div>
        <Button 
          radius="md"
          onClick={() => setCreateOpened(true)}
        >
          Novo Usuário
        </Button>
<Button
  color="green"
  radius="md"
  onClick={async () => {
    try {
      const resp = await invoke("cmd_gerar_usuarios_para_funcionarios");
      notifications.show({
        title: "Sucesso",
        message: resp,
        color: "green"
      });
      loadData();
    } catch (error) {
      notifications.show({
        title: "Erro",
        message: error.toString(),
        color: "red"
      });
    }
  }}
>
  Gerar usuários automaticamente
</Button>
      </Group>

      <Paper withBorder p="md" radius="md" mt="md">
        <Table verticalSpacing="sm" highlightOnHover>
          <Table.Thead>
            <Table.Tr>
              <Table.Th>Login</Table.Th>
              <Table.Th>Role</Table.Th>
              <Table.Th>Funcionário Vinculado</Table.Th>
              <Table.Th>Status</Table.Th>
              <Table.Th>Ações</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {users.map((u) => (
              <Table.Tr key={u.id}>
                <Table.Tr>
                  <Group gap="sm">
                    <IconShield size={16} color={u.role === "Admin" ? "var(--mantine-color-blue-6)" : "gray"} />
                    <Text fw={500}>{u.login}</Text>
                  </Group>
                </Table.Tr>
                <Table.Td>
                  <Badge color={u.role === "Admin" ? "blue" : "gray"} variant="light" radius="sm">
                    {u.role}
                  </Badge>
                </Table.Td>
                <Table.Td>{u.funcionario_nome || "-"}</Table.Td>
                <Table.Td>
                  {u.primeiro_acesso ? (
                    <Badge color="orange" variant="light">Pend. Senha</Badge>
                  ) : (
                    <Badge color="green" variant="light">Ativo</Badge>
                  )}
                </Table.Td>
                <Table.Td>
                  <Tooltip label="Resetar Senha">
                    <ActionIcon 
                      color="orange" 
                      variant="subtle" 
                      onClick={() => handleResetPassword(u.id, u.login)}
                      disabled={u.login === "admin"}
                    >
                      <IconRefresh size={18} />
                    </ActionIcon>
                  </Tooltip>
                </Table.Td>
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      </Paper>

      {/* Modal Criar Usuário */}
      <Modal 
        opened={createOpened} 
        onClose={() => setCreateOpened(false)} 
        title="Novo Usuário"
        centered
        radius="md"
      >
        <form onSubmit={createForm.onSubmit(handleCreate)}>
          <Stack>
            <TextInput 
              label="Login" 
              placeholder="ex: joao.silva" 
              required 
              {...createForm.getInputProps("login")} 
            />
            <Select 
              label="Role" 
              placeholder="Selecione..." 
              data={[
                { value: "admin", label: "Administrador (Acesso Total)" },
                { value: "funcionario", label: "Funcionário (Acesso Restrito)" }
              ]}
              required
              {...createForm.getInputProps("role")}
            />
            <Select 
              label="Funcionário Vinculado" 
              placeholder="Selecione (opcional)..." 
              data={funcionarios}
              clearable
              {...createForm.getInputProps("funcionario_id")}
            />
            <Button type="submit" fullWidth mt="md" radius="md">
              Criar Usuário
            </Button>
          </Stack>
        </form>
      </Modal>

      {/* Modal Senha Temporária */}
      <Modal 
        opened={!!resetModalData} 
        onClose={() => setResetModalData(null)} 
        title={resetModalData?.title}
        centered
        radius="md"
        padding="xl"
      >
        <Stack align="center" gap="xs">
          <IconLock size={48} color="var(--mantine-color-orange-6)" />
          <Text ta="center" size="sm" c="dimmed" mb="md">
            Forneça as credenciais abaixo ao usuário <b>{resetModalData?.login}</b>. 
            Ele deverá alterar a senha no primeiro acesso.
          </Text>
          
          <Paper withBorder p="md" w="100%" radius="md" style={{ backgroundColor: "var(--app-bg)" }}>
            <Stack gap="xs">
              <div>
                <Text size="xs" fw={700} tt="uppercase" c="dimmed">Login</Text>
                <Text fw={600}>{resetModalData?.login}</Text>
              </div>
              <Divider />
              <div>
                <Text size="xs" fw={700} tt="uppercase" c="dimmed">Senha Temporária</Text>
                <Group justify="space-between">
                  <Text fw={700} size="xl" c="blue" style={{ letterSpacing: 2 }}>
                    {resetModalData?.senha_temporaria}
                  </Text>
                  <CopyButton value={resetModalData?.senha_temporaria}>
                    {({ copied, copy }) => (
                      <ActionIcon color={copied ? "teal" : "blue"} variant="light" onClick={copy}>
                        {copied ? <IconCheck size={16} /> : <IconCopy size={16} />}
                      </ActionIcon>
                    )}
                  </CopyButton>
                </Group>
              </div>
            </Stack>
          </Paper>

          <Button fullWidth mt="xl" onClick={() => setResetModalData(null)} radius="md">
            Entendido
          </Button>
        </Stack>
      </Modal>
    </Stack>
  );
}
