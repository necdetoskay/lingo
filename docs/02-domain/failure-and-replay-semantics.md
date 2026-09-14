# Lingo Failure and Replay Semantics

Status: **Canonical foundation**  
Version: 1.0  
Tracking: #16

## Purpose

This document defines technology-independent behavior for retries, reconnects, duplicate delivery, ambiguous responses and stale state. Persistence implementation details remain owned by ADR-005 (#9).

## Core rule

Failures may delay progress, but they must not silently create a second logical gameplay action, duplicate score, duplicate entitlement, restored attempt or alternate random outcome.

## Replay classes

### Safe replay of the same committed action

If the same `actionId` and same canonical payload are received again after the action already committed:

- do not execute side effects again,
- return or reconstruct the prior logical result where possible,
- preserve the same authoritative revision/result identity.

### Conflicting replay

If the same `actionId` is reused with a materially different canonical payload:

- reject fail-closed,
- do not interpret it as a new action,
- record an auditable conflict/security reason where appropriate.

### Stale different action

If a different action was created against an older client-observed state:

- evaluate it against the current authoritative state,
- accept only if it is still legal now,
- otherwise reject as stale/illegal,
- never roll authoritative state back to match the client.

## Response loss

If a transition commits but the response is lost:

- a retry with the same action identity must resolve to the committed result,
- the retry must not repeat score/attempt/claim side effects.

This requirement applies even if the caller cannot distinguish timeout-before-commit from timeout-after-commit.

## Duplicate system triggers

Timeout/reveal/scheduled system triggers may be delivered more than once.

Handlers must verify current authoritative state and trigger identity/revision before mutating.

A duplicate timeout after a question already advanced is a no-op/rejected stale trigger, not a second completion.

A duplicate timed reveal must not consume another hidden position or generate another random draw.

## Reconnect

Reconnect is state restoration, not entitlement restoration.

It may return:

- current stage/question state,
- active owner/eligibility,
- attempts remaining,
- visible/revealed state,
- scores,
- current authoritative deadlines,
- effective dataset/policy version,
- revision/sequence needed to resume safely.

Reconnect must not itself:

- add attempts,
- recreate steal entitlement,
- extend an existing deadline,
- reroll reveal order,
- re-award score,
- revert a terminal result.

## Stale snapshot handling

An older snapshot/revision must never overwrite a newer committed state.

If the persistence/runtime architecture allows concurrent writers, it must provide an equivalent to optimistic/pessimistic concurrency control sufficient to preserve INV-015.

The specific mechanism is decided by ADR-005.

## Partial failure categories

The implementation must explicitly test at least these conceptual windows:

### F1 — Validation succeeded, commit did not happen

No authoritative gameplay effect exists. Retry may process as a fresh first-seen action if the idempotency record confirms no commit.

### F2 — Authoritative state committed, response lost

Retry resolves idempotently to the committed result.

### F3 — State mutation committed, required durable audit/event write uncertain

The persistence design must recover without rolling back already visible authoritative facts incorrectly or duplicating the mutation. ADR-005 must define the transactional/outbox-or-equivalent strategy.

### F4 — Durable audit/domain fact committed, in-memory/publication step failed

Recovery may republish/reconstruct non-authoritative downstream effects, but must not reapply the domain mutation.

### F5 — Process restart between steps

Restart recovery must derive the one authoritative result from durable evidence. It must not guess by re-executing an unsafe mutation without replay guards.

## Consumer failure

Experience/media/animation/haptic/telemetry consumer failure is isolated under INV-020.

A committed gameplay transition remains committed even when a presentation consumer fails.

Consumer retry may replay presentation carefully according to ADR-008 semantics, but cannot mutate gameplay.

## Randomness failure/replay

Gameplay-affecting random choices are initialized once for the logical Question according to #18.

Recovery/retry must reuse the same persisted/derived authoritative choice and cannot draw again simply because a process restarted.

## Word-policy failure

A competitive decision cannot silently fall back to a different dataset/policy version because the expected version is missing/corrupt.

The exact recovery/fail-closed policy is owned by #3/#10, but version mismatch must be explicit and deterministic.

## Audit requirements

For committed/rejected mutations, the system should eventually be able to explain at least:

- action identity,
- actor/player context,
- authoritative source revision,
- final result/reason,
- resulting revision,
- causation for any score/entitlement change,
- dataset/policy version where relevant.

Detailed fields/retention/privacy are owned by #22.

## Verification matrix

At minimum, #21 must eventually cover:

- same action/same payload replay,
- same key/different payload conflict,
- response loss after commit,
- duplicate timeout,
- duplicate claim,
- reconnect after consumed entitlement,
- restart after score commit,
- stale snapshot update attempt,
- consumer failure after domain commit,
- randomness replay/restart,
- dataset version mismatch.
