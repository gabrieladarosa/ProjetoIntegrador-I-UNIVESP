import { useState, useEffect, useCallback } from "react";
import {
  Title,
  Button,
  Table,
  TextInput,
  Modal,
  Group,
  Stack,
  Switch,
  Badge,
  ActionIcon,
  Text,
  Paper,
  Loader,
  Center,
  Tooltip,
  Divider,
  CopyButton,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconPlus,
  IconSearch,
  IconEdit,
  IconUsers,
  IconLock,
  IconCheck,
} from "@tabler/icons-react";
import { useTauriCommand } from "../hooks/useTauriCommand";
import { useAuth } from "../contexts/AuthContext";
import { formatPhoneNumber, normalizePhoneNumber } from "../utils/phone";

function Funcionarios() {
  const [funcionarios, setFuncionarios] = useState([]);
  const [modalAberto, setModalAberto] = useState(false);
  const [editando, setEditando] = useState(null);
  const [busca, setBusca] = useState("");
  const [credenciais, setCredenciais] = useState(null);
  const { execute, loading } = useTauriCommand();
  const { isAdmin } = useAuth();

  const form = useForm({
    initialValues: {
      nome: "",
      cargo: "",
      telefone: "",
      ativo: true,
    },
    validate: {
      nome: (v) => (v.trim().length === 0 ? "Nome é obrigatório" : null),
    },
  });

  const carregarDados = useCallback(async () => {
    try {
      const dados = busca.trim()
        ? await execute("buscar_funcionarios", { termo: busca })
        : await execute("listar_funcionarios");
      setFuncionarios(dados);
    } catch (err) {
      notifications.show({
        title: "Erro ao carregar",
        message: err,
        color: "red",
      });
    }
  }, [execute, busca]);

  useEffect(() => {
    carregarDados();
  }, [carregarDados]);

  const abrirNovo = () => {
    if (!isAdmin) return;
    setEditando(null);
    form.reset();
    setModalAberto(true);
  };

  const abrirEditar = (func) => {
    if (!isAdmin) return;
    setEditando(func);
    form.setValues({
      nome: func.nome,
      cargo: func.cargo || "",
      telefone: formatPhoneNumber(func.telefone),
      ativo: func.ativo,
    });
    setModalAberto(true);
  };

  const salvar = async (values) => {
    try {
      if (editando) {
        await execute("atualizar_funcionario", {
          data: {
            id: editando.id,
            nome: values.nome,
            cargo: values.cargo || null,
            telefone: normalizePhoneNumber(values.telefone) || null,
            ativo: values.ativo,
          },
        });
        notifications.show({
          title: "Sucesso",
          message: "Funcionário atualizado",
          color: "green",
        });
      } else {
        const resp = await execute("criar_funcionario", {
          data: {
            nome: values.nome,
            cargo: values.cargo || null,
            telefone: normalizePhoneNumber(values.telefone) || null,
          },
        });
        notifications.show({
          title: "Sucesso",
          message: "Funcionário cadastrado com usuário vinculado",
          color: "green",
        });
        // Mostrar credenciais do usuário auto-criado
        setCredenciais({
          nome: resp.funcionario.nome,
          login: resp.login,
          senha_temporaria: resp.senha_temporaria,
        });
      }

      setModalAberto(false);
      form.reset();
      carregarDados();
    } catch (err) {
      notifications.show({
        title: "Erro ao salvar",
        message: err,
        color: "red",
      });
    }
  };

  return (
    <>
      <Group justify="space-between" mb="lg">
        <Group gap="sm">
          <IconUsers size={28} stroke={1.5} color="var(--mantine-color-blue-6)" />
          <Title order={2}>Funcionários</Title>
        </Group>
        {isAdmin && (
          <Button leftSection={<IconPlus size={16} />} onClick={abrirNovo}>
            Novo Funcionário
          </Button>
        )}
      </Group>

      <Paper shadow="xs" p="md" radius="md" mb="md">
        <TextInput
          placeholder="Buscar por nome ou cargo..."
          leftSection={<IconSearch size={16} />}
          value={busca}
          onChange={(e) => setBusca(e.currentTarget.value)}
        />
      </Paper>

      {loading ? (
        <Center py="xl">
          <Loader />
        </Center>
      ) : funcionarios.length === 0 ? (
        <Paper shadow="xs" p="xl" radius="md">
          <Center>
            <Stack align="center" gap="xs">
              <IconUsers size={48} stroke={1} color="var(--mantine-color-gray-4)" />
              <Text c="dimmed">Nenhum funcionário cadastrado</Text>
              {isAdmin && (
                <Button variant="light" size="sm" onClick={abrirNovo}>
                  Cadastrar primeiro funcionário
                </Button>
              )}
            </Stack>
          </Center>
        </Paper>
      ) : (
        <Paper shadow="xs" radius="md" style={{ overflow: "hidden" }}>
          <Table striped highlightOnHover>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Nome</Table.Th>
                <Table.Th>Cargo</Table.Th>
                <Table.Th>Telefone</Table.Th>
                <Table.Th>Situação</Table.Th>
                {isAdmin && <Table.Th w={60}>Ações</Table.Th>}
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {funcionarios.map((func) => (
                <Table.Tr key={func.id} style={{ opacity: func.ativo ? 1 : 0.6 }}>
                  <Table.Td fw={500}>{func.nome}</Table.Td>
                  <Table.Td>{func.cargo || "—"}</Table.Td>
                  <Table.Td>{formatPhoneNumber(func.telefone) || "—"}</Table.Td>
                  <Table.Td>
                    <Badge
                      variant="light"
                      color={func.ativo ? "green" : "red"}
                      size="sm"
                    >
                      {func.ativo ? "Ativo" : "Inativo"}
                    </Badge>
                  </Table.Td>
                  {isAdmin && (
                    <Table.Td>
                      <Tooltip label="Editar">
                        <ActionIcon
                          variant="subtle"
                          color="blue"
                          onClick={() => abrirEditar(func)}
                        >
                          <IconEdit size={16} />
                        </ActionIcon>
                      </Tooltip>
                    </Table.Td>
                  )}
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Paper>
      )}

      {/* Modal de Cadastro/Edição */}
      <Modal
        opened={modalAberto}
        onClose={() => setModalAberto(false)}
        title={editando ? "Editar Funcionário" : "Novo Funcionário"}
        size="md"
      >
        <form onSubmit={form.onSubmit(salvar)}>
          <Stack gap="sm">
            <TextInput
              label="Nome"
              placeholder="Nome completo"
              required
              {...form.getInputProps("nome")}
            />
            <TextInput
              label="Cargo"
              placeholder="Ex: Mecânico, Eletricista, Pintor"
              {...form.getInputProps("cargo")}
            />
            <TextInput
              label="Telefone"
              placeholder="(11) 99999-9999"
              value={form.values.telefone}
              onChange={(event) =>
                form.setFieldValue(
                  "telefone",
                  formatPhoneNumber(event.currentTarget.value)
                )
              }
            />
            {editando && (
              <Switch
                label="Funcionário ativo"
                description="Funcionários inativos não podem ser atribuídos a novos serviços"
                {...form.getInputProps("ativo", { type: "checkbox" })}
              />
            )}
            <Group justify="flex-end" mt="md">
              <Button variant="default" onClick={() => setModalAberto(false)}>
                Cancelar
              </Button>
              <Button type="submit" loading={loading}>
                {editando ? "Salvar Alterações" : "Cadastrar"}
              </Button>
            </Group>
          </Stack>
        </form>
      </Modal>

      {/* Modal de Credenciais do Usuário */}
      <Modal
        opened={!!credenciais}
        onClose={() => setCredenciais(null)}
        title="Usuário Criado Automaticamente"
        centered
        radius="md"
        padding="xl"
      >
        <Stack align="center" gap="xs">
          <IconLock size={48} color="var(--mantine-color-blue-6)" />
          <Text ta="center" size="sm" c="dimmed" mb="md">
            Um usuário foi criado automaticamente para <b>{credenciais?.nome}</b>. Forneça as credenciais abaixo. A senha deverá ser alterada no primeiro acesso.
          </Text>
          <Paper withBorder p="md" w="100%" radius="md" style={{ backgroundColor: "var(--app-bg)" }}>
            <Stack gap="xs">
              <div>
                <Text size="xs" fw={700} tt="uppercase" c="dimmed">Login</Text>
                <Group justify="space-between">
                  <Text fw={600}>{credenciais?.login}</Text>
                  <CopyButton value={credenciais?.login || ""}>
                    {({ copied, copy }) => (
                      <ActionIcon color={copied ? "teal" : "gray"} variant="light" size="sm" onClick={copy}>
                        {copied ? <IconCheck size={14} /> : <IconEdit size={14} />}
                      </ActionIcon>
                    )}
                  </CopyButton>
                </Group>
              </div>
              <Divider />
              <div>
                <Text size="xs" fw={700} tt="uppercase" c="dimmed">Senha Temporária</Text>
                <Group justify="space-between">
                  <Text fw={700} size="xl" c="blue" style={{ letterSpacing: 2 }}>{credenciais?.senha_temporaria}</Text>
                  <CopyButton value={credenciais?.senha_temporaria || ""}>
                    {({ copied, copy }) => (
                      <ActionIcon color={copied ? "teal" : "blue"} variant="light" onClick={copy}>
                        {copied ? <IconCheck size={14} /> : <IconEdit size={14} />}
                      </ActionIcon>
                    )}
                  </CopyButton>
                </Group>
              </div>
            </Stack>
          </Paper>
          <Button fullWidth mt="xl" onClick={() => setCredenciais(null)} radius="md">Entendido</Button>
        </Stack>
      </Modal>
    </>
  );
}

export default Funcionarios;
