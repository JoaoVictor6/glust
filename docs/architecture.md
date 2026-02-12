# Arquitetura do Glust

A arquitetura do Glust é projetada em camadas (Layered Architecture), visando desacoplamento e clareza de responsabilidades.

O fluxo de dados segue o princípio unidirecional:
`Ingest → Domain → Storage`

## Camadas

### 1. Ingest (Ingestão)
**Responsabilidade**: Receber dados externos, validar o protocolo e converter para o modelo de domínio.
- **Protocolos suportados**: OTLP (OpenTelemetry Protocol) via HTTP/gRPC.
- **Componentes**: Adapters HTTP (Axum), gRPC (Tonic).
- **Características**: Deve ser extremamente rápida e não bloqueante. Validações leves ocorrem aqui.

### 2. Domain (Domínio)
**Responsabilidade**: Núcleo da lógica de negócio e definições de tipos.
- **Modelos**: `Trace`, `Span`, `Event`, `LogRecord`.
- **Lógica**: Regras de higienização, normalização e agregação.
- **Independência**: Esta camada não depende de frameworks web nem de banco de dados específicos.

### 3. Storage (Armazenamento)
**Responsabilidade**: Persistência eficiente dos dados processados.
- **Tecnologias**: PostgreSQL (inicialmente), com abstrações via Repositories.
- **Padrões**: Repository Pattern para isolar a infraestrutura de dados.
- **Estratégia**: Uso de *batch inserts* e conexões pooladas (SQLx) para performance.

### 4. Database Schema
O esquema é otimizado para gravação rápida e consultas por serviço/tempo.

**Tabela `logs`**:
- `timestamp` (TIMESTAMPTZ): Data/hora do log.
- `trace_id` (BYTEA): ID do trace para correlação.
- `span_id` (BYTEA): ID do span.
- `service_name` (TEXT): Nome do serviço (extraído de attributes).
- `severity_text` (TEXT) / `severity_number` (INT): Nível de log.
- `body` (JSONB): Conteúdo do log.
- `resource_attributes` / `scope_attributes` / `log_attributes` (JSONB): Metadados OTLP.

**Índices**:
- `idx_logs_service_time`: `(service_name, timestamp DESC)` - Principal caso de uso (Dashboard).
- `idx_logs_trace`: `(trace_id)` - Busca por trace.
- `idx_logs_timestamp`: `(timestamp DESC)` - Cauda global.

## Design Decisions

- **Async First**: Todo o I/O é assíncrono (Tokio) para suportar milhares de conexões simultâneas.
- **Zero-Copy Parsing**: Sempre que possível, evita-se cópia desnecessária de dados durante o parsing dos Protobufs.
- **Observabilidade Interna**: O próprio Glust utiliza instrumentação (Tracing) para monitorar sua performance.
