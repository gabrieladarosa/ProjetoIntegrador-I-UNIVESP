import { useState, useEffect } from "react";
import {
  Title,
  Select,
  Table,
  Badge,
  Group,
  Stack,
  Paper,
  Text,
  Loader,
  Center,
  ActionIcon,
  Tooltip,
  Menu,
  Modal,
  Textarea,
  Button,
  Checkbox,
  SimpleGrid,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconHistory,
  IconDotsVertical,
  IconCheck,
  IconShip,
  IconEdit,
} from "@tabler/icons-react";
import { useTauriCommand } from "../hooks/useTauriCommand";
import { useAuth } from "../contexts/AuthContext";

const STATUS_CONFIG = {
  em_execucao: { label: "Em Execução", color: "blue" },
  concluido: { label: "Concluído", color: "green" },
};

const servicoOptions = [
  "Limpeza",
  "Lavagem",
  "Motor",
  "Guincho da âncora",
  "Luzes de navegação",
  "Rádio de som",
  "radio VHF",
  "Buzina",
].sort((a, b) => a.localeCompare(b, "pt-BR", { sensitivity: "base" }));

const servicoOptionColumns = [
  servicoOptions.slice(0, 4),
  servicoOptions.slice(4, 8),
];

function Historico() {
  const [embarcacoes, setEmbarcacoes] = useState([]);
  const [servicos, setServicos] = useState([]);
  const [embarcacaoSelecionada, setEmbarcacaoSelecionada] = useState(null);
  const [carregandoServicos, setCarregandoServicos] = useState(false);
  const [editando, setEditando] = useState(null);
  
  const { execute, loading } = useTauriCommand();
  const { isAdmin, funcionarioId } = useAuth();

  const editForm = useForm({
    initialValues: {
      descricao: [],
      observacao: "",
      status: "",
    },
    validate: {
      descricao: (v) => (v.length === 0 ? "Selecione ao menos um serviço" : null),
    },
  });

  const carregarEmbarcacoes = async () => {
    try {
      const embs = await execute("listar_embarcacoes");
      setEmbarcacoes(embs);
    } catch (err) {
      notifications.show({ title: "Erro", message: err, color: "red" });
    }
  };

  const carregarServicos = async () => {
    setCarregandoServicos(true);
    try {
      const dados = embarcacaoSelecionada
        ? await execute("listar_servicos_por_embarcacao", { embarcacaoId: Number(embarcacaoSelecionada) })
        : await execute("listar_servicos");
      setServicos(dados);
    } catch (err) {
      notifications.show({ title: "Erro", message: err, color: "red" });
    } finally {
      setCarregandoServicos(false);
    }
  };

  useEffect(() => {
    carregarEmbarcacoes();
  }, [execute]);

  useEffect(() => {
    carregarServicos();
  }, [embarcacaoSelecionada, execute]);

  const abrirEditar = (srv) => {
    setEditando(srv);
    // Converter "Serviço 1, Serviço 2" para ["Serviço 1", "Serviço 2"]
    const descArray = srv.descricao 
      ? srv.descricao.split(", ").filter(s => s.trim() !== "") 
      : [];

    editForm.setValues({
      descricao: descArray,
      observacao: srv.observacao || "",
      status: srv.status,
    });
  };

  const salvarEdicao = async (values) => {
    try {
      await execute("atualizar_servico", {
        data: {
          id: editando.id,
          descricao: values.descricao.join(", "),
          observacao: values.observacao || null,
          status: values.status,
          funcionario_id: null, // Mantém o atual
          data_execucao: null, // Mantém o atual
        },
      });

      notifications.show({ title: "Sucesso", message: "Serviço atualizado", color: "green" });
      setEditando(null);
      carregarServicos();
    } catch (err) {
      notifications.show({ title: "Erro ao atualizar", message: err, color: "red" });
    }
  };

  const atualizarStatusDireto = async (servicoId, novoStatus) => {
    try {
      await execute("atualizar_servico", {
        data: {
          id: servicoId,
          status: novoStatus,
          descricao: null, observacao: null, funcionario_id: null, data_execucao: null
        },
      });

      notifications.show({
        title: "Status atualizado",
        message: `Serviço marcado como ${STATUS_CONFIG[novoStatus]?.label || novoStatus}`,
        color: "green",
      });
      carregarServicos();
    } catch (err) {
      notifications.show({ title: "Erro ao atualizar", message: err, color: "red" });
    }
  };

  const embarcacaoOptions = embarcacoes.map((e) => ({
    value: String(e.id),
    label: `${e.nome} — ${e.identificacao}`,
  }));

  const formatarDataHora = (dataIso) => {
    if (!dataIso) return "—";
    try {
      // Converte "YYYY-MM-DD HH:MM:SS" para "YYYY-MM-DDTHH:MM:SS" para compatibilidade com Safari/Webkit
      const formattedIso = dataIso.includes(" ") ? dataIso.replace(" ", "T") : dataIso;
      const data = new Date(formattedIso);
      
      if (isNaN(data.getTime())) return dataIso;
      
      return data.toLocaleString('pt-BR', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    } catch {
      return dataIso;
    }
  };

  const podeEditar = (srv) => {
    if (isAdmin) return true;
    return srv.status !== "concluido" && srv.funcionario_id === funcionarioId;
  };

  const podeConcluir = (srv) => isAdmin && srv.status === "em_execucao";

  return (
    <>
      <Group gap="sm" mb="lg">
        <IconHistory size={28} stroke={1.5} color="var(--mantine-color-blue-6)" />
        <Title order={2}>Histórico de Serviços</Title>
      </Group>

      <Paper shadow="xs" p="md" radius="md" mb="md">
        <Select
          label="Filtrar por Embarcação"
          placeholder={isAdmin ? "Todas as embarcações" : "Suas embarcações vinculadas"}
          data={embarcacaoOptions}
          searchable
          clearable
          nothingFoundMessage="Nenhuma embarcação encontrada"
          leftSection={<IconShip size={16} />}
          value={embarcacaoSelecionada}
          onChange={setEmbarcacaoSelecionada}
        />
      </Paper>

      {carregandoServicos || loading ? (
        <Center py="xl"><Loader /></Center>
      ) : servicos.length === 0 ? (
        <Paper shadow="xs" p="xl" radius="md">
          <Center>
            <Stack align="center" gap="xs">
              <IconHistory size={48} stroke={1} color="var(--mantine-color-gray-4)" />
              <Text c="dimmed">
                {embarcacaoSelecionada
                  ? "Nenhum serviço registrado para esta embarcação"
                  : "Nenhum serviço registrado"}
              </Text>
            </Stack>
          </Center>
        </Paper>
      ) : (
        <Paper shadow="xs" radius="md" style={{ overflow: "hidden" }}>
          <Table striped highlightOnHover>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>Embarcação</Table.Th>
                <Table.Th>Funcionário</Table.Th>
                <Table.Th>Serviços Realizados</Table.Th>
                <Table.Th>Status</Table.Th>
                <Table.Th>Criado em</Table.Th>
                <Table.Th>Atualizado em</Table.Th>
                <Table.Th w={60}>Ações</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {servicos.map((srv) => {
                const statusConf = STATUS_CONFIG[srv.status] || { label: srv.status, color: "gray" };
                const editavel = podeEditar(srv);
                const concluivel = podeConcluir(srv);

                return (
                  <Table.Tr key={srv.id}>
                    <Table.Td>{srv.embarcacao_nome || "—"}</Table.Td>
                    <Table.Td>{srv.funcionario_nome || "—"}</Table.Td>
                    <Table.Td>
                      <Text size="sm" lineClamp={2}>{srv.descricao}</Text>
                      {srv.observacao && (
                        <Text size="xs" c="dimmed" lineClamp={1}>Obs: {srv.observacao}</Text>
                      )}
                    </Table.Td>
                    <Table.Td>
                      <Badge variant="light" color={statusConf.color} size="sm">{statusConf.label}</Badge>
                    </Table.Td>
                    <Table.Td>
                      <Text size="xs" c="dimmed">{formatarDataHora(srv.created_at)}</Text>
                    </Table.Td>
                    <Table.Td>
                      <Text size="xs" c="dimmed">{formatarDataHora(srv.updated_at)}</Text>
                    </Table.Td>
                    <Table.Td>
                      {(editavel || concluivel) && (
                        <Menu shadow="md" width={200}>
                          <Menu.Target>
                            <Tooltip label="Opções">
                              <ActionIcon variant="subtle" color="gray">
                                <IconDotsVertical size={16} />
                              </ActionIcon>
                            </Tooltip>
                          </Menu.Target>
                          <Menu.Dropdown>
                            {editavel && (
                              <Menu.Item leftSection={<IconEdit size={14} />} onClick={() => abrirEditar(srv)}>
                                Editar Registro
                              </Menu.Item>
                            )}
                            {concluivel && (
                              <Menu.Item
                                leftSection={<IconCheck size={14} />}
                                color="green"
                                onClick={() => atualizarStatusDireto(srv.id, "concluido")}
                              >
                                Finalizar Serviço
                              </Menu.Item>
                            )}
                          </Menu.Dropdown>
                        </Menu>
                      )}
                    </Table.Td>
                  </Table.Tr>
                );
              })}
            </Table.Tbody>
          </Table>
        </Paper>
      )}

      {/* Modal de Edição */}
      <Modal
        opened={!!editando}
        onClose={() => setEditando(null)}
        title="Editar Registro de Serviço"
        centered
      >
        <form onSubmit={editForm.onSubmit(salvarEdicao)}>
          <Stack gap="sm">
            <Checkbox.Group
              label="Serviços Realizados"
              required
              {...editForm.getInputProps("descricao")}
            >
              <SimpleGrid cols={2} spacing="xl" mt="xs">
                {servicoOptionColumns.map((column, index) => (
                  <Stack key={index} gap="xs">
                    {column.map((servico) => (
                      <Checkbox
                        key={servico}
                        value={servico}
                        label={servico}
                      />
                    ))}
                  </Stack>
                ))}
              </SimpleGrid>
            </Checkbox.Group>

            <Textarea
              label="Observações"
              minRows={3}
              {...editForm.getInputProps("observacao")}
            />
            {isAdmin && editando?.status !== "concluido" && (
              <Select
                label="Alterar Status"
                data={[
                  { value: "em_execucao", label: "Em Execução" },
                  { value: "concluido", label: "Concluído" },
                ]}
                {...editForm.getInputProps("status")}
              />
            )}
            <Group justify="flex-end" mt="md">
              <Button variant="default" onClick={() => setEditando(null)}>Cancelar</Button>
              <Button type="submit" loading={loading}>Salvar Alterações</Button>
            </Group>
          </Stack>
        </form>
      </Modal>

      {servicos.length > 0 && (
        <Text size="sm" c="dimmed" mt="sm" ta="right">
          {servicos.length} serviço{servicos.length !== 1 ? "s" : ""} encontrado{servicos.length !== 1 ? "s" : ""}
        </Text>
      )}
    </>
  );
}

export default Historico;
