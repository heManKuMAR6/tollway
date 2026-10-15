# Design notes

Seams noticed while building that later milestones will need. These are notes, not
commitments; nothing here is built until its milestone. See `docs/adr/0001-narrow-scope.md`
for the scope.

## Planned milestones

- **`guard/` (v0.2):** per-run budgets and loop breaker, keyed on `x-tollway-run-id`. The run
  store starts as an in-process dict with a TTL. It moves to SQLite or Redis only if a
  two-replica setup needs it.
- **`tools/` (v0.3):** a thin MCP Streamable-HTTP proxy with an idempotency ledger for tools
  marked `side_effect: true`. Key = (run id, tool, canonical argument hash). States are
  pending → done | unknown. Never re-execute an unknown outcome.

## Deferred, maybe never

- Streaming response validation: buffer tool-call deltas while passing text through.
- JSON-schema validation of tool arguments (needs a dependency; revisit after v0.1).
- Optional scorer plug-in (EvalForge or others) to gate which backends may join a chain.

## Not building

A2A, admin UI, Grafana dashboards, virtual-key billing, full MCP aggregation and RBAC,
multi-tenancy. agentgateway, LiteLLM and Bifrost already cover these.
