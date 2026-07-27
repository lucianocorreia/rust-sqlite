# dblite

`dblite` e um banco de dados em memoria escrito em Rust, com foco didatico.
O projeto separa bem cada responsabilidade: parser, executor e armazenamento.

## Objetivo

Construir um mini banco relacional in-memory, evoluindo de um nucleo simples para uma camada de comandos mais rica.

## Arquitetura

- `src/value.rs`: tipo de celula (`Value::Integer`, `Value::Text`)
- `src/table.rs`: `Column`, `Row`, `Table` e validacao de insercao
- `src/database.rs`: catalogo de tabelas com `HashMap`
- `src/command.rs`: comandos de alto nivel (`Create`, `Insert`, `Select`)
- `src/parser.rs`: converte texto em `Command`
- `src/executor.rs`: executa `Command` no `Database`
- `src/main.rs`: REPL (loop interativo no terminal)

## Evolucao em 3 etapas

### Etapa 1 - Fundacao do banco in-memory

- modelagem de valores, colunas, linhas e tabelas
- validacao de quantidade de valores por linha
- catalogo de tabelas com prevencao de nomes duplicados

### Etapa 2 - Linguagem de comandos

- parser de entrada textual para comandos de dominio
- tratamento de erros de sintaxe e argumentos
- executor desacoplado do parser

### Etapa 3 - Evolucao avancada

- ampliar sintaxe para consultas e filtros
- melhorar formato de saida e experiencia do REPL
- adicionar mais regras de schema e validacoes de dominio

> Estado atual: etapas 1 e 2 implementadas no codigo, com REPL funcional.

## Como rodar

```bash
cargo run
```

No prompt `dblite>`, use:

```text
create users id name age
insert users 1 Ana 30
insert users 2 Bob 28
select users
.exit
```

## Como testar

Rodar todos os testes:

```bash
cargo test
```

Rodar testes por modulo:

```bash
cargo test value
cargo test table
cargo test database
cargo test parser
cargo test executor
```

## Exemplo rapido (nao interativo)

```bash
cat <<'EOF' | cargo run
create users id name age
insert users 1 Ana 30
insert users 2 Bob 28
select users
.exit
EOF
```

## Requisitos

- Rust toolchain recente (cargo + rustc)
- sem dependencias externas no `Cargo.toml`

## Licenca

Defina a licenca que voce preferir para o projeto (MIT, Apache-2.0, etc.).
