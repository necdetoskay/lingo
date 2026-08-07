# Lingo

Lingo is a mobile-first multiplayer Turkish word-game project.

> Status: **Pre-implementation / canonical design phase**

The project intentionally does **not** start with application code. Game rules, domain concepts, word validation, realtime behavior, architecture decisions, acceptance criteria, and test strategy must be documented before implementation begins.

## Current scope

The first Proof of Concept (PoC) targets **Classic Mode** for 2-4 players.

Classic Mode currently contains:

1. Stage 1 — Warm-up: 4-letter words
2. Stage 2 — Steal: 5-letter words
3. Stage 3 — Duel: 6-letter words
4. Final — **TBD**

The canonical rules for stages 1-3 are under `docs/01-game-design/classic-mode/`.

## Documentation map

- `docs/00-project/` — project charter, scope, terminology and implementation gate
- `docs/01-game-design/` — canonical game rules and game-state behavior
- `docs/02-domain/` — domain model and invariants
- `docs/03-word-platform/` — dictionary, answer-pool and word-intelligence design
- `docs/04-architecture/` — application, realtime and persistence architecture
- `docs/05-ai-ml/` — on-device AI and model-training experiments
- `docs/06-testing/` — rule, state-machine, multiplayer and data-quality tests
- `docs/07-delivery/` — implementation readiness and sprint planning
- `docs/adr/` — Architecture Decision Records

## Canonical rule

When implementation and documentation disagree, the **latest accepted canonical document wins**. A rule change must update its canonical document and related acceptance tests before code is changed.

## Implementation gate

No gameplay feature is implementation-ready until its rules, state transitions, timing behavior, edge cases, data requirements, acceptance criteria, and test scenarios are documented.
