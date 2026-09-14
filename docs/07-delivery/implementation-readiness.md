# Implementation Readiness

Status: Canonical gate

Implementation must not begin until the following PoC decisions are closed or deliberately scoped out.

This gate is complemented by `docs/07-delivery/aegis-mur-hardening-plan.md` and GitHub master backlog #15.

## Game design

- [x] Stage 1 canonical rules documented and LOCKED v1.1
- [x] Stage 2 canonical rules documented and LOCKED v1.1
- [x] Stage 3 canonical rules documented and LOCKED v1.1
- [x] `LOCKED` status consistency repaired for Stage 1/2/3 (#26)
- [ ] Final-stage canonical rules
- [ ] Match winner/tie-break rules
- [ ] Stage transition UX
- [x] Exact gameplay invalid-input behavior for Stage 1/2/3 defined
- [x] Exact 10-second bonus deadline boundary defined by ADR-004
- [x] Stage 2 bonus-value behavior during steal chain resolved: value freezes at primary commitment

## Domain / integrity

- [x] Canonical `docs/02-domain/` exists (#16)
- [x] Domain model / aggregate boundaries defined
- [x] Stable invariant registry (`INV-*`) defined
- [x] Intent guard / ownership/idempotency semantics defined
- [x] Replay/idempotency semantics defined
- [x] Failure/replay semantics defined
- [x] Multiplayer lifecycle/reconnect semantics defined (#5)
- [ ] Authoritative randomness/reveal-order implementation contract accepted (#18)
- [ ] Canonical dependency/status integrity rules defined (#19)

## Word platform

- [x] Guess Dictionary vs Answer Pool separation
- [x] Turkish normalization principle
- [x] Duplicate-letter evaluation principle
- [x] Canonical gameplay invalid-input classification/policy
- [ ] Source/licensing decision for initial dictionary
- [ ] Full canonical linguistic invalid-word/exclusion policy
- [ ] Initial dataset schema finalized
- [ ] Dataset build/version process
- [ ] Competitive effective word-policy version pinning finalized

## Multiplayer

- [x] Server-authoritative principle
- [x] Claim-window / answer-window separation
- [x] PoC network latency/grace policy: no gameplay grace; authoritative sequencer time only (ADR-004)
- [x] Disconnect/reconnect rules per lifecycle state
- [x] Room lifecycle
- [x] Player-ready/start rules
- [x] Lobby host privileges and non-authority boundaries
- [x] Roster lock / no late join / PoC abandonment semantics
- [ ] Duplicate connection/session takeover security behavior (#11/#17)
- [ ] Stale/replayed session security behavior (#11/#17)
- [ ] Competitive abuse/threat model defined (#17)
- [ ] Realtime performance/fairness budgets defined (#20)

## Architecture

- [ ] Mobile technology ADR
- [ ] Backend/runtime ADR
- [ ] Realtime transport ADR
- [x] ADR-004 authoritative timer/latency accepted
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
- [ ] Executable before/exact/after deadline vectors implemented in Golden suite (#4/#13)
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
