# Lingo AEGIS/MUR Hardening Plan

Status: **Canonical hardening plan**  
Date: 2026-09-14  
Reviewed baseline: `main@d5b137ea4d8f64ef38e679cdf38fbf28735b6b3f`  
Tracking: #24  
Master readiness: #15

## 1. Purpose

This document records the AEGIS + MUR review of Lingo before gameplay implementation begins. The goal is not to redesign Lingo, but to close architectural, integrity, security, determinism, replay/reconnect, observability, performance and qualification gaps before those gaps become implementation debt.

Lingo remains in the pre-implementation / canonical design phase. The authoritative direction remains:

- rules before code,
- server-authoritative competitive state,
- explicit state machines,
- deterministic rule evaluation,
- authoritative deadlines,
- intent/event separation,
- idempotent mutation handling,
- reconnect from authoritative state,
- versioned word-policy/dataset behavior,
- measurable and replayable verification.

## 2. AEGIS classification

Risk class: **L2**.

Rationale:

- realtime multiplayer state changes directly affect competitive outcomes,
- timers and simultaneous claims create fairness-sensitive race conditions,
- reconnect and retry require replay/idempotency guarantees,
- score and entitlement state must resist duplicate mutation,
- future identity/authentication creates authorization and privacy boundaries,
- word-policy versions must remain consistent across a competitive session.

Initial review result:

- CRITICAL: 0
- HIGH: 7
- MEDIUM: 7
- Implementation gate: **BLOCKED**

`BLOCKED` means canonical preparation is intentionally incomplete; it is not a production incident.

## 3. Primary findings

### H1 — Missing canonical domain layer

README references `docs/02-domain/` as the canonical home of the domain model and invariants, but the directory does not exist.

Repair owner: #16.

### H2 — LOCKED specifications contain unresolved implementation-affecting behavior

Stage 1 depends on unresolved deadline and invalid-input semantics. Stage 2 is `LOCKED` while bonus steal value decay remains `TBD`. Stage 3 still depends on multiplayer lifecycle interpretation.

Repair owner: #26 with #3, #4 and #5.

### H3 — Required ADR dependency graph is incomplete

ADR-008 is Proposed and depends on ADR-004 / ADR-005, while the required ADR-001..007 set is not yet present as accepted canonical decisions.

Repair owners: existing ADR issues #4, #6, #7, #8, #9, #10, #11.

### H4 — Competitive security/abuse model is missing

Authorization, stale/replayed sessions, wrong-player mutation, duplicate connections, action replay, deadline abuse and bounded abuse handling are not yet canonical.

Repair owner: #17.

### H5 — Authoritative randomness contract is incomplete

Random reveal order is required to remain stable across replay/reconnect, but seed generation, persistence, concurrency, client exposure and restart semantics are not canonical.

Repair owner: #18.

### H6 — Persistence crash-window / transactional mutation semantics are incomplete

State mutation, idempotency records and durable audit/domain-event persistence require an explicit atomicity/recovery contract.

Repair owner: #9, extended by #25.

### H7 — Adversarial qualification is not yet a release/readiness gate

The existing test strategy is strong on deterministic functionality, but malicious/stale/replayed/out-of-order intents and partial failures need their own MUR qualification layer.

Repair owner: #21, using #13 as the Golden Game foundation.

## 4. Canonical hardening backlog

### Core integrity

- #16 — Domain Model & Invariant Registry
- #26 — LOCKED Spec Consistency Repair
- #19 — Canonical Manifest & Documentation Integrity Gate

### Security and fairness

- #17 — Competitive Security & Abuse Threat Model
- #18 — Authoritative Randomness & Fairness Contract
- #20 — Realtime Performance & Fairness Budgets

### Reliability, audit and privacy

- #9 — Persistence, snapshot and event/audit strategy
- #22 — Observability, Privacy & Audit Contract

### Verification and delivery

- #13 — Golden Game Test Suite
- #21 — Adversarial Qualification Gate
- #23 — Supply Chain & Self-Hosted Qualification Policy

### Coordination

- #24 — Canonical Hardening Plan Tracking
- #25 — Existing Issue Extensions Coordination
- #15 — Implementation Readiness Master Backlog

## 5. Required domain invariant direction

The final invariant registry is owned by #16. At minimum it must cover:

- a question cannot award its score twice,
- a consumed attempt cannot be restored by reconnect,
- a steal entitlement can be consumed at most once,
- a completed question rejects later mutation,
- only the authoritative owner may submit owner-scoped intents,
- repeated same action/idempotency key with the same payload is replay-safe,
- same action/idempotency key with a conflicting payload fails closed,
- reconnect cannot extend an authoritative deadline,
- the effective word-policy/dataset version is stable for the applicable competitive scope,
- random reveal order is immutable for a question instance,
- authoritative scoring is reconstructable without duplicate awards.

Every invariant must have a stable canonical ID and must map to verification cases.

## 6. MUR attack model

The executable MUR suite defined by #21 must eventually attack at least:

### Authorization

- wrong-player mutation,
- wrong-match mutation,
- stale/revoked session,
- duplicate session/socket takeover,
- unauthorized claim or score mutation.

### Replay and idempotency

- duplicate same-payload mutation,
- same action key with different payload,
- delayed stale intent,
- duplicate timeout,
- duplicate claim,
- completed-question replay.

### Partial failure and recovery

- authoritative state committed but response lost,
- state committed while event/audit persistence fails,
- restart between mutation steps,
- stale snapshot after newer durable state,
- retry after ambiguous response without double award.

### Timer/realtime

- before / exact / after deadline,
- reconnect inside active timer,
- simultaneous steal claims,
- jitter and reordered delivery,
- manipulated client timestamp.

### Randomness

- reconnect changes reveal order,
- restart changes reveal order,
- duplicate reveal causes another random draw,
- concurrent seed initialization,
- client-supplied/manipulated seed.

### Dataset/version integrity

- client/server word-policy mismatch,
- corrupt or missing dataset,
- interrupted dataset update,
- match/session changes dataset policy mid-flow.

## 7. Canonical status rule

A gameplay document may be considered `LOCKED` only when:

- no implementation-affecting `TBD` remains,
- required canonical dependencies exist,
- those dependencies have an allowed accepted status,
- deterministic outcomes have one interpretation,
- acceptance/golden scenarios can be derived from the document,
- version and change rationale requirements are satisfied.

A dependent rule may remain provisional while a required ADR/spec is unresolved, but it must not silently present itself as final authority.

## 8. Canonical integrity direction

#19 will define a machine-readable canonical manifest. The integrity gate should eventually fail on:

- missing canonical paths,
- broken ADR/spec dependencies,
- forbidden implementation-affecting `TBD` in `LOCKED` specs,
- duplicate canonical IDs,
- invalid status transitions,
- missing required versions,
- stale README/documentation maps,
- superseded documents referenced as active authority.

## 9. Exact-SHA qualification rule

When implementation exists, qualification evidence must identify the exact code revision being evaluated.

Minimum evidence:

- exact commit SHA,
- canonical manifest/revision,
- dataset/policy version where relevant,
- qualification profile,
- pass/fail counts,
- unresolved CRITICAL/HIGH findings,
- environment/runner identity at an appropriate non-secret level,
- timestamp.

The project should use self-hosted runner qualification rather than relying on GitHub-hosted Actions.

## 10. Gate order

Recommended sequence:

1. #16 domain model and invariants
2. #26 + #2/#3/#4/#5/#12 deterministic rule closure
3. critical ADR set, especially #4/#9/#11
4. #18 authoritative randomness
5. #17 competitive security/abuse
6. #22 observability/privacy/audit
7. #19 canonical integrity
8. #13 golden fixtures
9. #21 adversarial qualification design
10. #20 performance/fairness budgets
11. #23 supply-chain/self-hosted qualification policy
12. final AEGIS/MUR review
13. implementation-gate decision

The technology ADRs remain required, but they should be evaluated against these canonical constraints rather than driving the constraints.

## 11. Implementation gate PASS

Before the first gameplay implementation sprint:

- no unresolved implementation-affecting TBD remains inside a final/locked authority,
- canonical domain model and invariant registry exist,
- the minimum required ADR set is Accepted,
- timer, reconnect, persistence/idempotency, identity/authz and randomness semantics are deterministic,
- canonical dependency integrity passes,
- Golden Game scenarios are sufficiently defined to implement against,
- adversarial qualification matrix is defined.

## 12. Real-user release gate

Before real-user competitive release:

- CRITICAL findings = 0,
- HIGH findings = 0 unless explicitly accepted as non-release-blocking with rationale,
- Golden Game suite passes on exact SHA,
- required MUR profiles pass on exact SHA,
- canonical integrity passes,
- security/privacy/supply-chain evidence passes,
- performance/fairness budgets are satisfied,
- unresolved partial-failure paths do not permit duplicate score, duplicate entitlement or state corruption.

## 13. Existing issue extension policy

Do not duplicate work already owned by #4, #9, #11, #13 or #14. AEGIS/MUR additions are coordinated by #25:

- #4 gains atomic timer-cancel, duplicate timeout/reveal and abuse boundaries,
- #9 gains mutation/idempotency/audit atomicity and crash-window recovery,
- #11 gains authorization chain, revocation and duplicate-session behavior,
- #13 maps canonical invariants to deterministic fixtures,
- #14 adopts the rule `MODEL_HIGH_CONFIDENCE != GOLD`; gold truth requires human verification or explicitly accepted deterministic truth plus provenance/review version.

## 14. Current decision

Lingo architecture is **not being rewritten**.

The existing architecture principles remain directionally sound. AEGIS/MUR hardening is a repair-and-complete exercise designed to make the canonical foundation executable, internally consistent and resistant to multiplayer/replay/failure abuse before implementation begins.
