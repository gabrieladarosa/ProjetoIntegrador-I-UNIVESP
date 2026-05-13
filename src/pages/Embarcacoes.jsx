import { useState, useEffect, useCallback } from "react";
import {
  Title,
  Button,
  Table,
  TextInput,
  Modal,
  Group,
  Stack,
  Select,
  NumberInput,
  Badge,
  ActionIcon,
  Text,
  Paper,
  Loader,
  Center,
  Tooltip,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconPlus,
  IconSearch,
  IconEdit,
  IconShip,
} from "@tabler/icons-react";
import { useTauriCommand } from "../hooks/useTauriCommand";
import { useAuth } from "../contexts/AuthContext";

const STATUS_LABELS = {
  ativa: "Ativa",
  inativa: "Inativa",
  em_manutencao: "Em Manutenção",
};

// --- TODO: mover para uma tabela no backend essa configuração
const TIPO_OPTIONS = [
  { value: "lancha", label: "Lancha" },
  { value: "veleiro", label: "Veleiro" },
  { value: "iate", label: "Iate" },
  { value: "jet_ski", label: "Jet Ski" },
  { value: "barco_pesca", label: "Barco de Pesca" },
  { value: "catamarã", label: "Catamarã" },
  { value: "outro", label: "Outro" },
];

function Embarcacoes() {
  const [embarcacoes, setEmbarcacoes] = useState([]);
  const [modalAberto, setModalAberto] = useState(false);
  const [editando, setEditando] = useState(null);
  const [busca, setBusca] = useState("");
  const { execute, loading } = useTauriCommand();
  const { isAdmin } = useAuth();

  const form = useForm({
    initialValues: {
      nome: "",
      identificacao: "",
      modelo: "",
      tipo: "",
      comprimento: "",
      ano_fabricacao: "",
      cliente_responsavel: "",
      status: "ativa",
    },
    validate: {
      nome: (v) => (v.trim().length === 0 ? "Nome é obrigatório" : null),
      identificacao: (v) => (v.trim().length === 0 ? "Identificação é obrigatória" : null),
    },
  });

  const carregarDados = useCallback(async () => {
    try {
      const dados = busca.trim()
        ? await execute("buscar_embarcacoes", { termo: busca })
        : await execute("listar_embarcacoes");
      setEmbarcacoes(dados);
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

  const abrirEditar = (emb) => {
    if (!isAdmin) return;
    setEditando(emb);
    form.setValues({
      nome: emb.nome,
      identificacao: emb.identificacao,
      modelo: emb.modelo || "",
      tipo: emb.tipo || "",
      comprimento: emb.comprimento || "",
      ano_fabricacao: emb.ano_fabricacao || "",
      cliente_responsavel: emb.cliente_responsavel || "",
      status: emb.status,
    });
    setModalAberto(true);
  };

  const salvar = async (values) => {
    try {
      const dados = {
        nome: values.nome,
        identificacao: values.identificacao,
        modelo: values.modelo || null,
        tipo: values.tipo || null,
        comprimento: values.comprimento || null,
        ano_fabricacao: values.ano_fabricacao || null,
        cliente_responsavel: values.cliente_responsavel || null,
      };

      if (editando) {
        await execute("atualizar_embarcacao", {
          data: { ...dados, id: editando.id, status: values.status },
        });
        notifications.show({
          title: "Sucesso",
          message: "Embarcação atualizada",
          color: "green",
        });
      } else {
        await execute("criar_embarcacao", { data: dados });
        notifications.show({
          title: "Sucesso",
          message: "Embarcação cadastrada",
          color: "green",
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
          <IconShip size={28} stroke={1.5} color="var(--mantine-color-blue-6)" />
          <Title order={2}>Embarcações</Title>
        </Group>
        {isAdmin && (
          <Button leftSection={<IconPlus size={16} />} onClick={abrirNovo}>
            Nova Embarcação
          </Button>
        )}
      </Group>

      <Paper shadow="xs" p="md" radius="md" mb="md">
        <TextInput
          placeholder="Buscar por nome, identificação ou cliente..."
          leftSection={<IconSearch size={16} />}
          value={busca}
          onChange={(e) => setBusca(e.currentTarget.value)}
        />
      </Paper>

      {loading ? (
        <Center py="xl">
          <Loader />
        </Center>
      ) : embarcacoes.length === 0 ? (
        <Paper shadow="xs" p="xl" radius="md">
          <Center>
            <Stack align="center" gap="xs">
              <IconShip size={48} stroke={1} color="var(--mantine-color-gray-4)" />
              <Text c="dimmed">Nenhuma embarcação cadastrada</Text>
              {isAdmin && (
                <Button variant="light" size="sm" onClick={abrirNovo}>
                  Cadastrar primeira embarcação
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
                <Table.Th>Identificação</Table.Th>
                <Table.Th>Tipo</Table.Th>
                <Table.Th>Modelo</Table.Th>
                <Table.Th>Cliente</Table.Th>
                <Table.Th>Status</Table.Th>
                {isAdmin && <Table.Th w={60}>Ações</Table.Th>}
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {embarcacoes.map((emb) => (
                <Table.Tr key={emb.id}>
                  <Table.Td fw={500}>{emb.nome}</Table.Td>
                  <Table.Td>
                    <Text size="sm" ff="monospace">
                      {emb.identificacao}
                    </Text>
                  </Table.Td>
                  <Table.Td>
                    {emb.tipo
                      ? TIPO_OPTIONS.find((t) => t.value === emb.tipo)?.label || emb.tipo
                      : "—"}
                  </Table.Td>
                  <Table.Td>{emb.modelo || "—"}</Table.Td>
                  <Table.Td>{emb.cliente_responsavel || "—"}</Table.Td>
                  <Table.Td>
                    <Badge
                      variant="light"
                      className={`status-${emb.status}`}
                      size="sm"
                    >
                      {STATUS_LABELS[emb.status] || emb.status}
                    </Badge>
                  </Table.Td>
                  {isAdmin && (
                    <Table.Td>
                      <Tooltip label="Editar">
                        <ActionIcon
                          variant="subtle"
                          color="blue"
                          onClick={() => abrirEditar(emb)}
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
        title={editando ? "Editar Embarcação" : "Nova Embarcação"}
        size="lg"
      >
        <form onSubmit={form.onSubmit(salvar)}>
          <Stack gap="sm">
            <Group grow>
              <TextInput
                label="Nome"
                placeholder="Nome da embarcação"
                required
                {...form.getInputProps("nome")}
              />
              <TextInput
                label="Identificação"
                placeholder="Número de registro"
                required
                {...form.getInputProps("identificacao")}
              />
            </Group>
            <Group grow>
              <Select
                label="Tipo"
                placeholder="Selecione o tipo"
                data={TIPO_OPTIONS}
                clearable
                {...form.getInputProps("tipo")}
              />
              <TextInput
                label="Modelo"
                placeholder="Modelo da embarcação"
                {...form.getInputProps("modelo")}
              />
            </Group>
            <Group grow>
              <NumberInput
                label="Comprimento (m)"
                placeholder="Em metros"
                decimalScale={2}
                min={0}
                {...form.getInputProps("comprimento")}
              />
              <NumberInput
                label="Ano de Fabricação"
                placeholder="Ex: 2020"
                min={1900}
                max={2030}
                {...form.getInputProps("ano_fabricacao")}
              />
            </Group>
            <TextInput
              label="Funcionário Responsável"
              placeholder="Nome do funcionário responsável"
              {...form.getInputProps("cliente_responsavel")}
            />
            {editando && (
              <Select
                label="Status"
                data={[
                  { value: "ativa", label: "Ativa" },
                  { value: "inativa", label: "Inativa" },
                  { value: "em_manutencao", label: "Em Manutenção" },
                ]}
                {...form.getInputProps("status")}
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
    </>
  );
}

export default Embarcacoes;
