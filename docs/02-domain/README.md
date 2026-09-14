# Lingo Domain Package

Status: **Canonical foundation**  
Version: 1.1

This directory contains framework-independent authoritative domain contracts for Lingo.

## Documents

- `domain-model.md` — aggregate boundaries, entities/value objects, ownership and revision model.
- `invariants.md` — stable `INV-*` canonical invariant registry.
- `intent-contract.md` — mutating intent, authorization/ownership guard and idempotency contract.
- `match-state-machine.md` — explicit Match/Stage/Question state-machine requirements for Classic Mode.
- `failure-and-replay-semantics.md` — retry, replay, reconnect, response-loss, restart and partial-failure semantics.
- `multiplayer-lifecycle.md` — room, roster, ready/start, host, disconnect/reconnect and abandonment semantics for the 2-4 player PoC.

## Authority boundaries

These documents define domain behavior while leaving unrelated unresolved decisions with their owning work:

- Final rules: #2
- Word source/licensing/dataset lifecycle: #3
- Authoritative timer/latency: **resolved by accepted ADR-004**
- Multiplayer lifecycle/reconnect: **resolved by `multiplayer-lifecycle.md` / #5**
- Persistence/event-audit: #9
- Word dataset storage/sync: #10
- Identity/authentication and session security: #11/#17
- Stage transition UX: #12
- Authoritative randomness implementation contract: #18

## Verification

Golden Game fixtures (#13) and AEGIS/MUR qualification (#21) must reference the stable invariant IDs from `invariants.md`.

Changes to canonical domain semantics require affected stage/ADR/test review before implementation changes.
