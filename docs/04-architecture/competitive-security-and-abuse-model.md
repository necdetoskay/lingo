# Competitive Security & Abuse Threat Model

Status: **Canonical / Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #17

## Purpose

Protect Lingo competitive multiplayer authority against malicious, stale, replayed, duplicated or malformed client behavior.

The client is treated as untrusted. Correct UI behavior is never a security control.

This document defines security semantics independent of the final authentication provider, realtime transport, backend runtime or persistence technology. Those implementation choices must preserve this contract.

## Protected assets

The highest-value authoritative assets are:

- match membership and player identity binding;
- current stage/question state and revision;
- active owner / eligibility;
- attempts and steal entitlements;
- authoritative deadlines and timer identity;
- score awards and final match result;
- answer/hidden reveal state and future randomization data;
- effective word-policy/dataset version;
- idempotency/action history needed to prevent replay;
- durable audit evidence;
- session/reconnect authority.

## Trust boundaries

### Untrusted client/device

Assume the client can be modified, reverse engineered, scripted, delayed, replayed or run multiple times.

The client may lie about:

- `playerId`;
- ownership;
- eligibility;
- score;
- remaining attempts;
- deadlines/timestamps;
- reveal order/seed;
- dataset version;
- current revision;
- whether an action was already sent.

None of those claims become authority without server validation.

### Transport boundary

Transport may duplicate, delay, reorder or reconnect messages. Transport delivery does not itself prove gameplay authorization.

### Identity/session boundary

Authentication/reconnect credentials establish a session identity according to ADR-007. They do not bypass match membership, ownership, state, deadline or idempotency guards.

### Authoritative match boundary

Only the state-owning authoritative match process/transaction may commit competitive state transitions.

### Persistence/event boundary

Persistence must preserve idempotency/revision/security evidence according to ADR-005. Experience/presentation events are not authorization authority.

## Canonical mutation authorization chain

Every mutating intent must pass the logical equivalent of this chain:

1. transport/message envelope is within bounded schema/size limits;
2. session credential/context is valid;
3. session security version/revision is current and not revoked;
4. connection/session generation is current for mutation authority;
5. match exists and the session is a member;
6. session is bound to the authoritative `playerId` for that match;
7. current authoritative match/question revision is loaded;
8. intent type is legal in the current state;
9. actor owns/is eligible for that exact action;
10. timer/deadline guard passes when applicable;
11. idempotency/action replay guard passes;
12. domain payload is validated/evaluated;
13. authoritative transition commits atomically according to persistence rules;
14. response/audit/experience facts are emitted after commit according to their contracts.

Checks may be implemented in a different physical order only if equivalent fail-closed behavior and non-disclosure are preserved.

## Server-resolved actor identity

The authoritative actor/player identity is resolved from server-owned session/match membership state.

A client-supplied `playerId` may be present for correlation/UI convenience, but it must either match the server-resolved identity or be ignored/rejected. It cannot select another player's authority.

## Client-forbidden authority

There is no legitimate client intent that directly sets:

- score;
- claim winner;
- active player;
- attempt count;
- steal eligibility;
- question completion;
- authoritative deadline;
- word-policy version;
- random seed/reveal order.

Those are derived/committed server-side from accepted intents and canonical rules.

## Session security version

Each mutation-authoritative session lineage has a server-controlled security/revocation version (name is implementation-specific, e.g. `securityVersion` or `sessionRevision`).

When the session is revoked, replaced, explicitly signed out, or security policy requires invalidation, the authoritative version advances or the lineage is invalidated.

Credentials/connections bound to an older version are rejected fail-closed before gameplay mutation.

Exact credential/token representation is owned by ADR-007.

## Connection generation / duplicate connection policy

One PlayerSession has at most one **current mutation-authoritative connection generation**.

On a valid reconnect/takeover:

1. identity/reconnect proof is verified;
2. server atomically advances the session's `connectionGeneration` (or equivalent lease generation);
3. the new connection becomes current;
4. older connections/generations lose mutation authority immediately.

Old transports should be closed when practical, but correctness does not rely on successful socket closure. A stale connection that remains physically open cannot mutate gameplay.

A duplicate connection without valid reconnect/session proof is rejected.

## Stale/revoked session behavior

A stale/revoked session or stale connection generation:

- cannot submit new gameplay mutation;
- cannot restore consumed attempts/entitlements;
- cannot use a previously valid action payload to bypass current security state;
- receives a stable non-sensitive rejection such as `SESSION_STALE_OR_REVOKED` / `STALE_CONNECTION`.

Authentication/session authorization is evaluated before replay/idempotency result disclosure. A stale session is not entitled to retrieve prior protected results merely because it knows an old `actionId`.

## Match membership and IDOR resistance

Possession of a `matchId`, `questionId`, `playerId`, action ID or room identifier does not grant access.

Every read/mutation is scoped through server-side membership and player binding.

For identifiers that should not be publicly enumerable, use opaque high-entropy internal IDs. A human-friendly room/join code, if introduced, is discovery/join input rather than authentication authority and must be rate-limited.

Unauthorized cross-match/cross-player requests should avoid revealing sensitive existence/state differences. Implementations may use a generic `NOT_AUTHORIZED_OR_NOT_FOUND` public response while retaining precise internal audit reasons.

## Idempotency and replay

Canonical action-ID semantics remain:

### Same `actionId` + same canonical payload + still-authorized context

Return/reconstruct the prior committed result or equivalent idempotent response without repeating side effects.

### Same `actionId` + different canonical payload

Reject fail-closed as `IDEMPOTENCY_CONFLICT`; never treat as a fresh action. Record a security/audit signal.

### Replayed action after state advanced

If it is not the exact already-committed action replay, normal current-state/revision/ownership rules apply. Stale legality is never resurrected because the action used to be valid.

Durable replay-horizon/retention is finalized by ADR-005, but it must cover the active match and required recovery window.

## Revision/stale-state protection

Competitive mutation handlers operate against authoritative current revision.

A client-observed revision can detect stale state but cannot force rollback.

A competitive mutation that targets a stale state and is not the exact idempotent replay of an already committed action is rejected when current rules no longer permit it.

No silent "best effort" remapping of a stale claim/guess to a newer question/turn is allowed.

## Deadline/timestamp abuse

ADR-004 is authoritative.

- Client timestamps do not extend deadlines.
- Client clock changes do not change claim order.
- Client cannot request grace.
- Exact-boundary behavior is server sequencer based.
- Flooding messages near a deadline cannot create multiple accepted claim winners.

## Randomness manipulation

The Authoritative Randomness contract is authoritative.

- Client cannot supply seed/reveal order.
- Client cannot request re-roll.
- Future hidden reveal order is not exposed.
- Reconnect/retry cannot generate a new random result.

## Score/entitlement integrity

Score and entitlements are server-derived facts.

Security-critical invariants include:

- one question cannot award the same score twice;
- one steal entitlement cannot be consumed twice;
- reconnect cannot restore entitlement;
- terminal questions reject mutation;
- client cannot submit a score-award mutation;
- duplicate timeout/claim/result events cannot duplicate state transition.

## Claim flooding and rate abuse

Every externally reachable intent path must have bounded resource behavior.

At minimum apply rate/budget controls by suitable dimensions such as:

- session/player;
- connection;
- match/room;
- intent family;
- source/network where appropriate.

Exact numeric thresholds are implementation/performance decisions (#20), but "unbounded until infrastructure fails" is not an allowed policy.

Rate limiting must not become claim-winner authority. A claim still wins only through the canonical authoritative sequencing/state rules.

## Malformed/oversized payloads

Transport/runtime must enforce bounded payload size, nesting/complexity and schema limits before expensive gameplay processing.

Malformed/oversized input:

- fails closed;
- cannot mutate authoritative state;
- cannot trigger unbounded logging/error recursion;
- may increment abuse/security counters.

Exact byte limits belong to the selected transport/runtime ADRs, but tests must prove the bound exists.

## Room/session enumeration

Room/match identifiers are not secrets by themselves, but unlisted session discovery should not rely on sequential predictable IDs.

Join attempts are rate-limited and do not expose protected match/player details before membership authorization succeeds.

## Transport confidentiality

Real-user traffic carrying credentials/session authority must use authenticated encrypted transport (e.g. TLS/WSS/HTTPS as appropriate to the selected stack).

Credentials/tokens must not be placed into persistent logs or URLs where avoidable.

## Web-specific controls when applicable

If the selected client/backend uses cookie-authenticated web mutation endpoints, CSRF/origin protections must be applied according to that architecture. This threat model does not assume cookies or a browser-only client.

## Observability without secret leakage

Security-relevant rejection/audit events should record bounded structured metadata such as:

- reason code;
- match/question/action correlation IDs;
- pseudonymous player/session identifier;
- authoritative revision;
- session security/generation mismatch category;
- intent family.

Do not log raw credentials, reconnect tokens, secrets or unnecessary personal data. Detailed retention/redaction belongs to #22.

## Threats explicitly accepted as client-untrusted reality

The PoC does not depend on preventing:

- client reverse engineering;
- UI automation;
- local memory inspection;
- modified visual presentation.

Those must not compromise competitive authority because score/state/timers/answers/ownership are server-controlled.

Device attestation/anti-tamper may be considered later but is not a substitute for authoritative server validation.

## Canonical security invariants

- `SEC-001` Client-supplied player/ownership/score/deadline data is never authority by itself.
- `SEC-002` Every mutation is bound to a current valid session security version.
- `SEC-003` Only the current connection/session generation may mutate for a PlayerSession.
- `SEC-004` Match membership and player binding are checked server-side for every protected operation.
- `SEC-005` Same action ID with different canonical payload fails closed.
- `SEC-006` Stale/revoked session cannot obtain mutation authority through replay.
- `SEC-007` Stale state cannot be silently remapped to a newer turn/question.
- `SEC-008` Client clock/timestamp cannot change competitive deadline authority.
- `SEC-009` Client cannot choose/re-roll gameplay randomness.
- `SEC-010` Terminal state rejects further gameplay mutation.
- `SEC-011` Rate/resource controls are bounded but never become claim-winner authority.
- `SEC-012` Security logging excludes raw credentials/secrets.

## AEGIS/MUR attack matrix

The executable qualification suite (#21) must include at minimum:

1. Player B submits a valid-looking guess/claim for Player A.
2. Non-member uses a discovered `matchId`/`questionId`.
3. Revoked/stale session submits an otherwise valid intent.
4. Old connection sends mutation after a valid reconnect advances generation.
5. Same action ID + same payload replay.
6. Same action ID + different payload conflict.
7. Captured action replayed after question/turn revision changed.
8. Claim flood near exact deadline.
9. Manipulated client timestamp before/after deadline.
10. Reconnect after consumed steal entitlement.
11. Post-completion guess/claim/score mutation.
12. Oversized/malformed payload.
13. Client-supplied score/eligibility/deadline/random seed fields.
14. Room/match enumeration attempt without membership.
15. Duplicate simultaneous connections for one player identity.

## Dependencies / ownership

This threat model is accepted independently of technology choice, while implementation details remain owned by:

- ADR-007 / #11 — credential, guest/account identity and reconnect-token format;
- ADR-003 / #8 — realtime transport;
- ADR-005 / #9 — durable idempotency/security evidence and crash recovery;
- #20 — numeric performance/rate budgets;
- #22 — observability/privacy retention/redaction;
- #21 — executable adversarial qualification.

## Acceptance criteria

- Every authoritative mutation follows a server-side authorization chain.
- Wrong-player/cross-match/stale/replay behavior is fail-closed.
- Duplicate connection cannot create duplicate mutation authority.
- Client clocks/randomness/score/ownership claims are non-authoritative.
- Abuse paths have bounded resource requirements.
- Security events are observable without credential/PII leakage.
- Threats above map to executable MUR cases.
