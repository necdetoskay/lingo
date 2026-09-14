# Lingo Domain Model

Status: **Canonical foundation**  
Version: 1.0  
Tracking: #16

## Purpose

This document defines the framework-independent authoritative domain model for Lingo Classic Mode. It does not select a database, realtime transport, mobile framework or persistence technology.

The canonical gameplay specifications remain authoritative for stage-specific rules. This document defines the entities, ownership boundaries and identity relationships required to implement those rules deterministically.

## Aggregate boundary

The initial competitive aggregate is `Match`.

A Match owns the authoritative gameplay state required to determine competitive outcomes for one game session. A server implementation may persist or partition this state differently, but those storage decisions must not change the domain semantics.

Conceptually:

```text
Match
  |- MatchPlayers[]
  |- CurrentStage
  |- StageState
  |- CurrentQuestion
  |    |- Attempts
  |    |- Visible/Revealed state
  |    |- Ownership
  |    |- Deadlines
  |    |- Steal entitlements/claims when applicable
  |    |- Randomness reference/order when applicable
  |- ScoreLedger
  |- EffectiveWordPolicyVersion
  |- Revision / authoritative sequence
```

## Core entities and value objects

### Match

Canonical responsibilities:

- identify the competitive session,
- define participating players,
- own current lifecycle/state,
- own stage/question progression,
- pin the effective word-policy/dataset version according to the accepted policy,
- own the authoritative score ledger,
- expose a monotonic authoritative revision/sequence suitable for stale-state detection,
- reject mutation after terminal completion.

A Match must not trust client-provided score, ownership, eligibility, timer state or stage state.

### MatchPlayer

Represents one authoritative player identity inside a Match.

Minimum semantics:

- `playerId`: match-scoped stable identity,
- `sessionIdentityRef`: identity/session reference defined by ADR-007,
- participation state,
- current eligibility/ownership derived from authoritative state,
- score derived from committed score awards.

Display name/profile data is presentation metadata and must not become the authority for identity.

### Stage

Represents the active Classic Mode stage and its stage-specific state.

Initial stage identities:

- Stage 1 — Warm-up,
- Stage 2 — Steal,
- Stage 3 — Duel,
- Final — defined by #2 before implementation.

Stage-specific rules remain in `docs/01-game-design/classic-mode/`.

### Question

Represents one authoritative puzzle instance.

Minimum canonical identity/context:

- `questionId`,
- `stageId`,
- answer reference/value available only to authoritative evaluation code,
- effective word-policy/dataset version,
- lifecycle state,
- current ownership,
- attempts/attempt budget,
- revealed/visible state,
- deadline references where applicable,
- randomness/reveal-order reference where applicable,
- terminal result,
- authoritative revision.

A Question instance is immutable with respect to its identity-defining policy inputs after activation. Reconnect or replay must not silently create a new logical Question under the same `questionId`.

### Attempt

Represents a committed answer/guess attempt.

Minimum semantics:

- `attemptId`,
- owning/submitting `playerId`,
- `questionId`,
- authoritative ordinal or attempt number,
- normalized submitted value,
- validation classification,
- answer evaluation result,
- commitment timestamp/order,
- resulting state transition reference.

An Attempt becomes consumed when the canonical rule says the submission has been committed. Reconnect cannot restore a consumed Attempt.

### StealEntitlement

Stage 2 domain concept representing whether a player remains eligible to claim/consume one steal attempt for a Question.

Minimum semantics:

- `questionId`,
- `playerId`,
- entitlement state,
- consumption reference when used.

A player must not receive the same entitlement twice because of reconnect, retry, duplicate delivery or worker restart.

### StealClaim

Represents one authoritative Stage 2 claim decision.

Minimum semantics:

- `claimId`,
- `questionId`,
- `playerId`,
- action/idempotency identity,
- authoritative receipt/order metadata required by ADR-004/ADR-003,
- accepted/rejected result,
- reason code.

A claim is not authoritative merely because the client says it was first.

### ScoreAward

Represents a committed award of points.

Minimum semantics:

- `scoreAwardId`,
- `matchId`,
- `stageId`,
- `questionId` or stage/match bonus source,
- beneficiary `playerId`,
- amount,
- reason/type,
- causation reference,
- authoritative revision/order.

The score ledger must make duplicate award prevention testable. Current score is derived from committed authoritative awards or an equivalent representation that preserves the same invariants.

### Deadline

Value object representing an authoritative time boundary.

The exact clock representation, grace/latency policy and exact-boundary behavior are owned by ADR-004 (#4).

Domain requirements independent of technology:

- deadline identity/purpose is explicit,
- reconnect does not extend it,
- client countdown is not authority,
- repeated timeout delivery cannot create duplicate transitions,
- different timer purposes are distinguishable.

### WordPolicyVersion

Represents the effective lexical/validation policy used for competitive evaluation.

It must be possible to explain which dataset/policy version evaluated a guess. The final pinning/negotiation/storage rules are owned by #3 and ADR-006 (#10).

### AuthoritativeAction

Represents a mutating client/server intent identity suitable for replay/idempotency control.

Minimum conceptual identity:

- `actionId` / idempotency key,
- actor/session identity reference,
- match/question scope,
- intent type,
- canonical payload fingerprint,
- first-seen/committed result reference.

The detailed contract is in `intent-contract.md`.

## Ownership model

Ownership is authoritative state, not a client assertion.

Examples:

- Stage 1 standard question ownership belongs to the active primary player.
- Stage 2 primary ownership changes into claim/steal ownership according to canonical Stage 2 rules.
- Stage 3 normal rotation and transferred exclusive ownership are represented explicitly.

Every owner-scoped mutation must prove that the authenticated/session identity maps to the authoritative player who currently owns that mutation right.

## State revision

Authoritative state must expose a monotonic revision/sequence or equivalent concurrency token sufficient to:

- detect stale snapshots/intents,
- correlate committed transitions,
- prevent older state from overwriting newer state,
- support replay/debugging.

The concrete persistence mechanism is owned by ADR-005 (#9).

## Domain events vs experience events

Durable domain/audit facts and in-process Experience Events are different concepts.

- Domain state/transition determines what actually happened.
- Durable audit/domain-event strategy is owned by ADR-005.
- Presentation feedback is owned by ADR-008 / Game Experience Event System.
- Experience consumers must never become the source of authoritative state.

## Unresolved dependencies

This domain model intentionally does not invent unresolved rules. The following remain external blockers:

- Final stage/winner/tie-break: #2,
- invalid-word semantics: #3,
- timer/deadline/grace: #4,
- room/reconnect/eligible-player lifecycle: #5,
- persistence atomicity: #9,
- word dataset synchronization/pinning details: #10,
- identity/authentication: #11,
- stage transition UX/state: #12,
- authoritative randomness details: #18.

## Acceptance rule

An implementation may choose different class/table/type names, but it must preserve the canonical identities, ownership boundaries and invariants defined by this domain package.
