# CLAUDE.md — tollway

Instructions for Claude Code (and humans) working in this repo. Follow them exactly. If
something here conflicts with what works in practice, stop and ask; don't silently deviate.

## What tollway is

A small, self-hosted gateway that sits between AI agents and the services they call. It does
three things:

1. **Keeps an agent running when a provider fails** (v0.1): model aliases, ordered failover
   chains, circuit breaker, retry budget, streaming-safe failover, capability-aware chains,
   and response validation.
2. **Stops an agent when it loops or overspends** (v0.2): per-run budgets and a
   repetition-based loop breaker.
3. **Stops a retry from repeating an action** (v0.3): an idempotency ledger for side-effecting
   MCP tool calls.

A "run" is one agent task, identified by the `x-tollway-run-id` request header.

**Not building:** A2A, admin UI, dashboards, virtual-key billing, full MCP aggregation or RBAC,
multi-tenancy. If you see a seam we'll need later, write it in `docs/design-notes.md`; don't
build it. EvalForge (or any scorer) is an optional plug-in, not core.

Design principles: a single static binary, few dependencies, no database, stateless request
path except the in-process run store, small enough to read in an afternoon.

## Stack

Rust (edition 2024, stable; MSRV in `Cargo.toml`) · tokio · axum · reqwest (rustls) ·
serde + serde_json + a maintained YAML crate · tracing (JSON) · clap · `cargo test` +
wiremock · rustfmt · clippy (`-D warnings`) · cargo-deny · cargo-llvm-cov · k6 for
benchmarks. See `docs/adr/0003-rust.md`.

`unsafe` is forbidden. Do not add crates beyond these without asking; every new crate must
pass cargo-deny.

## Layout

```
src/
  main.rs (thin binary)  lib.rs  app.rs  config.rs  cli.rs  error.rs
  llm/  mod.rs  router.rs  breaker.rs  retry_budget.rs  streaming.rs  capabilities.rs  validators.rs
  llm/providers/  mod.rs (Provider trait)  openai_compat.rs  mock.rs
  guard/   (v0.2)    tools/  (v0.3)
tests/   integration tests (drive the axum app in-process)
examples/langgraph-agent/   deploy/   bench/   chaos/   docs/adr/   docs/design-notes.md
```

## LLM routing rules (v0.1)

- **API:** `POST /v1/chat/completions` (streaming and not), `GET /v1/models` (aliases only),
  `/healthz`, `/readyz` (false until config is valid).
- **Aliases:** the client sends `model: <alias>`; config maps it to an ordered list of
  backends. Each backend has url, model, api_key (`${ENV}` only, never literal secrets),
  timeout, first_token_timeout, max_context, and capabilities (`tools`, `json_mode`,
  `vision`).
- **Planner:** skip, without trying, any backend that lacks a capability the request needs or
  whose `max_context` is too small, or whose breaker is open.
- **Retryable:** 429, 500, 502, 503, 504, connect errors, read timeouts. `Retry-After` on a 429
  marks that backend to skip until then.
- **Not retryable:** every other 4xx is returned to the client as-is.
- **Response validation (non-streaming):** a tool call naming a tool not in the request, tool
  arguments that are not valid JSON, or a `json_object`/`json_schema` response that doesn't
  parse is treated as a retryable failure and fails over.
- **Streaming:** pass SSE through chunk by chunk with no buffering. Fail over only before the
  first chunk reaches the client. Missing `first_token_timeout` counts as a failure. An
  upstream error after the first chunk ends the stream with an SSE error event.
- **Circuit breaker, per backend:** closed → open after N failures within window W → half-open
  after cooldown C → one probe; success closes it, failure reopens it.
- **Retry budget:** a token bucket that caps retries at a share of recent requests (default
  20%). When it's exhausted, fail fast.
- **All backends failed:** 503 with a JSON list of attempts (backend, status, error).
- **Headers on every response:** `x-tollway-backend`, `x-tollway-attempts`,
  `x-tollway-request-id`, and `x-tollway-failover-reason` when a failover happened.

## Security and data

- Provider keys come only from env vars. Never log keys, prompt bodies or response bodies.
- No persistence of request or response bodies.

## Workflow

- Work in phases. Stop after each phase, show test results, and wait for "go".
- Before every commit run the full check:
  `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
- Small conventional commits, one per logical change (`feat:`, `fix:`, `docs:`, `chore:`,
  `test:`).
- Every rule above has a named test. Tests use the mock provider and wiremock; no real
  network calls.
- Record significant decisions as ADRs in `docs/adr/`.

## Definition of done for a milestone

1. CI green: fmt, clippy, tests on stable and MSRV, cargo-deny, line coverage ≥ 85%.
2. `docker compose up` works from a clean clone.
3. The chaos script proves the milestone's headline behavior.
4. Benchmark results committed in `bench/results/`.
5. README, CHANGELOG and tag updated; README marks unbuilt features as planned.
