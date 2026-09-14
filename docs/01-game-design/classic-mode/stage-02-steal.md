# Classic Mode — Stage 2: Steal

Status: **LOCKED**  
Version: 1.1

## Change rationale — 1.1

Resolves the previous `TBD` around bonus-value decay during the steal chain, adopts ADR-004 for exact timer/deadline semantics, adopts the canonical Word Validation Policy, and aligns reconnect behavior with the canonical Multiplayer Lifecycle.

## Purpose

Increase risk and player-to-player interaction. A wrong answer can create a scoring opportunity for opponents.

## Standard questions

- Each player receives 3 standard questions.
- Each answer is a 5-letter Turkish word.
- First letter is visible at question start.
- The ordinary guess/reveal feedback uses the same green/orange/wrong semantics as Stage 1.

## Primary-player guess

The active question initially belongs to its primary player.

A malformed input is rejected before gameplay commitment. A dictionary-invalid standard input consumes no attempt, produces no reveal, and does **not** open a steal window.

If the primary player answers correctly, the question ends and normal Stage 2 scoring for that question is applied.

If the primary player submits a **valid wrong** guess:

1. The ordinary reveal animation completes.
2. After the reveal, the question enters `STEAL_WINDOW`.
3. Eligible opponents receive an active `Tahmin Et` / Guess button.
4. The steal-claim window remains open for 5 seconds.
5. The first eligible player whose claim is authoritatively accepted receives the steal attempt.
6. All other claim buttons lock for that steal attempt.

## Steal attempt

- The stealing player has 3 seconds after winning the claim to submit an answer.
- Each opponent may use at most one steal attempt for the same question.
- Malformed input does not consume the entitlement, but the existing 3-second deadline continues.
- Dictionary-invalid answer consumes the entitlement and is a failed steal.
- Correct steal answer: stealing player receives **1000 points** and the question ends.
- Wrong steal answer: 0 points for that attempt; that player becomes ineligible for further steal attempts on that question.
- Timeout after claiming: treated as a failed steal attempt; player becomes ineligible for that question.

If unused eligible opponents remain after a failed steal attempt, a fresh 5-second steal-claim window opens for those players.

If nobody claims during a 5-second window, the question ends and play advances to the primary player's next question.

If all eligible opponents have used their attempt without solving, the question ends.

The primary player does not regain the question after the steal chain begins.

## Multiplayer fairness

The server is authoritative for:

- 5-second claim-window start and end;
- first accepted claim;
- 3-second answer deadline;
- player eligibility;
- scoring;
- timer identity/revision;
- claim/action idempotency.

ADR-004 defines the exact boundary: an intent accepted by the authoritative match sequencer at `authoritativeReceivedAt <= deadlineAt` is on time. The PoC adds no gameplay grace window.

Client receipt/render time and client-supplied timestamps never determine the winner of a simultaneous claim.

If two accepted claims have the same authoritative timestamp, authoritative match sequence/order is the deterministic tie-break.

Reconnect follows the canonical Multiplayer Lifecycle and never resets a claim/answer timer or restores a consumed entitlement.

## Stage 2 bonus question

After the stage, each player receives one bonus question.

- Answer length: 10 letters
- First letter visible at start
- Starting value: **4000 points**
- Initial decision window: 10 seconds
- Primary player has one committed lexical answer attempt
- Every 2 seconds one random hidden position is revealed
- Each timed reveal reduces the available value by **250 points**
- Reveal order is random but fixed for that question instance

Indicative value schedule:

- Start: 4000
- 2s: 3750
- 4s: 3500
- 6s: 3250
- 8s: 3000

ADR-004 governs the exact 10-second and reveal-milestone boundaries.

## Primary bonus answer

- Malformed input is rejected before commitment while the decision deadline continues.
- Valid correct answer awards the current authoritative value and ends the bonus question.
- Valid wrong answer consumes the primary direct answer and opens the steal chain.
- Dictionary-invalid answer also consumes the primary direct answer, awards 0 to the primary player, and opens the steal chain.

## Bonus-value freeze — canonical decision

When the primary player's committed lexical answer is authoritatively accepted, the then-current bonus value is captured as `frozenStealValue`.

**The value does not continue decreasing during the steal chain.**

All later claim and answer windows use that frozen value.

If the primary commitment occurs exactly at a timed reveal/value milestone, ADR-004 applies the due reveal/value reduction first and then freezes the resulting value.

This removes the previous `TBD` and prevents stealers from losing points solely because the claim chain or network path consumed time outside their control.

## Bonus stealing

If the primary player's committed answer is wrong or dictionary-invalid, the bonus question does not immediately end.

- Eligible opponents enter the same 5-second claim-window pattern.
- First accepted claimant receives 3 seconds to answer.
- Correct answer: that player receives `frozenStealValue` and the question ends.
- Dictionary-invalid, valid-wrong, or answer-timeout consumes that player's entitlement, awards 0, and makes the player ineligible for the question.
- Malformed input does not consume entitlement but does not extend the 3-second deadline.
- If unused opponents remain, a new 5-second claim window opens.
- If nobody claims, all eligible players are exhausted, or nobody answers correctly, the bonus question ends with no award.

## Acceptance-critical invariants

- A steal claim can have only one winner.
- A player cannot steal twice on the same question.
- Standard dictionary-invalid primary input does not open a steal window.
- Claim timeout and answer timeout are distinct timers.
- Exact timer boundaries follow ADR-004.
- Steal points always belong to the player who supplies the correct steal answer.
- Player disconnect/reconnect must not create a second steal entitlement.
- Bonus value freezes exactly once at the primary committed answer and never decays during the steal chain.
- Replay/duplicate timeout/claim delivery cannot create duplicate eligibility, timer, or score state.

## Canonical dependencies

- `docs/03-word-platform/word-validation-policy.md`
- `docs/02-domain/multiplayer-lifecycle.md`
- `docs/02-domain/invariants.md`
- ADR-004 — Authoritative Timer and Latency Policy
- Authoritative Randomness & Fairness Contract (#18) must preserve fixed reveal order without changing this stage rule.
