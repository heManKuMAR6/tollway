# ADR-0001: Narrow scope to three production pains

- Status: accepted
- Date: 2026-10-15

## Context

The first plan was a six-milestone "agent gateway for everything": LLM routing, virtual keys
and budgets, MCP gateway, observability, eval-gated routing and A2A. A survey of the field in
October 2026 found that nearly all of it already exists in mature open-source projects:
LiteLLM, Bifrost, agentgateway (Linux Foundation), Portkey, Kong and Envoy AI Gateway. One
awesome-list tracks more than 160 gateways. Building the same features later and in Python
would not help anyone.

The author has personally hit three problems running agents: provider outages and 429s,
runaway cost from looping agents, and tool calls repeated by retries. Independent data backs
these: rate-limit errors are the most common LLM call failure in Datadog's telemetry, LiteLLM
added per-session caps because of runaway loops, and duplicate side effects on retry show up
in several framework bug reports.

## Decision

tollway does three things, one per milestone:

1. v0.1: keep an agent running when a provider fails (failover that's safe for agents).
2. v0.2: stop an agent when it loops or overspends (per-run guard).
3. v0.3: stop a retry from repeating an action (idempotency ledger for MCP tool calls).

It stays small: few dependencies, no database, readable in an afternoon.

Dropped: A2A, admin UI, dashboards, virtual-key billing, full MCP aggregation and RBAC,
multi-tenancy. EvalForge becomes an optional scorer plug-in.

## Consequences

- Smaller surface to build, test and secure.
- Fewer features than LiteLLM or agentgateway. The README says so and points users to them
  when they need more.
- Each milestone must prove its pain is solved with a chaos demo. If a milestone doesn't get
  traction, later ones can be dropped.
