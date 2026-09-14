# Authoritative Randomness & Fairness Contract

Status: **Canonical / Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #18

## Purpose

Define server-authoritative, replay-safe and fairness-preserving randomness for Classic Mode and future gameplay-affecting random decisions.

The first concrete use is Stage 1/2 timed hidden-letter reveal order. The contract also defines the extension boundary for future randomized answer/question selection without changing current stage rules.

## Core rule

A gameplay-affecting random choice is made **once** at the authoritative question/session initialization boundary and the resulting chosen outcome/order is persisted as canonical state.

The runtime must not depend on re-running a PRNG after reconnect, retry, replay, worker restart or software upgrade to reconstruct a previously made gameplay choice.

## Why the derived result is canonical, not only the seed

Persisting only a seed is insufficient because a future implementation could change:

- PRNG algorithm;
- shuffle implementation;
- language/runtime behavior;
- iteration order;
- randomization version.

Therefore the canonical active-question state stores the **derived reveal order itself**. A seed or entropy provenance value may also be stored for diagnostics/audit, but it is not required to reconstruct the active question.

## Randomization classes

### R1 — hidden reveal order

Current Classic Mode Stage 1/2 use this class.

### R2 — future answer/question selection

Not currently defined by this contract as a game-design rule. If later used, the same one-draw/persist-result principle applies and answer-selection policy must remain compatible with the versioned Answer Pool.

### R3 — presentation-only randomness

Purely cosmetic randomness that cannot affect score, time, eligibility, answer visibility, ownership or game outcome is outside competitive authority and may remain client-local if it cannot leak or change gameplay state.

## Stage 1 reveal order

At authoritative Stage 1 bonus-question initialization:

1. the answer and initially visible first position are already fixed;
2. the remaining hidden positions are enumerated;
3. one authoritative permutation of those hidden positions is generated;
4. the entire resulting order is stored on the question instance before the question becomes active;
5. timed reveal milestones consume positions from this stored order only.

Stage 1 currently uses up to four timed reveals at +2s/+4s/+6s/+8s under ADR-004.

## Stage 2 reveal order

The same rule applies to the Stage 2 bonus question:

1. answer and initial visible position are fixed;
2. hidden positions are enumerated;
3. one authoritative permutation is generated once;
4. resulting order is persisted before activation;
5. timed reveals consume that stored order.

Stage 2 bonus-value freeze and exact milestone timing remain governed by Stage 2 v1.1 and ADR-004.

## Entropy and generation

Production gameplay randomness must originate from a server-side cryptographically secure entropy source or an equivalently unpredictable platform primitive.

The client cannot:

- supply the seed;
- request a re-roll;
- choose shuffle order;
- influence the random source;
- provide a timestamp that becomes entropy authority.

A deterministic shuffle/permutation algorithm may be used internally, but its algorithm/version is recorded when useful for diagnostics. The persisted derived order remains the final authority.

## Initialization atomicity

Randomization initialization is part of question creation and must be protected by the same authoritative revision/compare-and-set/transaction boundary as other question initialization state.

Two workers racing to initialize the same question cannot create two competing orders.

Only one initialization may become canonical for a given `questionId` and creation revision.

A duplicate/replayed question-creation operation returns/reuses the existing canonical randomization result instead of drawing again.

## Randomization identity

Where relevant, authoritative randomized state should carry:

- `questionId`
- question/state revision
- `randomizationVersion`
- derived `revealOrder`
- optional entropy/seed provenance reference or non-secret diagnostic identifier
- initialization event/action identity

The exact persistence representation is finalized by ADR-005, but it must preserve these semantics.

## Client exposure policy

Future hidden reveal positions are gameplay-sensitive information and are not sent to the client merely for convenience.

Clients receive only what they need to render current authoritative state, such as:

- already revealed positions;
- current visible board;
- current reveal count;
- next authoritative deadline/milestone when applicable.

The full future `revealOrder`, raw seed or secret entropy must not be exposed before those choices become observable through gameplay.

## Reconnect and replay

Reconnect restores the existing authoritative reveal state and stored order. It never creates a new draw.

Replay/debugging uses the already-committed randomization result.

A replayed timer/reveal event:

- references the same question/revision;
- consumes no additional random draw;
- becomes a no-op if that reveal was already applied;
- cannot advance to a different position because of duplicate delivery.

## Restart/recovery

If the authoritative runtime restarts between reveals, recovery must restore the existing derived order and reveal cursor/count.

The system must **not** generate a replacement order because the seed/order is unavailable in volatile memory.

If the canonical derived order for an already-active randomized question is missing/corrupt and cannot be recovered from durable authoritative state, the system fails closed into recovery/error handling. It must not silently re-roll and continue.

## No strategic re-roll

Once a question becomes authoritative/active, operational problems, reconnect, client complaints, worker retry or perceived difficulty cannot trigger a new reveal order.

If question initialization discovers invalid data before activation, initialization may fail and choose/create a different valid question according to the future question-selection policy. After activation, randomization is immutable.

## Fairness relationship with timers

Randomness does not own time.

ADR-004 determines whether a reveal milestone is due. When a milestone is due, the next position comes from the stored authoritative order.

Answer commitment stopping a timer cannot cause a hidden extra random draw. Unused future positions remain unused.

## Testing and deterministic fixtures

Production entropy and test determinism are separated.

Test/golden fixtures may provide one of:

- an explicit fixed `revealOrder`; or
- a test-only deterministic entropy/seed source behind a non-production interface.

Production clients can never select that test source.

Golden fixtures should prefer explicit derived orders when the goal is to prove gameplay state transitions, because that keeps tests independent of PRNG implementation changes.

## MUR attack cases

The AEGIS/MUR suite must attack at least:

- reconnect after one or more reveals;
- server restart between reveals;
- duplicate reveal/timer delivery;
- repeated question initialization;
- two workers racing to initialize one question;
- client-supplied seed/order;
- stale snapshot followed by newer reveal state;
- missing randomization state during recovery;
- attempted re-roll after activation;
- client exposure of unrevealed future positions;
- software/randomization-version change during replay/recovery.

## Canonical invariants

- `RAND-001` One active `questionId` has at most one canonical derived gameplay-randomization result per initialization revision.
- `RAND-002` Reconnect/replay/restart cannot change the reveal order.
- `RAND-003` Duplicate reveal delivery cannot consume a second random choice or reveal a different position.
- `RAND-004` Client-controlled data cannot become gameplay-randomness authority.
- `RAND-005` Future hidden reveal positions are not disclosed before becoming authoritative visible state.
- `RAND-006` Active-question recovery with missing/corrupt randomization state fails closed rather than re-rolling.
- `RAND-007` Test determinism mechanisms are not reachable from production client authority.
- `RAND-008` Randomization result is immutable after question activation.

These complement the stable `INV-*` registry, especially the invariant that random reveal order is immutable per question instance.

## Verification

Acceptance requires documentation/test design proving:

- fixed-order Stage 1/2 replay;
- one-time initialization under concurrency;
- reconnect/restart order preservation;
- duplicate reveal idempotency;
- fail-closed missing-state recovery;
- future-order secrecy at the client boundary;
- deterministic Golden fixtures independent of production entropy.

## Related canonical sources

- `docs/01-game-design/classic-mode/stage-01-warmup.md`
- `docs/01-game-design/classic-mode/stage-02-steal.md`
- `docs/02-domain/domain-model.md`
- `docs/02-domain/invariants.md`
- `docs/02-domain/failure-and-replay-semantics.md`
- ADR-004 — Authoritative Timer and Latency Policy
- ADR-005 — Persistence and event/audit strategy (planned)
- GitHub Issues #13, #18, #21
