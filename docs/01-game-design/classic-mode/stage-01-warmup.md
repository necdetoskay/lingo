# Classic Mode — Stage 1: Warm-up

Status: **LOCKED**  
Version: 1.0

## Purpose

Introduce the core guess/reveal loop with low initial pressure, then finish with a high-value single-attempt bonus.

## Standard questions

- Each player receives 3 standard questions.
- Each answer is a 4-letter Turkish word.
- The first letter is visible when the question starts.
- Each question allows up to 5 guesses.

## Guess reveal loop

1. Player submits a guess.
2. The submitted word is displayed immediately without result colors.
3. A reveal delay of approximately 2 seconds occurs.
4. Letter tiles reveal sequentially with a short per-tile animation.
5. Feedback colors are applied.
6. If the answer is not correct and attempts remain, the next attempt begins.

## Letter feedback

- Green: correct letter, correct position. Green letters remain revealed in subsequent attempts.
- Orange: letter exists in the answer but is in a different position. Orange letters do not auto-place in the next attempt.
- Red/neutral-wrong: letter is not part of the answer according to duplicate-letter evaluation rules.

All previous guesses and their revealed feedback remain visible.

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

A dictionary-invalid entry is not treated as a valid guess. Exact UX and retry semantics remain subject to the Word Validation specification; the one-attempt failure rule below applies specifically to the stage bonus question.

## Stage 1 bonus question

After the 3 standard questions, the player receives one bonus question.

- Answer length: 8 letters
- First letter visible at start
- Starting value: 2500 points
- Total decision window: 10 seconds
- Exactly one submitted answer is allowed
- Every 2 seconds, one still-hidden letter is revealed at a random unrevealed position
- Each timed reveal reduces the available value by 200 points
- Random reveal order must be determined once for the question and must not change during replay/reconnect

Indicative values:

- Start: 2500
- After first timed reveal: 2300
- After second: 2100
- After third: 1900
- After fourth: 1700

At the 10-second deadline, an unanswered question awards 0. The exact boundary behavior at exactly 10.000 seconds must be defined in the timer ADR before implementation.

## Bonus answer behavior

- Submitting an answer stops the timer immediately.
- No further letter may reveal while the answer is being evaluated.
- Correct answer: award the currently available bonus value.
- Wrong answer: 0 points and question ends.
- Dictionary-invalid answer: 0 points and question ends.
- No second attempt exists.

## Maximum stage score

- 3 first-attempt standard answers: 1500
- Three-question completion bonus: 1000
- Immediate correct bonus answer: 2500
- Maximum: **5000 points**

## Acceptance-critical invariants

- Standard-question score depends only on the successful attempt number.
- Green positions persist into later attempts.
- Orange positions do not auto-place.
- Bonus timer stops at answer commitment, not after validation completes.
- Bonus letter reveal positions are random but deterministic for the active question instance.
