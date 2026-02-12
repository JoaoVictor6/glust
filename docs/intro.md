# Introdução ao Glust

**Glust** é um ingestor de logs e traces de alta performance, desenvolvido em **Rust**, projetado para processar grandes volumes de dados de observabilidade via protocolo **OTLP (OpenTelemetry)**.

## Propósito

O objetivo do Glust não é ser um substituto completo para soluções como Prometheus ou Jaeger, mas sim um **laboratório de engenharia de sistemas**. Ele foca deliberadamente em desafios de infraestrutura e performance, mantendo o domínio de negócio propositalmente simples.

Os principais vetores de estudo são:
- **Concorrência e Paralelismo**: Uso eficiente de threads e async runtime (Tokio).
- **Engenharia de Resiliência**: Implementação de *backpressure*, *rate limiting* e *fail fast*.
- **Alta Performance**: Minimização de alocações no *hot path* e uso eficiente de I/O.
- **Arquitetura Hexagonal/Limpa**: Separação clara entre camadas de ingestão, domínio e persistência.

## O Que Ele Faz?

O Glust atua como um coletor que:
1.  Recebe dados via gRPC/HTTP (formato OTLP).
2.  Valida e normaliza esses dados.
3.  Persiste em um armazenamento eficiente.
4.  Permite consultas simples sobre os dados ingeridos.

## Diferenciais Técnicos

- **Hot Path Não Bloqueante**: A ingestão de dados nunca deve ser bloqueada por operações de I/O lentas.
- **Backpressure Explícito**: O sistema rejeita novas requisições quando está sobrecarregado, protegendo sua integridade.
- **Fail Fast**: Erros são detectados e tratados o mais cedo possível.
