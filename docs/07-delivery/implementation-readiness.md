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
- [x] Authoritative randomness/reveal-order contract accepted (#18)
- [x] One-draw/persist-derived-result semantics defined
- [x] Reconnect/restart/duplicate reveal cannot re-roll active question randomness
- [x] Future reveal-order/seed secrecy boundary defined
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

## Multiplayer / competitive security

- [x] Server-authoritative principle
- [x] Claim-window / answer-window separation
- [x] PoC network latency/grace policy: no gameplay grace; authoritative sequencer time only (ADR-004)
- [x] Disconnect/reconnect rules per lifecycle state
- [x] Room lifecycle
- [x] Player-ready/start rules
- [x] Lobby host privileges and non-authority boundaries
- [x] Roster lock / no late join / PoC abandonment semantics
- [x] Competitive Security & Abuse Threat Model accepted (#17)
- [x] Server-resolved mutation authorization chain defined
- [x] Session security/revocation version requirement defined
- [x] Duplicate connection/takeover uses one current mutation-authoritative connection generation
- [x] Old connection generation loses mutation authority after valid reconnect/takeover
- [x] Stale/revoked session cannot regain authority via replay
- [x] Cross-match/wrong-player mutation fail-closed semantics defined
- [x] Client score/ownership/deadline/randomness fields are non-authoritative
- [x] Bounded malformed/oversized/rate-abuse requirements defined
- [ ] ADR-007 credential/account/guest/reconnect-token representation accepted (#11)
- [ ] Realtime performance/fairness numeric budgets defined (#20)

## Architecture

- [ ] Mobile technology ADR
- [ ] Backend/runtime ADR
- [ ] Realtime transport ADR
- [x] ADR-004 authoritative timer/latency accepted
- [x] ADR-005 persistence/snapshot/event-audit accepted
- [ ] ADR-006 local word-dataset storage accepted
- [ ] ADR-007 authentication/identity accepted
- [ ] Experience Event System ADR-008 dependencies are satisfied and its final status is consistent

## Reliability / observability / privacy

- [x] Materialized authoritative state is durable recovery authority (ADR-005)
- [x] Mutation + revision + idempotency + required domain/audit evidence commit atomically (#9)
- [x] Immutable score-award / entitlement integrity boundaries defined (#9)
- [x] Transactional outbox or equivalent required for must-deliver post-commit publication (#9)
- [x] Commit-before-response, response-loss and worker-restart recovery semantics defined (#9)
- [x] Stale snapshot/cache cannot overwrite a higher durable revision (#9)
- [x] Active timers restore existing deadline; they do not restart after recovery (#9)
- [x] Active-question randomization state restores existing derived result; missing state fails closed (#9/#18)
- [ ] Structured observability / correlation contract defined (#22)
- [ ] Privacy/redaction/minimum-data retention durations defined (#22)
- [x] Security model prohibits raw credential/secret logging
- [ ] Audit trail retention/field policy finalized (#22)

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
- [x] Randomness test contract permits explicit derived reveal-order fixtures (#18)
- [x] Security threat model defines required wrong-user/stale/replay/duplicate-connection attack cases (#17)
- [x] ADR-005 defines required persistence/crash-window failure-injection cases (#9)
- [ ] Executable fixed-order randomness fixtures implemented in Golden suite (#13)
- [ ] Executable before/exact/after deadline vectors implemented in Golden suite (#4/#13)
- [ ] Executable partial-failure/idempotency/recovery cases implemented (#21)
- [ ] AEGIS/MUR adversarial qualification matrix/executable cases defined (#21)

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
