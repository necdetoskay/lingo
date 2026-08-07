# Classic Mode — Stage 3: Duel

Status: **LOCKED WITH ONE MULTIPLAYER INTERPRETATION NOTE**  
Version: 1.0

## Purpose

Create the highest-pressure pre-final stage by making players share the same question state and alternate attempts.

## Questions

- Answer length: 6 letters
- Total questions in the stage: 4
- Questions are assigned in player turn order; the designated player is the starting player for that question
- Each question is worth a fixed **3000 points**
- The value never decreases
- The first letter is visible at question start
- The question has **5 total guess attempts**, shared by the question rather than granted separately to each player

## Turn flow

The designated starting player takes attempt 1.

If the answer is wrong but the guess is valid:

1. The result is revealed using the shared letter-feedback board.
2. Ownership of the next attempt moves to the next eligible player in turn order.
3. Players continue rotating through the shared 5-attempt budget until the answer is solved or attempts are exhausted.

For a two-player game the sequence is therefore:

`A -> B -> A -> B -> A`

For 3-4 players the canonical implementation should apply the same rule as a circular player order:

`starting player -> next player -> next player -> ...`

This interpretation preserves the agreed "next player receives the next attempt" rule while supporting Classic Mode's 2-4 player requirement.

## Shared board

All players see:

- all previous guesses
- letter feedback for every guess
- which player submitted each guess
- the currently active player
- attempts remaining

Green/orange/wrong feedback follows the same canonical evaluation semantics used by previous stages.

## Correct answer

The player who submits the correct answer receives the full **3000 points**, regardless of:

- who the original question was assigned to
- which attempt solved it
- how many attempts remain

The question then ends immediately.

## Critical invalid-guess rule

An invalid guess is deliberately more severe in Stage 3.

If the player whose turn it is submits a dictionary-invalid word or otherwise commits an input that qualifies as an invalid guess under the Word Validation specification:

1. That player's rotational ownership ends for the question.
2. **All remaining guess attempts transfer to the next eligible player.**
3. The receiving player becomes the exclusive owner of the remaining attempts for that question.
4. If that receiving player then commits an invalid guess, the question ends immediately with no further transfer.

A normal valid-but-wrong guess by the receiving player does **not** end the question; the player may continue using the transferred remaining attempts until solved or exhausted.

This transfer rule is intentionally stronger than normal rotation and must be represented explicitly in the state machine.

## Question end conditions

A question ends when any of the following occurs:

- correct answer is submitted
- all 5 attempts are consumed
- the player holding transferred remaining attempts commits an invalid guess

If no correct answer exists at termination, no player receives the 3000 points and the correct answer is revealed.

## Acceptance-critical invariants

- Exactly 5 total attempts exist per question.
- 3000 points are fixed and never decay.
- A valid wrong guess rotates normal ownership.
- A first invalid guess transfers all remaining attempts to the next eligible player.
- After transfer, a second invalid guess terminates the question.
- Shared feedback from earlier guesses remains visible after ownership changes.
- Correct-answer points always go to the actual solver.
