# Lingo Authoritative Intent Contract

Status: **Canonical foundation**  
Version: 1.1  
Tracking: #16 / #17

## Change rationale — 1.1

Binds the mutation contract to the accepted Competitive Security & Abuse Threat Model: current session security version, current connection generation, server-resolved player identity, bounded payload processing and fail-closed stale/revoked behavior are now explicit.

## Purpose

This document defines the minimum contract for mutating client/server intents. It is transport-independent and does not assume WebSocket, HTTP, RPC or any specific runtime.

## Principle

Clients submit **intent**. The authoritative server decides whether that intent is allowed in the current state.

A client may request `SUBMIT_GUESS`, `CLAIM_STEAL`, `SUBMIT_STEAL_GUESS` or future equivalent actions. It does not authoritatively set score, ownership, eligibility, deadline, stage state, question result, random seed/order or effective word-policy version.

## Minimum envelope/context

Every mutating intent must carry or resolve to the equivalent of:

- `actionId` — stable idempotency identity;
- `matchId`;
- actor/session identity from authenticated server context;
- current session security/revocation version or equivalent server-side lineage state;
- current mutation-authoritative connection/session generation or equivalent lease;
- intent type;
- canonical payload;
- optional client-observed revision for stale-state detection;
- optional client timestamp only as non-authoritative metadata.

Where question/stage scoped, include canonical identifiers such as `stageId` and `questionId`.

The authoritative `playerId` is resolved from server-owned session + match-membership state. A client-supplied player identifier cannot select another player's authority.

## Canonical validation order

The authoritative handler evaluates the logical equivalent of:

1. Bounded envelope/schema/size validity.
2. Authenticated/session identity validity.
3. Session security/revocation version is current.
4. Connection/session generation is current for mutation authority.
5. Match existence and membership.
6. Server-resolved player identity binding.
7. Current authoritative revision/state.
8. Intent is legal in the current state.
9. Actor currently owns/is eligible for the action.
10. Deadline/timer guard where applicable.
11. Idempotency/replay guard.
12. Domain validation/evaluation.
13. Atomic authoritative transition and required durable side effects.
14. Emit accepted domain/experience/audit facts only after the authoritative transition is committed according to the relevant contract.

Implementation may reorder physical checks only when it preserves equivalent fail-closed semantics, stale-session rejection and non-disclosure of protected state.

## Session and connection authority

A valid identity alone is not enough: mutation authority is tied to the current server-controlled session security state and current connection/session generation.

After revocation or successful reconnect/takeover advances those values:

- an older credential/security version cannot mutate;
- an older connection generation cannot mutate even if the socket remains open;
- stale authority is rejected before gameplay mutation or idempotent-result disclosure.

The exact credential/reconnect-token representation is owned by ADR-007; the security semantics are owned by the Competitive Security & Abuse Threat Model.

## Idempotency semantics

### First-seen action ID

Process according to current authoritative security/state guards. Persist enough evidence, according to ADR-005, to recognize future replay where required.

### Replay: same action ID + same canonical payload

Only after the current caller/session is still authorized to access that protected result, return/reconstruct the prior committed result or an equivalent idempotent response. Do not repeat side effects.

### Conflict: same action ID + different canonical payload

Reject fail-closed with `IDEMPOTENCY_CONFLICT`. Never treat it as a fresh action. Record an audit/security signal.

## Payload canonicalization

Idempotency comparison uses a deterministic canonical representation/fingerprint. Equivalent harmless serialization differences must not create two logical actions, while materially different payloads must not collide.

Exact hashing/serialization technology is an implementation choice but must be covered by tests.

## Bounded payload behavior

Before expensive gameplay/domain work, implementation must enforce transport/runtime bounds for payload size, nesting/complexity and schema.

Malformed/oversized input cannot mutate state and must not cause unbounded logging or resource use.

Exact byte thresholds belong to the selected transport/runtime, but absence of a hard bound is not acceptable.

## State and revision guards

A client-observed revision may help detect stale actions but cannot replace server-side legality checks.

If an intent was valid in revision N but arrives after authoritative state advanced to N+1:

- exact replay of an already committed same action is handled idempotently only for a still-authorized caller;
- a different stale action is accepted only if current canonical state still independently permits it;
- otherwise it is rejected as stale/illegal.

The server never rolls authoritative state back to the client's revision and never silently remaps a stale action to a different turn/question.

## Ownership and authorization

For owner-scoped actions the server resolves:

`valid session -> current security version -> current connection generation -> match membership -> bound player identity -> current authoritative state/revision -> ownership/eligibility -> intent permission`

Client-supplied ownership, score, eligibility, deadline or player claims are advisory/untrusted data at most.

Detailed credential technology is owned by ADR-007 (#11); threat/fail-closed semantics are defined by #17.

## Timer-sensitive intents

ADR-004 is authoritative:

- client render/send time is not authoritative;
- client receipt time is not the claim-winner rule;
- reconnect does not reset existing deadlines;
- on-time means `authoritativeReceivedAt <= deadlineAt`;
- timeout may terminally close only after `authoritativeNow > deadlineAt`;
- repeated timeout processing is idempotent.

## Randomness-sensitive state

The Authoritative Randomness & Fairness Contract is authoritative:

- client cannot provide a seed/order;
- duplicate/replayed initialization cannot re-roll;
- reads/snapshots do not expose future hidden reveal order without canonical need.

## Reason codes

Rejected intents produce stable machine-readable reason categories suitable for tests, UX explanation and audit without leaking sensitive details.

Minimum families:

- `MALFORMED_INTENT`
- `PAYLOAD_TOO_LARGE`
- `UNAUTHENTICATED`
- `SESSION_STALE_OR_REVOKED`
- `STALE_CONNECTION`
- `NOT_AUTHORIZED_OR_NOT_FOUND` where non-disclosure is required
- `NOT_MATCH_MEMBER` for safe/internal contexts
- `NOT_AUTHORIZED_OWNER`
- `ILLEGAL_STATE`
- `STALE_INTENT`
- `DEADLINE_EXPIRED`
- `IDEMPOTENCY_CONFLICT`
- `ALREADY_COMPLETED`
- `ENTITLEMENT_CONSUMED`
- `INVALID_GUESS`
- domain-specific reasons defined by accepted rules.

Final public/user-facing wording is a UX concern; machine reasons remain deterministic.

## Non-mutating reads

Snapshot/read requests are outside mutation idempotency but still enforce current session validity, membership/access policy and protected-state filtering. They must not expose hidden answers, future reveal order, raw secrets or another match's protected state.

## Verification requirements

At minimum test:

- wrong-player / wrong-match action;
- stale/revoked session;
- old connection after reconnect generation advances;
- same action/same payload replay by still-authorized caller;
- same action/different payload conflict;
- stale action after transition;
- terminal-question action;
- duplicate steal claim;
- deadline-expired/exact-boundary action;
- reconnect then replay;
- client-supplied score/owner/deadline/randomness fields;
- malformed/oversized payload;
- unauthorized read/snapshot hidden-data filtering.

## Related canonical sources

- `docs/04-architecture/competitive-security-and-abuse-model.md`
- `docs/02-domain/invariants.md`
- `docs/02-domain/failure-and-replay-semantics.md`
- `docs/02-domain/authoritative-randomness.md`
- ADR-004
- ADR-007 (planned)
- ADR-005 (planned)
