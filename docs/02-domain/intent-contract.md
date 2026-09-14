# Lingo Authoritative Intent Contract

Status: **Canonical foundation**  
Version: 1.0  
Tracking: #16

## Purpose

This document defines the minimum contract for mutating client/server intents. It is transport-independent and does not assume WebSocket, HTTP, RPC or any specific runtime.

## Principle

Clients submit **intent**. The authoritative server decides whether that intent is allowed in the current state.

A client may request `SUBMIT_GUESS`, `CLAIM_STEAL`, `SUBMIT_STEAL_GUESS` or future equivalent actions. It does not authoritatively set score, ownership, eligibility, deadline, stage state or question result.

## Minimum envelope

Every mutating intent must carry or resolve to the equivalent of:

- `actionId` — stable idempotency identity,
- `matchId`,
- actor/session identity from the authenticated connection/context,
- intent type,
- canonical payload,
- optional client-observed revision for stale-state detection,
- optional client timestamp only when permitted by ADR-004; never authority by itself.

Where question/stage scoped, include canonical identifiers such as `stageId` and `questionId`.

## Canonical validation order

The authoritative handler should evaluate the logical equivalent of:

1. Envelope/schema validity.
2. Authenticated/session identity validity.
3. Match existence and membership.
4. Current authoritative revision/state.
5. Intent is legal in the current state.
6. Actor currently owns/is eligible for the action.
7. Deadline/timer guard where applicable.
8. Idempotency/replay guard.
9. Domain validation/evaluation.
10. Atomic authoritative transition and required durable side effects.
11. Emit accepted domain/experience facts only after the authoritative transition is committed according to the relevant contract.

Implementation may reorder checks only when it preserves equivalent fail-closed semantics and does not leak sensitive state.

## Idempotency semantics

### First-seen action ID

Process according to current authoritative state. Persist enough evidence, according to ADR-005, to recognize future replay where required.

### Replay: same action ID + same canonical payload

Return/reconstruct the prior committed result or an equivalent idempotent response. Do not repeat side effects.

### Conflict: same action ID + different canonical payload

Reject fail-closed with a stable conflict/replay reason. Never treat it as a fresh action.

## Payload canonicalization

Idempotency comparison must use a deterministic canonical representation/fingerprint. Equivalent harmless serialization differences must not create two logical actions, while materially different payloads must not collide.

Exact hashing/serialization technology is an implementation choice but must be covered by tests.

## State and revision guards

A client-observed revision may help detect stale actions but cannot replace server-side legality checks.

If an intent was valid in revision N but arrives after the authoritative state advanced to revision N+1:

- replay of the already committed same action is handled idempotently,
- a different stale action is accepted only if the canonical current-state rules still permit it,
- otherwise it is rejected as stale/illegal.

The server must not roll authoritative state back to the client's revision.

## Ownership and authorization

For owner-scoped actions the server resolves:

`session identity -> match membership -> player identity -> current authoritative state -> ownership/eligibility -> intent permission`

Client-supplied ownership claims are advisory data at most.

Detailed security/session semantics are owned by ADR-007 (#11) and #17.

## Timer-sensitive intents

Timer/deadline decisions are server-authoritative.

- client render time is not authoritative,
- client receipt time is not the claim-winner rule,
- reconnect does not reset existing deadlines,
- exact-boundary/grace policy is owned by ADR-004 (#4),
- repeated timeout processing must be idempotent.

## Reason codes

Rejected intents should produce stable machine-readable reason categories suitable for tests, UX explanation and audit without leaking sensitive details.

Recommended families:

- `MALFORMED_INTENT`,
- `UNAUTHENTICATED`,
- `NOT_MATCH_MEMBER`,
- `NOT_AUTHORIZED_OWNER`,
- `ILLEGAL_STATE`,
- `STALE_INTENT`,
- `DEADLINE_EXPIRED`,
- `IDEMPOTENCY_CONFLICT`,
- `ALREADY_COMPLETED`,
- `ENTITLEMENT_CONSUMED`,
- `INVALID_GUESS`,
- domain-specific reasons defined by accepted rules.

Final public/user-facing wording is a UX concern; the machine reason remains deterministic.

## Non-mutating reads

Snapshot/read requests are outside mutation idempotency but must still enforce identity/match access policy and must not expose hidden answer/randomness information.

## Verification requirements

At minimum test:

- same action/same payload replay,
- same action/different payload conflict,
- wrong-player action,
- stale action after transition,
- terminal-question action,
- duplicate steal claim,
- deadline-expired action,
- reconnect then replay,
- malformed/oversized payload behavior when implementation limits are defined.
