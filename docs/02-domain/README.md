# Lingo Domain Package

Status: **Canonical foundation**  
Version: 1.0

This directory contains framework-independent authoritative domain contracts for Lingo.

## Documents

- `domain-model.md` — aggregate boundaries, entities/value objects, ownership and revision model.
- `invariants.md` — stable `INV-*` canonical invariant registry.
- `intent-contract.md` — mutating intent, authorization/ownership guard and idempotency contract.
- `match-state-machine.md` — explicit Match/Stage/Question state-machine requirements for Classic Mode.
- `failure-and-replay-semantics.md` — retry, replay, reconnect, response-loss, restart and partial-failure semantics.

## Authority boundaries

These documents define domain behavior but intentionally do not invent unresolved decisions owned elsewhere:

- Final rules: #2
- Word/invalid-input policy: #3
- Authoritative timer/latency: #4
- Multiplayer lifecycle/reconnect: #5
- Persistence/event-audit: #9
- Word dataset storage/sync: #10
- Identity/authentication: #11
- Stage transition UX: #12
- Authoritative randomness: #18

## Verification

Golden Game fixtures (#13) and AEGIS/MUR qualification (#21) must reference the stable invariant IDs from `invariants.md`.

Changes to canonical domain semantics require affected stage/ADR/test review before implementation changes.
