# Lingo Canonical Invariant Registry

Status: **Canonical foundation**  
Version: 1.0  
Tracking: #16

## Purpose

This registry gives stable IDs to rules that must remain true regardless of UI, transport, database or framework choices. Tests and AEGIS/MUR qualification must reference these IDs directly.

## Invariants

### INV-001 — Score award uniqueness

A logical scoring cause may award points at most once.

A retry, reconnect, duplicate delivery, response loss or worker restart must not create a second equivalent `ScoreAward`.

### INV-002 — Attempt consumption is monotonic

Once an attempt is authoritatively consumed, reconnect/replay cannot restore it.

Attempts remaining may stay the same only when the canonical rule says the submitted input did not consume an attempt.

### INV-003 — Steal entitlement is single-use

For one Stage 2 question, a player cannot consume the same steal entitlement more than once.

Reconnect, duplicate claim delivery or retry cannot recreate consumed entitlement.

### INV-004 — Terminal question immutability

Once a Question is terminal, later gameplay mutations that would change attempts, ownership, score or result are rejected.

Idempotent replay of the already committed terminal action may return the prior result, but it cannot create new side effects.

### INV-005 — Authoritative owner enforcement

An owner-scoped intent is accepted only when the authenticated/session identity resolves to the authoritative player who currently owns that action right.

Client-provided `playerId`, turn, eligibility or ownership fields are not sufficient authority.

### INV-006 — Idempotency key consistency

Reusing the same action/idempotency key with the same canonical payload is replay-safe and must not duplicate side effects.

Reusing the same key with a conflicting canonical payload fails closed.

### INV-007 — Deadline monotonicity

Reconnect, retry, resubscribe or snapshot refresh cannot move an authoritative deadline later unless a canonical rule explicitly creates a new timer window.

A client cannot extend a deadline by sending a different timestamp.

### INV-008 — Effective word-policy stability

The effective word-policy/dataset version used for a competitive decision must be deterministic and explainable.

A match/question may not silently switch to an incompatible policy in the middle of an active authoritative flow.

Final pinning scope is defined by #3/#10.

### INV-009 — Question randomness immutability

Random reveal order or equivalent gameplay-affecting random choice is determined once for the logical Question instance and remains stable across reconnect, replay and restart.

Duplicate timer/event delivery cannot trigger a fresh random draw.

### INV-010 — Score reconstructability

The authoritative score for a player must be explainable from committed canonical scoring facts or an equivalent ledger that preserves the same auditability and uniqueness properties.

### INV-011 — No negative attempt budget

Remaining attempts can never become negative.

### INV-012 — Stage 2 single accepted claim winner per claim window

A Stage 2 claim window may have at most one authoritative accepted claimant.

Concurrent requests cannot produce two winners for the same window.

### INV-013 — Timer purpose separation

Distinct authoritative timers such as Stage 1 bonus decision, Stage 2 steal claim and Stage 2 steal answer are not interchangeable and cannot satisfy each other's timeout transition.

### INV-014 — Reconnect does not mutate competitive rights

A reconnect operation restores authoritative state; it does not itself grant attempts, claims, score, ownership or bonus value.

### INV-015 — State revision monotonicity

Authoritative state revisions/sequences advance monotonically according to committed transitions. Older snapshots cannot overwrite newer committed state.

### INV-016 — Word validation version traceability

Every competitive guess decision must be attributable to an effective validation policy/dataset version.

### INV-017 — Stage 3 shared attempt budget integrity

For a Stage 3 Question, exactly the canonical shared attempt budget exists. Normal rotation or ownership transfer cannot multiply the budget.

### INV-018 — Stage 3 transfer exclusivity

After the first invalid guess causes remaining attempts to transfer, the receiving player becomes the exclusive owner of those remaining attempts according to the canonical Stage 3 rule.

A second invalid guess by that owner terminates the question; no further transfer is created.

### INV-019 — Completion bonus uniqueness

A stage/match completion bonus tied to one completion condition may be awarded at most once for the same beneficiary and canonical cause.

### INV-020 — Experience consumer non-authority

Experience/media/animation/haptic/telemetry consumers cannot mutate authoritative competitive state or determine competitive outcomes.

Consumer failure must not roll back or corrupt a committed authoritative transition.

## Verification mapping rule

Each invariant must eventually have:

- at least one positive test,
- at least one negative/forbidden test where meaningful,
- replay/idempotency coverage where meaningful,
- an owner test suite or fixture reference.

`docs/06-testing/test-strategy.md`, #13 and #21 own the executable verification layer.

## Change control

Changing an invariant requires:

1. rationale,
2. version increment,
3. review of affected stage specs/ADRs,
4. updated Golden/MUR tests before implementation changes.
