# ADR-0002: Treat invalid responses as a failover trigger

- Status: accepted
- Date: 2026-10-15

## Context

Gateways fail over on HTTP status only. During an outage the fallback model can return a
200 that the agent cannot use: a call to a tool that wasn't offered, tool arguments that
aren't valid JSON, or a JSON-mode answer that doesn't parse. The gateway logs success and the
agent crashes or misbehaves. A fallback may also lack a capability the request needs, such as
tool calling or enough context.

## Decision

- Each backend declares its capabilities (`tools`, `json_mode`, `vision`) and `max_context`
  in config. The planner skips, without trying, any backend that can't serve the request.
- For non-streaming responses, tollway checks the response before returning it. A tool call
  naming a tool not in the request, arguments that aren't valid JSON, or a JSON-mode response
  that doesn't parse counts as a retryable failure. tollway moves to the next backend and sets
  `x-tollway-failover-reason`.
- Validation stays structural (names and JSON syntax). Full JSON-schema validation of tool
  arguments is deferred until it can be done without new dependencies.
- Streaming responses are not validated in v0.1. Text is passed through immediately, and the
  "failover only before the first token" rule (no splicing two models' output) takes
  precedence.

## Consequences

- An agent sees fewer broken answers during failover.
- Adds a small parse cost to non-streaming responses. It is measured separately in the
  benchmark.
- A model that returns invalid output consistently will trip its breaker, which is the
  intended behavior.
