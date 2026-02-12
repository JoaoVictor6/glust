# Configuração e Execução

Este guia fornece instruções para configurar, rodar e testar o Glust em seu ambiente local.

## Pré-requisitos

Para rodar o projeto, você precisará de:
- **Rust** (stable toolchain)
- **Docker** e **Docker Compose** (para o banco de dados)

## Passos Iniciais

### 1. Configurar variáveis de ambiente
Copie o arquivo de exemplo `.env.example` para `.env`:

```bash
cp .env.example .env
```

Garanta que as configurações do banco de dados no `.env` correspondam ao seu ambiente Docker.

### 2. Iniciar dependências (Postgres)
Suba o banco de dados via Docker Compose:

```bash
docker-compose up -d
```

Isso iniciará um container PostgreSQL configurado para o Glust.

## Executando o Glust

### Via Script de Teste E2E (Recomendado)
Para uma experiência completa "out-of-the-box", utilize o script `run_e2e.sh`. Ele compila o projeto, inicia o servidor Glust, roda o cliente de teste e verifica a ingestão de logs.

```bash
./run_e2e.sh
```

Este script irá:
1.  Compilar o projeto (`cargo build`).
2.  Iniciar o servidor Glust em background.
3.  Iniciar o `test-client` para gerar carga.
4.  Enviar uma requisição de teste para o endpoint HTTP.
5.  Mostrar os logs do servidor e do cliente.
6.  Limpar os processos ao finalizar.

### Manualmente

Para rodar apenas o servidor:

```bash
cargo run --bin glust
```
O servidor estará disponível em `http://localhost:8080` (ou na porta configurada).

## Testando

### Gerar Carga com Test Client
O projeto inclui um `test-client` para simular tráfego OTLP:

```bash
cargo run -p test-client
```

### Rodar Testes Unitários
Para executar a suite de testes do Rust:

```bash
cargo test
```
