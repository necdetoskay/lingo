# Lingo Failure and Replay Semantics

Status: **Canonical foundation**  
Version: 1.1  
Tracking: #16 / #9

## Change rationale — 1.1

ADR-005 is now Accepted. The previously deferred persistence/concurrency/crash-window behavior is therefore resolved and referenced directly here.

## Purpose

Define technology-independent behavior for retries, reconnects, duplicate delivery, ambiguous responses, restart and stale state, aligned with ADR-005 persistence semantics.

## Core rule

Failures may delay progress, but they must not silently create a second logical gameplay action, duplicate score, duplicate entitlement, restored attempt or alternate random outcome.

## Replay classes

### Safe replay of the same committed action

If the same `actionId` and same canonical payload are received again after the action already committed:

- current caller/session authorization is checked first;
- no side effect executes again;
- the prior logical result is returned/reconstructed from durable idempotency evidence;
- the same authoritative result/revision identity is preserved.

### Conflicting replay

If the same `actionId` is reused with a materially different canonical payload:

- reject fail-closed as `IDEMPOTENCY_CONFLICT`;
- do not interpret it as a new action;
- retain auditable conflict/security evidence.

### Stale different action

If a different action was created against an older client-observed state:

- evaluate it against current authoritative state/revision;
- accept only if it is independently legal now;
- otherwise reject stale/illegal;
- never roll state back or silently remap the intent to a newer turn/question.

## Response loss

If a transition commits but the response is lost:

- the atomic state/idempotency/domain-audit commit is still authoritative;
- retry with the same action identity resolves to the committed result;
- retry cannot repeat score/attempt/claim side effects.

This requirement applies even when the caller cannot distinguish timeout-before-commit from timeout-after-commit.

## Duplicate system triggers

Timeout/reveal/scheduled system triggers may be delivered more than once.

Handlers verify current authoritative state, trigger identity, timer identity and revision before mutating.

A duplicate timeout after a question advanced is a no-op/stale trigger, not a second completion.

A duplicate timed reveal cannot consume another hidden position or generate another random draw.

## Reconnect

Reconnect is state restoration, not entitlement restoration.

It returns an authorized view generated from the latest committed durable revision, including where relevant:

- current stage/question state;
- active owner/eligibility;
- attempts remaining;
- visible/revealed state;
- scores;
- current authoritative deadlines;
- effective dataset/policy version;
- revision/sequence needed to resume safely.

Reconnect must not itself:

- add attempts;
- recreate steal entitlement;
- extend an existing deadline;
- reroll reveal order;
- re-award score;
- revert a terminal result.

## Stale snapshot handling

An older snapshot/revision never overwrites newer committed state.

ADR-005 requires a monotonically advancing authoritative revision plus storage-level optimistic/serialization protection (or equivalent) so two writers targeting one revision cannot both commit.

A stale cache/snapshot is discarded/reloaded when a higher durable revision exists.

## Partial failure categories

### F1 — Validation succeeded, atomic commit did not happen

No authoritative gameplay effect exists. The transaction is absent/rolled back. Retry may process as a first-seen action if durable idempotency evidence confirms no prior commit.

### F2 — Atomic commit succeeded, response lost

State, revision, required idempotency evidence and required domain/audit evidence are committed. Retry resolves idempotently to the existing result.

### F3 — State update and required idempotency/domain-audit evidence would diverge

ADR-005 forbids this as an accepted state. These effects belong to one atomic commit boundary. The observable result is either fully committed or not committed.

A design that can expose "state committed but required replay/audit evidence missing" is non-conforming.

### F4 — Commit succeeded, required downstream publication failed

Gameplay remains committed. A transactional outbox (or equivalent same-commit durable delivery record) remains pending and retries publication.

Duplicate publication is handled with stable message/event identity and cannot reapply domain mutation.

### F5 — Process restart before commit

No accepted mutation exists; storage transaction rollback/non-visibility applies.

### F6 — Process restart after commit but before response/publication

Recovery reloads the latest materialized state/revision and idempotency evidence. The action is already committed; response can be reconstructed and pending outbox work continues separately.

### F7 — stale cache survives restart/ownership transfer

Durable higher revision wins. Stale memory cannot overwrite committed state.

## Consumer failure

Experience/media/animation/haptic/telemetry consumer failure is isolated under INV-020 and ADR-008.

A committed gameplay transition remains committed even when a presentation consumer fails.

Transient consumer retry cannot mutate gameplay. If durable eventual publication is required, it originates from ADR-005 outbox/equivalent rather than from uncommitted memory.

## Randomness failure/replay

Gameplay-affecting random choices are initialized once according to the Authoritative Randomness contract.

The derived result/order is persisted before question activation. Recovery/retry reuses it and never redraws because a process restarted.

Missing/corrupt authoritative randomization state for an already-active question fails closed rather than rerolling.

## Timer failure/replay

Timer identity/deadline is durable authoritative state under ADR-004 + ADR-005.

A restarted worker restores the same deadline; it does not compute `now + duration`.

Timeout callback duplication is safe because state/revision/timer identity are rechecked.

## Score / entitlement failure safety

Score-award identity and entitlement consumption are persisted within the atomic authoritative mutation boundary.

- duplicate award attempts converge on the same immutable award identity/business key;
- reconnect/restart cannot restore consumed entitlement;
- response loss cannot cause a second award/consumption.

## Word-policy failure

A competitive decision cannot silently fall back to a different dataset/policy version because the expected version is missing/corrupt.

The exact dataset recovery/update policy remains #3/#10, but version mismatch is explicit and deterministic.

## Audit requirements

For committed transitions, durable evidence can explain at least:

- action identity;
- actor/player context;
- authoritative source/result revisions;
- transition/result/reason;
- causation for score/entitlement change;
- timer/deadline context when relevant;
- dataset/policy version when relevant;
- randomization/version reference when relevant.

Rejected/security decisions may use separate bounded decision/security audit records rather than polluting the committed domain-transition stream.

Detailed PII/redaction/calendar retention remains owned by #22.

## Verification matrix

At minimum #21 must cover:

- same action/same payload replay;
- same key/different payload conflict;
- concurrent duplicate action delivery;
- concurrent mutations on one revision;
- rollback before commit;
- crash after commit before response;
- response loss then retry;
- outbox publish failure/retry;
- duplicate outbox delivery;
- duplicate timeout;
- duplicate claim;
- reconnect after consumed entitlement;
- restart after score commit;
- stale snapshot overwrite attempt;
- consumer failure after domain commit;
- randomness replay/restart;
- dataset version mismatch.

## Related canonical sources

- ADR-005 — Persistence, Snapshot and Event/Audit Strategy
- ADR-004 — Authoritative Timer and Latency Policy
- `docs/02-domain/intent-contract.md`
- `docs/02-domain/authoritative-randomness.md`
- `docs/04-architecture/competitive-security-and-abuse-model.md`
