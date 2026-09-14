# Classic Mode — Stage 3: Duel

Status: **LOCKED**  
Version: 1.1

## Change rationale — 1.1

Resolves the previous multiplayer interpretation note by making 2-4 player turn order canonical, adopts the Word Validation Policy for malformed/dictionary-invalid distinction, and aligns disconnect/reconnect behavior with the canonical Multiplayer Lifecycle.

## Purpose

Create the highest-pressure pre-final stage by making players share the same question state and alternate attempts.

## Questions

- Answer length: 6 letters
- Total questions in the stage: 4
- Questions are assigned in frozen roster turn order; the designated player is the starting player for that question
- Each question is worth a fixed **3000 points**
- The value never decreases
- The first letter is visible at question start
- The question has **5 total committed guess attempts**, shared by the question rather than granted separately to each player

## Canonical turn order

At match start the 2-4 player roster/order is frozen by the Multiplayer Lifecycle.

For each Stage 3 question:

1. the designated starting player owns the first attempt;
2. after a committed valid-wrong guess, ownership moves to the next player in circular frozen-roster order;
3. the circular order continues through the shared five-attempt budget unless the question is solved or exclusive-transfer semantics activate.

Examples:

- 2 players: `A -> B -> A -> B -> A`
- 3 players starting at B: `B -> C -> A -> B -> C`
- 4 players starting at C: `C -> D -> A -> B -> C`

`Next eligible player` means the next player in frozen roster order who remains gameplay-eligible under Stage 3 rules. Network connectivity does not silently remove a player from that order.

If the required next owner is disconnected, the Multiplayer Lifecycle governs suspension/reconnect/abandonment. Disconnect alone does not skip the player or transfer attempts.

## Shared board

All players see:

- all previous committed guesses;
- letter feedback for every committed valid guess;
- which player submitted each committed guess;
- the currently active player;
- attempts remaining.

Green/orange/wrong feedback follows the same canonical evaluation semantics used by previous stages.

## Correct answer

The player who submits the correct answer receives the full **3000 points**, regardless of:

- who the original question was assigned to;
- which attempt solved it;
- how many attempts remain.

The question then ends immediately.

## Invalid-input distinction

The Word Validation Policy distinguishes malformed input from a committed dictionary-invalid guess.

### Malformed input

Malformed input is rejected before gameplay commitment. It:

- consumes no shared attempt;
- changes no ownership;
- produces no reveal;
- does not trigger transfer.

### First committed dictionary-invalid guess

If the current rotational owner commits a dictionary-invalid word:

1. the current shared attempt is consumed;
2. that player's rotational ownership ends for the question;
3. **all remaining shared attempts transfer to the next player in canonical frozen-roster order**;
4. the receiving player becomes exclusive owner of those remaining attempts.

Example: if the first committed attempt is dictionary-invalid, 4 attempts remain and transfer to the next player.

### Dictionary-invalid guess after transfer

If the exclusive transferred-attempt owner later commits a dictionary-invalid word:

1. the current remaining attempt is consumed;
2. the question ends immediately;
3. any still-unused remainder is discarded;
4. no further transfer occurs.

A normal committed valid-wrong guess by the exclusive owner does **not** end the question; the same exclusive owner may continue consuming the transferred remainder until solved or exhausted.

This penalty is intentionally stronger than normal rotation and must be represented explicitly in the state machine.

## Question end conditions

A question ends when any of the following occurs:

- correct answer is committed;
- all 5 shared attempts are consumed;
- the player holding transferred remaining attempts commits a dictionary-invalid guess.

If no correct answer exists at termination, no player receives the 3000 points and the correct answer is revealed.

## Disconnect/reconnect semantics

The canonical Multiplayer Lifecycle applies:

- roster/order does not change on reconnect;
- reconnect does not restore an attempt;
- disconnect alone does not cause invalid-guess transfer;
- if the required owner is disconnected and progression is otherwise blocked, the match uses reconnect suspension;
- successful reconnect resumes with the same owner and remaining attempts;
- unrecovered blocking disconnect abandons the PoC match rather than silently skipping/rebalancing players.

## Acceptance-critical invariants

- Exactly 5 total committed attempts exist per question.
- 3000 points are fixed and never decay.
- A valid wrong committed guess rotates normal ownership.
- Malformed input consumes no attempt and changes no ownership.
- A first dictionary-invalid committed guess consumes the current attempt and transfers only the remainder.
- The transfer receiver has exclusive ownership of the remaining attempts.
- A dictionary-invalid committed guess after transfer consumes the current attempt and terminates the question.
- Shared feedback from earlier guesses remains visible after ownership changes.
- Correct-answer points always go to the actual solver.
- Disconnect/reconnect cannot change frozen player order or restore consumed attempts.

## Canonical dependencies

- `docs/03-word-platform/word-validation-policy.md`
- `docs/02-domain/multiplayer-lifecycle.md`
- `docs/02-domain/invariants.md`
- `docs/02-domain/match-state-machine.md`
