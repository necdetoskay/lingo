# Classic Mode — Stage 1: Warm-up

Status: **LOCKED**  
Version: 1.1

## Change rationale — 1.1

Resolves the previous implementation-affecting dependencies for standard invalid-input behavior and the exact 10-second timer boundary by adopting the canonical Word Validation Policy and ADR-004.

## Purpose

Introduce the core guess/reveal loop with low initial pressure, then finish with a high-value single-attempt bonus.

## Standard questions

- Each player receives 3 standard questions.
- Each answer is a 4-letter Turkish word.
- The first letter is visible when the question starts.
- Each question allows up to 5 **committed valid guesses**.

## Guess reveal loop

1. Player submits input.
2. Input is normalized/classified under the canonical Word Validation Policy.
3. A dictionary-invalid standard input is rejected without consuming an attempt or starting reveal.
4. A committed valid guess is displayed immediately without result colors.
5. A reveal delay of approximately 2 seconds occurs.
6. Letter tiles reveal sequentially with a short per-tile animation.
7. Feedback colors are applied.
8. If the answer is not correct and attempts remain, the next attempt begins.

## Letter feedback

- Green: correct letter, correct position. Green letters remain revealed in subsequent attempts.
- Orange: letter exists in the answer but is in a different position. Orange letters do not auto-place in the next attempt.
- Red/neutral-wrong: letter is not part of the answer according to duplicate-letter evaluation rules.

All previous committed guesses and their revealed feedback remain visible.

## Standard-question scoring

- Correct on attempt 1: 500 points
- Correct on attempt 2: 400 points
- Correct on attempt 3: 300 points
- Correct on attempt 4: 200 points
- Correct on attempt 5: 100 points
- Not solved: 0 points

## Three-question completion bonus

If the player solves all 3 standard questions, award **+1000 points** automatically. No additional question is asked for this award.

## Standard-question invalid input

The canonical Word Validation Policy applies:

- malformed input is rejected before gameplay commitment;
- dictionary-invalid input consumes no attempt;
- no reveal is produced;
- ownership stays with the same player/question;
- the player receives a structured rejection reason.

## Stage 1 bonus question

After the 3 standard questions, the player receives one bonus question.

- Answer length: 8 letters
- First letter visible at start
- Starting value: 2500 points
- Total decision window: 10 seconds
- Exactly one committed lexical answer is allowed
- Every 2 seconds, one still-hidden letter is revealed at a random unrevealed position
- Each timed reveal reduces the available value by 200 points
- Random reveal order must be determined once for the question and must not change during replay/reconnect

Indicative values:

- Start: 2500
- After first timed reveal: 2300
- After second: 2100
- After third: 1900
- After fourth: 1700

### Authoritative timing

ADR-004 is authoritative:

- reveal milestones occur at +2s, +4s, +6s and +8s;
- an answer commitment is on time when `authoritativeReceivedAt <= deadlineAt`;
- therefore a commitment accepted at exactly 10.000s is valid;
- the PoC adds no gameplay grace window;
- commitment before a reveal milestone prevents it;
- commitment at or after a reveal milestone observes that reveal/value reduction first;
- reconnect never resets or extends the deadline.

At a deadline with no accepted commitment, the question awards 0.

## Bonus answer behavior

- Accepting the answer commitment stops the authoritative timer as part of the same state transition.
- No later letter reveal may mutate the question after commitment.
- Correct answer: award the currently available bonus value.
- Wrong answer: 0 points and question ends.
- Dictionary-invalid answer: consumes the one lexical commitment, awards 0, and ends the question.
- Malformed input is rejected before commitment while the existing deadline continues.
- No second committed lexical attempt exists.

## Maximum stage score

- 3 first-attempt standard answers: 1500
- Three-question completion bonus: 1000
- Immediate correct bonus answer: 2500
- Maximum: **5000 points**

## Acceptance-critical invariants

- Standard-question score depends only on the successful committed attempt number.
- Standard dictionary-invalid input does not consume an attempt.
- Green positions persist into later attempts.
- Orange positions do not auto-place.
- Bonus timer stops at authoritative answer commitment, not after validation completes.
- Exact deadline and reveal-boundary behavior follows ADR-004.
- Bonus letter reveal positions are random but immutable for the active question instance.
- Reconnect cannot reset timer, reveal order, or consumed answer state.

## Canonical dependencies

- `docs/03-word-platform/word-validation-policy.md`
- `docs/02-domain/invariants.md`
- ADR-004 — Authoritative Timer and Latency Policy
- Authoritative Randomness & Fairness Contract (#18) must preserve the immutable reveal-order invariant without changing this stage rule.
