# Gemini Agent Context — Glust

## 1. Visão Geral

Glust é um **ingestor simples de logs OpenTelemetry (OTLP)** escrito em **Rust**.

O projeto tem **finalidade educacional** e existe para estudar:
- ingestão de dados binários (Protobuf)
- hotpath de escrita
- storage incremental
- evolução arquitetural guiada por dor real

O sistema **NÃO** pretende ser um OpenTelemetry Collector completo.

> Princípio central: **começar simples e evoluir conscientemente**  
> Princípio central: **começar simples e evoluir conscientemente**  
> (“ver uma carroça virar um carro”)

## 1.2 Histórico do Projeto
- O agente MUST consultar `git log` para entender o contexto de testes de estresse e decisões passadas.

## 1.1 Objetivo de Aprendizagem (Rust)

Este projeto também tem como objetivo explícito o **aprendizado da linguagem Rust**.

### Diretrizes de Aprendizado

- O agente MUST assumir que o autor está **aprendendo Rust**
- O agente SHOULD privilegiar soluções **idiomáticas, porém simples**
- O agente MUST evitar código excessivamente avançado sem explicação
- O agente SHOULD explicar conceitos relevantes quando introduzidos:
  - ownership
  - borrowing
  - lifetimes (quando inevitáveis)
  - `Arc`, `Mutex`, `RwLock`
  - `dyn Trait`
  - `Send` e `Sync`

### Complexidade

- O agente MUST preferir código claro a código “esperto”
- O agente MUST evitar:
  - lifetime annotations complexas sem necessidade
  - macros avançadas no MVP
  - abstrações genéricas profundas
- O agente MAY sugerir alternativas mais avançadas, **desde que explique o porquê**

### Estilo de Código

- O código gerado SHOULD ser:
  - explícito
  - legível
  - fácil de debugar
- O agente SHOULD adicionar comentários quando o código envolver:
  - regras do borrow checker
  - motivos para uso de `Arc` ou `dyn`
  - restrições de thread-safety

### Evolução

- O agente MUST aceitar soluções "simples mas corretas"
- Refactors e melhorias SHOULD ser sugeridos apenas após o código funcionar
- O aprendizado incremental é mais importante que a perfeição técnica

> Aprender Rust corretamente é um objetivo tão importante quanto o funcionamento do sistema.


---

## 2. Escopo Atual (MVP)

### 2.1 O sistema MUST
- Receber logs OTLP via HTTP (`POST /v1/logs`)
- Aceitar payload `application/x-protobuf`
- Decodificar `ExportLogsServiceRequest`
- Persistir logs em PostgreSQL
- Permitir leitura simples dos logs

### 2.2 O sistema MUST NOT
- Implementar traces ou spans
- Implementar sharding
- Implementar particionamento
- Implementar retenção automática
- Implementar batching complexo
- Implementar alta disponibilidade

---

## 3. Stack Técnica

### 3.1 Runtime
- O projeto MUST usar **Tokio** como runtime async

### 3.2 HTTP
- O projeto MUST usar **Axum**
- O handler MUST aceitar corpo binário (`Bytes`)
- O hotpath MUST NOT usar JSON

### 3.3 Protobuf / OTLP
- O projeto MUST usar:
  - `opentelemetry-proto`
  - `prost`
- O sistema MUST decodificar diretamente:
  - `ExportLogsServiceRequest`
- O sistema MAY ignorar campos não essenciais no MVP

### 3.4 Storage
- O projeto MUST usar **PostgreSQL**
- O projeto MUST usar **SQLx**
- O projeto MUST usar o pool nativo do SQLx
- O hotpath MUST realizar apenas `INSERT`

### 3.5 Migrations
- O projeto MUST usar migrations do SQLx
- Toda migration MUST ter pares `.up.sql` e `.down.sql`
- Migrations MUST ser reversíveis

### 3.6 Commits
- O projeto MUST seguir [Conventional Commits](https://www.conventionalcommits.org/)
- Tipos permitidos: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`
- Escopos sugeridos: `ingest`, `storage`, `api`, `domain`, `migration`

### 3.7 Ferramentas
- O agente MUST utilizar o `gh` CLI para consultar contexto do repositório
- O agente MAY listar issues, PRs e releases para entender decisões passadas
- O agente MUST assumir que o `gh` já está autenticado e configurado

### 3.5 Observabilidade interna
- O projeto SHOULD usar `tracing`
- Logs internos SHOULD ser estruturados

---

## 4. Modelo de Dados

### 4.1 Log (mínimo)

Cada log MUST conter:
- `timestamp: TIMESTAMPTZ` (era time_unix_nano)
- `trace_id: TEXT` (Hex string)
- `span_id: TEXT` (Hex string)
- `service_name: TEXT` (extraído de ResourceAttributes)
- `body: JSONB`

### 4.2 Tempo
- O campo `timestamp` MUST ser armazenado como `TIMESTAMPTZ`
- Conversão para formato humano ocorre na inserção

---

## 5. Decisões Arquiteturais

### 5.1 Imutabilidade
- Logs MUST ser tratados como imutáveis
- O sistema MUST seguir mentalidade append-only
- O sistema MUST NOT atualizar ou deletar logs individualmente

### 5.2 Simplicidade
- O código MUST priorizar clareza
- O agente MUST evitar abstrações desnecessárias
- Traits genéricas MUST ser usadas apenas quando houver múltiplas implementações reais

---

## 6. Organização de Código

A estrutura esperada do projeto é:

```text
src/
 ├── main.rs
 ├── ingest/
 │    └── http.rs
 ├── adapter/
 │    └── otlp.rs
 ├── domain/
 │    └── log.rs
 └── storage/
      └── repository.rs
````

Regras:

* Todo arquivo `.rs` MUST pertencer a um módulo
* Módulos MUST ser declarados explicitamente
* Arquivos órfãos MUST NOT existir

---

## 7. Diretrizes para o Agente

### 7.1 Ao gerar código

* O agente MUST priorizar legibilidade
* O agente MUST escrever código explícito
* O agente SHOULD adicionar comentários curtos quando necessário
* O agente MUST evitar macros complexas no MVP

### 7.2 Concorrência

* `Arc` MUST ser usado apenas quando necessário
* `dyn Trait` MUST ser usado apenas para abstrações reais
* Tipos compartilhados entre threads MUST ser `Send + Sync`

### 7.3 Otimizações

* O agente MUST NOT propor otimizações prematuras
* O agente SHOULD explicar trade-offs quando sugerir mudanças
* Qualquer otimização MUST ser justificada por dor real

---

## 8. Fora de Escopo (não sugerir automaticamente)

O agente MUST NOT sugerir automaticamente:

* Sharding
* Particionamento
* Cache
* Batch processing
* gRPC
* CQRS
* Event sourcing
* Traces completos

Esses temas ONLY entram quando explicitamente solicitados.

---

## 9. Objetivo Final

O objetivo do Glust é aprendizado profundo e incremental sobre:

* OpenTelemetry
* Protobuf
* Rust async
* Ingestão de logs
* Evolução arquitetural consciente

Este projeto é um **laboratório**, não um produto comercial.

