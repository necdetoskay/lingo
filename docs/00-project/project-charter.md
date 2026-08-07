# Project Charter

Status: Canonical

## Product intent

Build a mobile-first Turkish word-game experience that is enjoyable for families and competitive multiplayer play. The first milestone is a working Classic Mode PoC. Other game modes are intentionally deferred until the Classic Mode has been implemented, played, measured, and evaluated.

## PoC scope

- 2-4 players
- Classic Mode only
- Stage 1, Stage 2, Stage 3 and a later-defined Final
- Shared word-validation engine
- Deterministic scoring
- Realtime multiplayer behavior where required
- Local/offline-friendly word lookup where possible
- Foundation for future speech input

## Non-goals for the first implementation

- Designing all future game modes
- Large-scale production matchmaking
- Monetization
- Production-grade AI content generation
- Training a custom model before a gold dataset exists

## Canonical design principles

1. Rules before code.
2. Server-authoritative multiplayer state for competitive play.
3. Deterministic game outcomes: identical inputs and state must yield identical results.
4. Word validity and answer eligibility are separate concepts.
5. Timers must have explicit start, stop, timeout, and ownership semantics.
6. A player must always be able to understand why a guess was accepted, rejected, scored, or transferred.
7. Undecided behavior is documented as `TBD`; implementation must not silently invent rules.

## Implementation gate

A gameplay feature cannot enter implementation until all of the following exist:

- Canonical rule definition
- State-transition definition
- Timer semantics
- Scoring semantics
- Invalid-input behavior
- Multiplayer ownership behavior
- Edge-case list
- Acceptance criteria
- Automated-test scenarios

## Change control

Once a stage is marked `LOCKED`, changes require:

1. A version increment to the canonical rules document.
2. A short rationale.
3. Updated acceptance tests.
4. Review of effects on scoring and later stages.
