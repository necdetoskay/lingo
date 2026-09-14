# ADR-004 — Authoritative Timer and Latency Policy

Status: **Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #4

## Context

Lingo Classic Mode contains gameplay decisions whose result depends on authoritative time: Stage 1 bonus deadlines and timed reveals, Stage 2 steal claim windows, Stage 2 steal-answer windows, reconnect behavior, and duplicate timeout delivery.

Client-local countdowns, client-supplied timestamps, network retry order, or presentation timing must never decide a competitive outcome.

## Decision

### 1. The authoritative match sequencer owns time decisions

Every competitive match is processed through one authoritative sequencing boundary. The backend/runtime ADR may choose the implementation mechanism, but it must preserve the semantics in this ADR.

A client countdown is presentation only. The client cannot extend, shorten, or redefine an authoritative deadline.

### 2. Deadline representation

Authoritative timers use a persisted absolute `deadlineAt` plus timer identity and expected state/revision metadata.

Within a running process, monotonic elapsed-time facilities should be used where available to avoid wall-clock jumps. On recovery/restart, the persisted `deadlineAt` remains authoritative and is never recomputed as `now + duration`.

### 3. Intent timestamp authority

The authoritative acceptance timestamp is captured when an intent enters the state-owning authoritative match sequencer.

Client-provided send timestamps, device clocks, animation timestamps, or socket receipt time outside the authoritative sequencing boundary are not competitive authority.

### 4. Exact boundary rule

An intent is on time when:

`authoritativeReceivedAt <= deadlineAt`

An intent is late when:

`authoritativeReceivedAt > deadlineAt`

Therefore an intent accepted by the authoritative sequencer at exactly the deadline is valid.

The initial PoC uses **no additional gameplay grace window**. Any future latency/grace allowance requires a new ADR revision, abuse analysis, and boundary tests.

### 5. Scheduled milestone ordering

For timed reveal/value-decay milestones occurring at timestamp `T`:

- answer commitment with `authoritativeReceivedAt < T` stops the timer before that milestone;
- answer commitment with `authoritativeReceivedAt >= T` observes that milestone as already due.

This makes exact reveal-boundary behavior deterministic.

### 6. Timer cancellation is part of the authoritative transition

When an answer commitment stops a timer, the state transition must atomically establish that the timer is no longer active for that state/revision.

A delayed scheduled callback cannot revive or reapply the timer. Timer callbacks carry sufficient timer identity and expected revision metadata to become a no-op when stale.

### 7. Timeout delivery is a trigger, not authority by itself

A timeout worker/scheduler may request a timeout transition, but the authoritative state machine re-checks:

- current state,
- timer identity,
- expected revision,
- deadline,
- terminal status.

Repeated timeout delivery is idempotent and cannot double-transition, double-reveal, double-transfer ownership, or double-award score.

### 8. Reconnect never extends a deadline

Reconnect returns the current authoritative snapshot and existing `deadlineAt`. It does not restart or extend a timer and does not restore consumed attempts/claims.

### 9. Stage 1 bonus

- Total decision deadline: 10 seconds from authoritative bonus-question start.
- Timed reveal milestones: +2s, +4s, +6s, +8s.
- Exact 10.000s commitment is accepted when it reaches the authoritative sequencer at the deadline.
- A commitment before a reveal milestone prevents that milestone; commitment at or after the milestone observes the reveal/value reduction first.
- Once answer commitment is accepted, no later timed reveal may occur.

### 10. Stage 2 steal timers

Claim and answer timers are separate authorities:

- steal claim window: 5 seconds;
- accepted claimant answer window: 3 seconds.

The claim winner is the first eligible claim accepted by the authoritative sequencer at or before the claim deadline. Client receipt/render order is irrelevant.

### 11. Stage 2 bonus-value decision

The Stage 2 bonus value is **frozen at the moment the primary player's direct answer commitment is authoritatively accepted**.

If that commitment fails and a steal chain begins, the frozen value does not continue to decay during claim or answer windows.

Rationale: stealers must not be penalized by network/claim-chain duration they do not control, and the score must be reconstructable from one authoritative commitment point.

If the primary commitment occurs exactly at a timed reveal/value milestone, that milestone applies first under the ordering rule above, then the resulting value is frozen.

### 12. Simultaneous/equal-time ordering

If two intents have the same authoritative timestamp, the authoritative match sequence/order breaks the tie. That sequence must be deterministic and observable for audit/replay.

## Security and abuse constraints

- Client clocks are never trusted for competitive outcomes.
- A client cannot request a custom deadline or grace period.
- Replayed/stale timer or timeout identifiers fail closed/no-op according to current revision.
- Claim flooding must not create multiple accepted winners; abuse/rate behavior is covered by the security threat model.

## Verification

Tests must cover at minimum:

- immediately before / exactly at / immediately after deadline;
- immediately before / exactly at / immediately after reveal milestone;
- duplicate timeout delivery;
- stale timer revision;
- answer-commitment vs timer callback race;
- reconnect during active timer;
- Stage 2 claim and answer timers independently;
- simultaneous claims;
- Stage 2 bonus value freezing after primary commitment;
- restart/recovery without deadline reset.

Use fake/virtual clocks for deterministic rule tests.

## Consequences

### Positive

- One unambiguous deadline rule exists for every competitive timer.
- Client clock manipulation cannot grant extra time.
- Reconnect/retry cannot extend timers or duplicate timer effects.
- Stage 2 bonus steal scoring is independent of steal-chain duration.

### Cost

The selected backend/runtime must provide a state-owning sequencing boundary capable of preserving these semantics.

## Related canonical sources

- `docs/01-game-design/classic-mode/stage-01-warmup.md`
- `docs/01-game-design/classic-mode/stage-02-steal.md`
- `docs/02-domain/invariants.md`
- `docs/02-domain/failure-and-replay-semantics.md`
- `docs/04-architecture/architecture-principles.md`
- GitHub Issues #4, #16, #17, #18, #21
