# Lingo Domain Package

Status: **Canonical foundation**  
Version: 1.4

This directory contains framework-independent authoritative domain contracts for Lingo.

## Documents

- `domain-model.md` — aggregate boundaries, entities/value objects, ownership and revision model.
- `invariants.md` — stable `INV-*` canonical invariant registry.
- `intent-contract.md` — mutating intent, authorization/ownership guard and idempotency contract.
- `match-state-machine.md` — explicit Match/Stage/Question state-machine requirements for Classic Mode.
- `failure-and-replay-semantics.md` — retry, replay, reconnect, response-loss, restart and partial-failure semantics aligned with ADR-005.
- `multiplayer-lifecycle.md` — room, roster, ready/start, host, disconnect/reconnect and abandonment semantics for the 2-4 player PoC.
- `authoritative-randomness.md` — one-draw/persist-result, replay-safe randomness and client-secrecy contract for gameplay-affecting random choices.

## Authority boundaries

These documents define domain behavior while leaving unrelated unresolved decisions with their owning work:

- Final rules: #2
- Word source/licensing/dataset lifecycle: #3
- Authoritative timer/latency: **resolved by accepted ADR-004**
- Multiplayer lifecycle/reconnect: **resolved by `multiplayer-lifecycle.md` / #5**
- Authoritative randomness/reveal-order: **resolved by `authoritative-randomness.md` / #18**
- Persistence/snapshot/event-audit and crash-window semantics: **resolved by accepted ADR-005 / #9**
- Identity credential/reconnect-token model: **resolved by accepted ADR-007 / #11**
- Competitive security/fail-closed authority: **resolved by `competitive-security-and-abuse-model.md` / #17**
- Word dataset storage/sync: #10
- Stage transition UX: #12

## Persistence boundary summary

Accepted competitive mutation semantics require:

- durable materialized authoritative state as recovery authority;
- monotonic revision/concurrency guard;
- state + required idempotency + required domain/audit evidence in one atomic commit;
- immutable score-award/entitlement integrity evidence where applicable;
- transactional outbox or equivalent for must-deliver post-commit external publication;
- stale cache/snapshot never overriding a higher durable revision.

## Identity/security boundary summary

The PoC uses guest-first server-issued identity:

- display name/device ID are not authority;
- match Player is server-bound to a PlayerSession lineage;
- reconnect requires current server-issued high-entropy proof;
- `securityVersion` controls revocation lineage;
- `connectionGeneration` provides exactly one current mutation-authoritative connection generation;
- future persistent account linking cannot rewrite active match identity.

## Verification

Golden Game fixtures (#13) and AEGIS/MUR qualification (#21) must reference the stable invariant IDs from `invariants.md` plus applicable `RAND-*`, `PERSIST-*`, `SEC-*`, and `AUTH-*` invariants.

Changes to canonical domain semantics require affected stage/ADR/test review before implementation changes.
