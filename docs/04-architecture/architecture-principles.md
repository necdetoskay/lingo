# Architecture Principles

Status: Canonical foundation

## 1. Server-authoritative competitive state

For online play, the server is authoritative for:

- current stage/question
- active player
- attempt ownership
- timers and deadlines
- steal-claim winner
- score
- question termination
- reconnect eligibility

Clients render state and submit intents; they do not decide competitive outcomes.

## 2. Explicit game state machine

Gameplay must be implemented as explicit states and transitions rather than scattered UI booleans.

Illustrative states include:

- `QUESTION_READY`
- `PRIMARY_GUESSING`
- `GUESS_COMMITTED`
- `REVEALING`
- `STEAL_WINDOW`
- `STEAL_CLAIMED`
- `STEAL_ANSWERING`
- `BONUS_ACTIVE`
- `QUESTION_COMPLETE`
- `STAGE_COMPLETE`

Each transition must define actor, guard, side effects, timer behavior, and idempotency.

## 3. Deterministic rule engine

Word evaluation, attempt accounting, score calculation and state transitions must be pure/deterministic where possible and independently unit-testable from UI/network code.

## 4. Timer model

All competitive timers use authoritative deadlines rather than client decrement counters.

Clients may animate locally from synchronized timestamps, but final acceptance is determined by the authoritative deadline and documented grace/latency policy.

## 5. Intent/event separation

Clients send intents such as:

- `SUBMIT_GUESS`
- `CLAIM_STEAL`
- `SUBMIT_STEAL_GUESS`

The server emits accepted domain events such as:

- `GUESS_ACCEPTED`
- `GUESS_REVEALED`
- `STEAL_CLAIM_ACCEPTED`
- `SCORE_AWARDED`
- `QUESTION_ENDED`

This separation supports replay, debugging and auditability.

## 6. Idempotency

Network retries must not create duplicate guesses, duplicate steals, or duplicate score awards. Mutation intents should carry session/player/action identifiers suitable for deduplication.

## 7. Reconnectability

A reconnecting client receives an authoritative snapshot plus current deadlines. Reconnect must not reset timers or restore consumed attempts.

## 8. Local word lookup

Word validation should be fast and preferably local/offline-capable for non-authoritative UX, while online competitive acceptance must remain consistent with the server's versioned word policy.

## 9. Observability from the PoC

The PoC should record enough structured events to answer:

- which rule/state caused a question to end
- which player owned each attempt
- timer/claim timing
- validation reason
- score calculation inputs
- word dataset version

Do not log unnecessary raw microphone data or sensitive content by default.

## 10. Technology selection

Framework, database, realtime transport, mobile stack and deployment platform are intentionally not locked in this document. They require ADRs based on PoC requirements rather than preference alone.
