# Classic Mode — Stage 2: Steal

Status: **LOCKED**  
Version: 1.0

## Purpose

Increase risk and player-to-player interaction. A wrong answer can create a scoring opportunity for opponents.

## Standard questions

- Each player receives 3 standard questions.
- Each answer is a 5-letter Turkish word.
- First letter is visible at question start.
- The ordinary guess/reveal feedback uses the same green/orange/wrong semantics as Stage 1.

## Primary-player guess

The active question initially belongs to its primary player.

If the primary player answers correctly, the question ends and normal Stage 2 scoring for that question is applied.

If the primary player submits a wrong valid guess:

1. The ordinary reveal animation completes.
2. After the reveal, the question enters `STEAL_WINDOW`.
3. Eligible opponents receive an active `Tahmin Et` / Guess button.
4. The steal-claim window remains open for 5 seconds.
5. The first eligible player whose claim is authoritatively accepted receives the steal attempt.
6. All other claim buttons lock for that steal attempt.

## Steal attempt

- The stealing player has 3 seconds after winning the claim to submit the answer.
- Each opponent may use at most one steal attempt for the same question.
- Correct steal answer: stealing player receives **1000 points** and the question ends.
- Wrong steal answer: 0 points for that attempt; that player becomes ineligible for further steal attempts on that question.
- Timeout after claiming: treated as a failed steal attempt; player becomes ineligible for that question.

If unused eligible opponents remain after a failed steal attempt, a fresh 5-second steal-claim window opens for those players.

If nobody claims during a 5-second window, the question ends and play advances to the primary player's next question.

If all eligible opponents have used their attempt without solving, the question ends.

The primary player does not regain the question after the steal chain begins.

## Multiplayer fairness

The server must be authoritative for:

- 5-second claim-window start and end
- first accepted claim
- 3-second answer deadline
- player eligibility
- scoring

Client receipt time must not determine the winner of a simultaneous claim.

## Stage 2 bonus question

After the stage, each player receives one bonus question.

- Answer length: 10 letters
- First letter visible at start
- Starting value: **4000 points**
- Initial decision window: 10 seconds
- Primary player has one direct answer attempt
- Every 2 seconds one random hidden position is revealed
- Each timed reveal reduces the available value by **250 points**
- Reveal order is random but fixed for that question instance

Indicative value schedule:

- Start: 4000
- 2s: 3750
- 4s: 3500
- 6s: 3250
- 8s: 3000

Exact behavior on the 10-second boundary remains governed by the timer ADR.

## Bonus stealing

If the primary player submits a wrong answer, the bonus question does not immediately end.

- Eligible opponents enter the same 5-second claim-window pattern.
- First accepted claimant receives 3 seconds to answer.
- Correct answer: that player receives the currently available bonus value and the question ends.
- Wrong answer/claim timeout: that player receives 0 and becomes ineligible for the question.
- If unused opponents remain, a new 5-second claim window opens.
- If nobody claims, all eligible players are exhausted, or nobody answers correctly, the bonus question ends with no award.

The bonus value at the moment the primary player commits an answer must be captured by the authoritative game state. Whether value continues decreasing during the steal chain is **TBD** and must be explicitly decided before implementation.

## Acceptance-critical invariants

- A steal claim can have only one winner.
- A player cannot steal twice on the same question.
- Claim timeout and answer timeout are distinct timers.
- Steal points always belong to the player who supplies the correct steal answer.
- Player disconnect/reconnect must not create a second steal entitlement.
