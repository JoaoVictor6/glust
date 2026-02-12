# Glust

**Glust** é um ingestor de logs e traces baseado em **OpenTelemetry**, desenvolvido em **Rust**.
O projeto tem como objetivo principal o **estudo de sistemas de alta performance, concorrência, observabilidade e engenharia de resiliência**, utilizando um domínio propositalmente simples para focar nos desafios reais de infraestrutura.

O domínio do projeto é mínimo por design: o foco não está em regras de negócio, mas sim em **resistência do sistema**, incluindo:

* alta concorrência
* ingestão em alto volume
* controle de latência
* tolerância a falhas
* arquitetura escalável
* engenharia de dados

> A complexidade do Glust não está no domínio, mas na resistência do sistema.

## Documentação

A documentação completa do projeto encontra-se na pasta [`docs/`](docs/):

- [Introdução e Propósito](docs/intro.md)
- [Configuração e Execução](docs/setup.md)
- [Arquitetura e Design](docs/architecture.md)

---

## Objetivos

O pipeline do Glust é intencionalmente simples:

1. **Serializar / higienizar** pacotes OpenTelemetry (OTLP / Protobuf)
2. **Persistir** os dados de observabilidade
3. **Expor / apresentar** os dados ingeridos

Pipeline conceitual:

```
OTLP → Adapter → Domain → Storage → Query
```

---

## Escopo inicial

O projeto começa com um escopo reduzido para garantir solidez estrutural:

* Ingestão de traces/logs OTLP
* Validação e normalização de dados
* Persistência simples em banco de dados
* Leitura básica para visualização
* Testes de carga e estresse

---

## Plano geral

* [ ] Serialização e parsing dos protobufs OTLP
* [ ] Validação e higienização dos dados
* [ ] Persistência em banco de dados
* [ ] Sharding e estratégia de particionamento
* [ ] Pipeline assíncrono de ingestão
* [ ] Testes de estresse e carga
* [ ] Métricas internas (latência, p95, p99, throughput)
* [ ] Apresentação e consulta dos dados
* [ ] Resiliência (retry, backpressure, limites, filas)

---

## Princípios de arquitetura

O Glust segue princípios de engenharia de sistemas:

* **Hot path não bloqueante**
* **Backpressure explícito**
* **Falha rápida (fail fast)**
* **Domínio mínimo**
* **Fluxo de dados unidirecional**
* **Concorrência controlada**
* **Separação clara entre ingestão, domínio e storage**
* **Escalabilidade horizontal**
* **Resiliência acima de abstração**

---

## Motivação

Este projeto existe para estudar, na prática:

* Rust em sistemas concorrentes
* engenharia de ingestão de dados
* sharding e resharding
* ingestão distribuída
* arquitetura de observabilidade
* OpenTelemetry e OTLP
* design de ingestors
* sistemas de alta disponibilidade
* testes de carga
* métricas de latência (p95, p99)
* consistência eventual
* pipelines de dados

---

## Nome do projeto

**Glust** = *gluttony* + *rust*
Um sistema projetado para “consumir” grandes volumes de dados de observabilidade com eficiência e resistência.

