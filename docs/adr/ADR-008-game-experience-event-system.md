# ADR-008 — Game Experience Event System

Status: **Proposed**  
Date: 2026-08-12  
Tracking: GitHub Issue #1

## Context

Lingo Classic Mode contains presentation-sensitive gameplay transitions that will later need audio, animation, visual feedback, haptic feedback, and potentially diagnostics/telemetry.

Examples include:

- question start and first-letter reveal,
- guess commitment and validation result,
- sequential letter feedback reveal,
- Stage 1 timed bonus-letter reveals and score decay,
- Stage 2 steal-window opening, authoritative claim acceptance, and steal-answer timeout,
- Stage 3 turn rotation and remaining-attempt transfer.

Embedding media calls directly inside gameplay/state-machine logic would couple authoritative rules to presentation behavior. It would also make future audio/animation changes risky because a missing or failing media consumer could affect gameplay execution.

Classic Mode rules in the locked stage specifications remain authoritative. This ADR does not redefine scoring, timing, validation, turn ownership, or multiplayer fairness.

## Decision

Lingo will introduce a typed **Game Experience Event System** between authoritative gameplay state transitions and presentation/feedback consumers.

```text
Authoritative gameplay transition
        |
        v
Typed Gameplay Event
        |
        v
Experience Event Bus
        |
        +--> Audio / Media
        +--> Animation
        +--> Visual Effects
        +--> Haptic
        +--> UI Feedback
        +--> Diagnostics / Telemetry (optional later)
```

### 1. Authoritative state remains the source of truth

Gameplay state machines decide what happened. Experience events are emitted only after the relevant authoritative transition has been committed.

Consumers must not mutate authoritative gameplay state.

The Experience Event Bus is not an event-sourcing store, multiplayer transport, persistence layer, or authoritative timer.

### 2. Events describe domain facts, not presentation commands

Valid event semantics describe completed or committed gameplay facts, for example:

- `QUESTION_STARTED`
- `GUESS_COMMITTED`
- `GUESS_INVALID`
- `ANSWER_CORRECT`
- `ANSWER_WRONG`
- `LETTER_REVEALED`
- `SCORE_AWARDED`
- `STEAL_WINDOW_OPENED`
- `STEAL_CLAIM_ACCEPTED`
- `TURN_CHANGED`
- `ATTEMPTS_TRANSFERRED`

Presentation commands such as `PLAY_CORRECT_SOUND`, `SHOW_CONFETTI`, or component selectors are not gameplay events.

### 3. Event contracts are typed and versionable

Events will use a discriminated typed contract with event-specific payloads.

Common correlation metadata should include, where relevant:

- `version`
- `occurredAt`
- `gameId`
- `stageId`
- `questionId`
- `playerId`
- `attemptId`
- `claimId`

Payloads contain only the minimum domain context needed by downstream consumers. Audio filenames, animation identifiers, and UI component names are excluded from gameplay event contracts.

### 4. Experience mapping is a separate layer

A mapping layer translates gameplay events into one or more presentation reactions.

Conceptually:

```text
ANSWER_CORRECT
  -> audio: answer.correct.default
  -> animation: answer.correct
  -> visual: success

LETTER_REVEALED + reason=timed_bonus
  -> audio: timer.letter_reveal

STEAL_WINDOW_OPENED
  -> audio: steal.window_open
  -> visual: steal.prompt
```

Semantic media/reaction keys are preferred over hardcoded asset paths.

### 5. Consumer failures are isolated

A missing asset, audio failure, animation failure, or consumer exception must not roll back, block, or corrupt gameplay state.

One failing consumer must not prevent other consumers from receiving the same event.

### 6. Timer semantics remain authoritative outside the experience bus

Timer events mirror authoritative timer transitions; they do not create timer authority.

Different timer purposes must remain distinguishable, especially:

- Stage 1 bonus decision timer,
- Stage 2 steal-claim timer,
- Stage 2 steal-answer timer.

Stage 1's locked rule remains unchanged: committing the bonus answer stops the timer immediately, before validation completes.

Exact deadline/boundary semantics remain governed by ADR-004.

### 7. Multiplayer fairness remains outside presentation timing

Stage 2 claim winners and deadlines are determined by server-authoritative gameplay state, never by client receipt time or experience-event delivery time.

Stage 3 turn ownership and transferred-attempt ownership are emitted only after the authoritative ownership transition is committed.

### 8. Replay/reconnect must not duplicate user feedback accidentally

The implementation must define sufficient sequencing/idempotency metadata so reconnect, replay, or state rehydration does not unintentionally replay transient presentation feedback as if it were a new gameplay transition.

The exact persistence relationship with durable domain/audit events is deferred to ADR-005.

## Initial event families

The initial registry should cover these semantic families without requiring every candidate event to survive implementation unchanged:

- Game / Stage lifecycle
- Question lifecycle
- Guess / Validation
- Reveal / Letter feedback
- Timer milestones
- Score / Bonus
- Stage 2 Steal
- Stage 3 Turn / Ownership

Final names must align with the implemented canonical state machine and avoid duplicate meanings.

## Alternatives considered

### Direct media calls from gameplay logic

Rejected because it tightly couples game rules and presentation, makes testing harder, and allows presentation failures to threaten gameplay flow.

### UI-only event emitter

Rejected as the canonical source because many important transitions are authoritative multiplayer/domain events, not UI interactions.

### Distributed broker from the start

Rejected for the initial implementation. Kafka, Redis Streams, or similar infrastructure would add complexity without solving the immediate in-process presentation-decoupling problem.

### Full event sourcing

Rejected as the purpose of this ADR. Durable domain/audit event strategy is a separate architecture concern covered by ADR-005.

## Consequences

### Positive

- Audio/media can be added without changing gameplay rules.
- Animation, haptic, and visual effects can reuse the same gameplay facts.
- Gameplay can be tested with all presentation consumers disabled.
- Media assets can be replaced or remapped without touching authoritative state machines.
- Failure isolation becomes explicit and testable.
- Multiplayer presentation can reflect authoritative results instead of client-local assumptions.

### Costs / risks

- Event taxonomy and ordering become API contracts that require discipline.
- Duplicate or overly granular events can create confusing consumers.
- Reconnect/replay semantics require explicit idempotency handling.
- Mapping configuration can become complex if mode/stage-specific overrides are allowed without clear precedence rules.

## Verification

Implementation is acceptable only when tests demonstrate at minimum:

- typed event/payload contract safety,
- deterministic publish/subscribe behavior,
- unsubscribe/lifecycle cleanup,
- consumer failure isolation,
- Stage 1 answer-commitment/timer ordering,
- Stage 2 steal-window ordering and separate claim/answer timeout semantics,
- Stage 3 turn-change and attempt-transfer semantics,
- gameplay remains fully functional with all experience consumers disabled,
- missing/broken media assets do not affect authoritative game outcomes.

## Rollback

Because the experience layer is non-authoritative, consumers can be disabled independently while preserving gameplay.

During migration, event emission may coexist temporarily with existing presentation behavior, but duplicated feedback must be prevented. Once mapped consumers are verified, direct presentation calls should be removed from gameplay/state-machine code.

## Related documents

- `docs/01-game-design/classic-mode/stage-01-warmup.md`
- `docs/01-game-design/classic-mode/stage-02-steal.md`
- `docs/01-game-design/classic-mode/stage-03-duel.md`
- `docs/04-architecture/architecture-principles.md`
- ADR-004 — Authoritative timer and latency policy
- ADR-005 — Persistence and event/audit strategy
- GitHub Issue #1 — Game Experience Event System
