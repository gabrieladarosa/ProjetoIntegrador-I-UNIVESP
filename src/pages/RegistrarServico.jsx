import { useState, useEffect } from "react";
import {
  Title,
  Button,
  Select,
  Checkbox,
  Textarea,
  Group,
  Stack,
  Paper,
  Text,
  Alert,
  SimpleGrid,
} from "@mantine/core";
import { DateInput } from "@mantine/dates";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconTool,
  IconCheck,
  IconAlertCircle,
} from "@tabler/icons-react";
import { useTauriCommand } from "../hooks/useTauriCommand";
import { useAuth } from "../contexts/AuthContext";
import "dayjs/locale/pt-br";

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

function RegistrarServico() {
  const [embarcacoes, setEmbarcacoes] = useState([]);
  const [funcionarios, setFuncionarios] = useState([]);
  const [sucesso, setSucesso] = useState(false);
  const { execute, loading } = useTauriCommand();
  const { isAdmin, funcionarioId } = useAuth();

  const form = useForm({
    initialValues: {
      embarcacao_id: null,
      funcionario_id: isAdmin ? null : String(funcionarioId),
      descricao: [],
      data_execucao: new Date(),
      observacao: "",
    },
    validate: {
      embarcacao_id: (v) => (v === null ? "Selecione uma embarcação" : null),
      funcionario_id: (v) => (v === null ? "Selecione um funcionário" : null),
      descricao: (v) => (v.length === 0 ? "Selecione ao menos um serviço" : null),
      data_execucao: (v) => (v === null ? "Data é obrigatória" : null),
    },
  });

  useEffect(() => {
    const carregarDados = async () => {
      try {
        const [embs, funcs] = await Promise.all([
          execute("listar_embarcacoes"),
          execute("listar_funcionarios_ativos"),
        ]);
        setEmbarcacoes(embs);
        setFuncionarios(funcs);
        
        // Se for funcionário, o id já está no initialValues, mas se carregar depois:
        if (!isAdmin && funcionarioId) {
          form.setFieldValue("funcionario_id", String(funcionarioId));
        }
      } catch (err) {
        notifications.show({
          title: "Erro ao carregar dados",
          message: err,
          color: "red",
        });
      }
    };
    carregarDados();
  }, [execute, isAdmin, funcionarioId]);

  const salvar = async (values) => {
    try {
      const dataStr = values.data_execucao
        ? values.data_execucao.toISOString().split("T")[0]
        : "";

      await execute("criar_servico", {
        data: {
          embarcacao_id: Number(values.embarcacao_id),
          funcionario_id: Number(values.funcionario_id),
          descricao: values.descricao.join(", "),
          data_execucao: dataStr,
          observacao: values.observacao || null,
        },
      });

      notifications.show({
        title: "Serviço registrado",
        message: "O serviço foi registrado e está 'Em Execução'",
        color: "green",
        icon: <IconCheck size={16} />,
      });

      form.reset();
      // Re-setar o funcionário id se não for admin
      if (!isAdmin && funcionarioId) {
        form.setFieldValue("funcionario_id", String(funcionarioId));
      }
      
      setSucesso(true);
      setTimeout(() => setSucesso(false), 5000);
    } catch (err) {
      notifications.show({
        title: "Erro ao registrar",
        message: err,
        color: "red",
      });
    }
  };

  const embarcacaoOptions = embarcacoes.map((e) => ({
    value: String(e.id),
    label: `${e.nome} — ${e.identificacao}`,
  }));

  const funcionarioOptions = funcionarios.map((f) => ({
    value: String(f.id),
    label: `${f.nome}${f.cargo ? ` (${f.cargo})` : ""}`,
  }));

  return (
    <>
      <Group gap="sm" mb="lg">
        <IconTool size={28} stroke={1.5} color="var(--mantine-color-blue-6)" />
        <Title order={2}>Registrar Serviço</Title>
      </Group>

      {sucesso && (
        <Alert
          icon={<IconCheck size={16} />}
          title="Serviço registrado com sucesso!"
          color="green"
          mb="md"
          withCloseButton
          onClose={() => setSucesso(false)}
        >
          O serviço foi iniciado e já está disponível no histórico.
        </Alert>
      )}

      {embarcacoes.length === 0 || (isAdmin && funcionarios.length === 0) ? (
        <Alert
          icon={<IconAlertCircle size={16} />}
          title="Cadastros necessários"
          color="yellow"
        >
          <Text size="sm">
            {isAdmin 
              ? "Para registrar um serviço, é necessário ter pelo menos uma embarcação e um funcionário ativo cadastrados."
              : "Você não possui embarcações vinculadas ou não há funcionários ativos."}
          </Text>
        </Alert>
      ) : (
        <Paper shadow="xs" p="xl" radius="md" maw={700}>
          <form onSubmit={form.onSubmit(salvar)}>
            <Stack gap="md">
              <Select
                label="Embarcação"
                placeholder="Selecione a embarcação"
                data={embarcacaoOptions}
                searchable
                required
                nothingFoundMessage="Nenhuma embarcação vinculada encontrada"
                {...form.getInputProps("embarcacao_id")}
              />

              <Select
                label="Funcionário Responsável"
                placeholder="Selecione o funcionário"
                data={funcionarioOptions}
                searchable
                required
                disabled={!isAdmin}
                nothingFoundMessage="Nenhum funcionário ativo encontrado"
                {...form.getInputProps("funcionario_id")}
              />

              <DateInput
                label="Data de Execução"
                placeholder="Selecione a data"
                required
                disabled={!isAdmin}
                locale="pt-br"
                valueFormat="DD/MM/YYYY"
                {...form.getInputProps("data_execucao")}
              />

              <Checkbox.Group
                label="Serviços Realizados"
                required
                {...form.getInputProps("descricao")}
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
                placeholder="Observações adicionais (opcional)"
                minRows={2}
                autosize
                {...form.getInputProps("observacao")}
              />

              <Group justify="flex-end" mt="md">
                <Button
                  variant="default"
                  onClick={() => {
                    form.reset();
                    if (!isAdmin && funcionarioId) {
                      form.setFieldValue("funcionario_id", String(funcionarioId));
                    }
                  }}
                >
                  Limpar
                </Button>
                <Button type="submit" loading={loading}>
                  Registrar e Iniciar Serviço
                </Button>
              </Group>
            </Stack>
          </form>
        </Paper>
      )}
    </>
  );
}

export default RegistrarServico;
