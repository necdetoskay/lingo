# Lingo Match State-Machine Contract

Status: **Canonical foundation**  
Version: 1.0  
Tracking: #16

## Purpose

This document defines the required shape of Lingo's authoritative state machines without inventing unresolved stage rules.

The implementation must represent competitive lifecycle as explicit states/transitions rather than scattered booleans.

## Match lifecycle

Conceptual top-level states:

```text
MATCH_CREATED
  -> LOBBY / READY_FLOW
  -> MATCH_ACTIVE
  -> MATCH_COMPLETING
  -> MATCH_COMPLETED
```

Room/ready/host/disconnect details are owned by #5. Final/winner/tie-break completion is owned by #2.

A completed Match is terminal for competitive mutation.

## Stage lifecycle

Conceptual lifecycle:

```text
STAGE_PENDING
  -> STAGE_STARTING
  -> STAGE_ACTIVE
  -> STAGE_COMPLETING
  -> STAGE_COMPLETE
```

Transition UX/acknowledgement details are owned by #12. Presentation animation never determines authoritative transition completion.

## Question lifecycle — common states

Implementations may use more specific names, but they must preserve equivalent explicit semantics:

- `QUESTION_READY`
- `QUESTION_ACTIVE`
- `GUESS_COMMITTED`
- `VALIDATING`
- `REVEALING`
- stage-specific ownership/timer substates
- `QUESTION_COMPLETE`

`QUESTION_COMPLETE` is terminal under INV-004.

## Stage 1

Minimum explicit distinctions:

### Standard question

```text
QUESTION_READY
 -> PRIMARY_GUESSING
 -> GUESS_COMMITTED
 -> VALIDATING
 -> REVEALING
 -> PRIMARY_GUESSING | QUESTION_COMPLETE
```

Attempt consumption and invalid-input retry semantics depend on #3 and must not be silently invented.

### Bonus question

Minimum explicit concepts:

- active authoritative decision deadline,
- deterministic fixed reveal order for the Question,
- answer commitment atomically prevents later timed reveals for that logical state,
- validation after commitment,
- terminal correct/wrong/invalid/timeout result.

Exact deadline boundary is owned by ADR-004 (#4). Randomness details are owned by #18.

## Stage 2

Minimum explicit distinctions:

```text
PRIMARY_GUESSING
 -> GUESS_COMMITTED
 -> VALIDATING
 -> REVEALING
 -> QUESTION_COMPLETE                     (correct)
 -> STEAL_WINDOW                          (valid wrong)

STEAL_WINDOW
 -> STEAL_CLAIMED                         (one accepted winner)
 -> QUESTION_COMPLETE                     (no claim / no eligible players)

STEAL_CLAIMED
 -> STEAL_ANSWERING
 -> QUESTION_COMPLETE                     (correct)
 -> STEAL_WINDOW                          (failed, eligible players remain)
 -> QUESTION_COMPLETE                     (failed, none remain)
```

The claim window and steal-answer window are distinct timer purposes under INV-013.

Only one accepted winner may exist per claim window under INV-012.

Bonus steal follows the same ownership/claim pattern but its value-decay semantics remain blocked by #4/#26.

## Stage 3

Minimum explicit distinctions:

```text
QUESTION_READY
 -> ROTATIONAL_OWNERSHIP

ROTATIONAL_OWNERSHIP
 -> next ROTATIONAL_OWNERSHIP             (valid wrong + attempts remain)
 -> TRANSFERRED_EXCLUSIVE_OWNERSHIP       (first invalid guess)
 -> QUESTION_COMPLETE                     (correct / attempts exhausted)

TRANSFERRED_EXCLUSIVE_OWNERSHIP
 -> TRANSFERRED_EXCLUSIVE_OWNERSHIP       (valid wrong + attempts remain)
 -> QUESTION_COMPLETE                     (correct / exhausted / second invalid)
```

The shared attempt budget is preserved under INV-017. Transfer exclusivity follows INV-018.

`next eligible player` resolution depends on #5 for disconnect/forfeit lifecycle and must be deterministic before implementation.

## Transition contract

Every mutating transition definition must identify:

- source state,
- allowed intent/system trigger,
- actor/authority,
- guards,
- state changes,
- attempt/entitlement changes,
- score effects,
- timer/deadline effects,
- randomness effects if any,
- idempotency/replay behavior,
- authoritative revision change,
- durable audit/domain facts required by #9,
- Experience Events derived after commit where applicable.

## Forbidden transitions

Every allowed transition should have corresponding forbidden-state tests where meaningful.

Examples:

- guess after `QUESTION_COMPLETE`,
- claim before `STEAL_WINDOW`,
- second accepted claim in one Stage 2 window,
- non-owner guess during transferred Stage 3 ownership,
- timed reveal after Stage 1 bonus answer commitment,
- reconnect operation directly granting an attempt or entitlement.

## System-triggered transitions

Timeouts and scheduled reveal milestones are authoritative system triggers, not trusted client intents.

They must still be idempotent and state-guarded. A delayed duplicate timeout for an already advanced/terminal state is a no-op/rejected transition, never a second completion.

## Snapshot/reconnect

Reconnect restores the latest authoritative state plus relevant deadlines/revisions. It is not a gameplay transition that grants rights.

Snapshot details and persistence are owned by #5/#9.

## Testability requirement

The state machine must be executable headlessly without UI/media consumers. Golden Game fixtures (#13) and MUR qualification (#21) should drive the same authoritative transition layer used by the runtime.
