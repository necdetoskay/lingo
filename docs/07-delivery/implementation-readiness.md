# Implementation Readiness

Status: Canonical gate

Implementation must not begin until the following PoC decisions are closed or deliberately scoped out.

This gate is complemented by `docs/07-delivery/aegis-mur-hardening-plan.md` and GitHub master backlog #15.

## Game design

- [x] Stage 1 canonical rules documented
- [x] Stage 2 canonical rules documented
- [x] Stage 3 canonical rules documented
- [ ] `LOCKED` status consistency repaired; no implementation-affecting unresolved dependency/TBD remains (#26)
- [ ] Final-stage canonical rules
- [ ] Match winner/tie-break rules
- [ ] Stage transition UX
- [ ] Exact invalid-input policy for standard questions
- [ ] Exact 10-second bonus deadline boundary
- [ ] Whether Stage 2 bonus value continues decaying during steal chain

## Domain / integrity

- [ ] Canonical `docs/02-domain/` exists (#16)
- [ ] Domain model / aggregate boundaries defined
- [ ] Stable invariant registry (`INV-*`) defined
- [ ] Intent guard / authorization-independent ownership semantics defined
- [ ] Replay/idempotency semantics defined
- [ ] Failure/replay semantics defined
- [ ] Authoritative randomness/reveal-order contract accepted (#18)
- [ ] Canonical dependency/status integrity rules defined (#19)

## Word platform

- [x] Guess Dictionary vs Answer Pool separation
- [x] Turkish normalization principle
- [x] Duplicate-letter evaluation principle
- [ ] Source/licensing decision for initial dictionary
- [ ] Canonical invalid-word policy
- [ ] Initial dataset schema finalized
- [ ] Dataset build/version process
- [ ] Competitive effective word-policy version pinning finalized

## Multiplayer

- [x] Server-authoritative principle
- [x] Claim-window / answer-window separation
- [ ] Network latency/grace policy
- [ ] Disconnect/reconnect rules per state
- [ ] Room lifecycle
- [ ] Player-ready/start rules
- [ ] Host privileges, if any
- [ ] Duplicate connection/session takeover behavior
- [ ] Stale/replayed session behavior
- [ ] Competitive abuse/threat model defined (#17)
- [ ] Realtime performance/fairness budgets defined (#20)

## Architecture

- [ ] Mobile technology ADR
- [ ] Backend/runtime ADR
- [ ] Realtime transport ADR
- [ ] ADR-004 authoritative timer/latency accepted
- [ ] ADR-005 persistence/event-audit accepted
- [ ] ADR-006 local word-dataset storage accepted
- [ ] ADR-007 authentication/identity accepted
- [ ] Experience Event System ADR-008 dependencies are satisfied and its final status is consistent

## Reliability / observability / privacy

- [ ] Mutation + idempotency + durable audit/event crash-window semantics defined (#9)
- [ ] Partial-failure/restart recovery semantics defined (#9)
- [ ] Structured observability / correlation contract defined (#22)
- [ ] Privacy/redaction/minimum-data rules defined (#22)
- [ ] Audit trail can explain score, ownership, timeout and validation decisions (#22)

## Testing

- [x] Test strategy defined
- [ ] Canonical Stage 1 golden scenarios
- [ ] Canonical Stage 2 golden scenarios
- [ ] Canonical Stage 3 golden scenarios
- [ ] Final golden scenarios after Final is defined
- [ ] 2-player full-match fixture
- [ ] 3-player full-match fixture
- [ ] 4-player full-match fixture
- [ ] Canonical `INV-*` -> fixture/test mapping defined (#13/#16)
- [ ] Fixed-seed randomness fixtures defined (#13/#18)
- [ ] Before/exact/after deadline vectors defined (#4/#13)
- [ ] AEGIS/MUR adversarial qualification matrix defined (#21)

## Delivery / supply chain

- [ ] Canonical manifest/document-integrity gate defined (#19)
- [ ] Self-hosted exact-SHA qualification policy defined (#23)
- [ ] Dependency lock/pinning and update policy defined (#23)
- [ ] Secret scanning / SCA / SBOM expectations defined for the chosen stack (#23)
- [ ] Qualification evidence format records exact SHA and relevant canonical/data versions (#21/#23)

## AI/ML

AI/ML experiments do not block the initial game-engine PoC unless explicitly moved into sprint scope.

- [x] On-device AI roadmap documented
- [ ] Gold dataset definition
- [ ] Gold policy enforces `MODEL_HIGH_CONFIDENCE != GOLD`
- [ ] Human verification or explicitly accepted deterministic truth required for gold labels
- [ ] Baseline model benchmark protocol
- [ ] Specialized classifier experiment plan

## Implementation gate rule

The first gameplay implementation sprint begins only when all of the following are true or explicitly, safely scoped out:

1. Final rules and other deterministic gameplay blockers are resolved.
2. No final/`LOCKED` gameplay authority contains implementation-affecting unresolved `TBD` or broken dependencies.
3. The canonical domain model and invariant registry exist.
4. The minimum architecture ADR set is Accepted.
5. Timer, reconnect, persistence/idempotency, identity/authorization and randomness semantics are deterministic.
6. Canonical dependency integrity can be checked and passes.
7. Golden Game scenarios are sufficiently defined to implement against.
8. The AEGIS/MUR adversarial qualification matrix is defined.

Until these conditions are met, the canonical decision is **IMPLEMENTATION BLOCKED**.

## Real-user release rule

A later real-user competitive release additionally requires:

- unresolved CRITICAL findings = 0,
- unresolved release-blocking HIGH findings = 0,
- Golden Game suite PASS on exact SHA,
- required AEGIS/MUR qualification profiles PASS on exact SHA,
- canonical integrity PASS,
- security/privacy/supply-chain evidence PASS,
- performance/fairness budgets satisfied,
- no known partial-failure path that can create duplicate score, duplicate entitlement or authoritative state corruption.
