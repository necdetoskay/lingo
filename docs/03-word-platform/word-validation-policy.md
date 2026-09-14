# Word Validation Policy

Status: **Canonical / Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #3

## Purpose

Define the deterministic boundary between malformed input, dictionary-invalid guesses, valid wrong guesses, and correct answers, and specify how each Classic Mode stage reacts.

This document closes the gameplay-affecting invalid-input ambiguity needed by Stage 1/2/3. Dictionary source/licensing, dataset schema finalization, and dataset build lifecycle remain owned by Issue #3 until separately completed.

## Classification pipeline

Every submitted text is processed in this order:

1. transport/schema validation;
2. Turkish normalization;
3. structural validation;
4. Guess Dictionary lookup against the match-pinned effective word-policy version;
5. answer comparison.

The result is one of:

- `MALFORMED_INPUT`
- `DICTIONARY_INVALID`
- `VALID_WRONG`
- `VALID_CORRECT`

## Turkish normalization

- Locale-aware Turkish casing is mandatory (`I/ı`, `İ/i`).
- Unicode normalization is deterministic and consistent across client/server tooling.
- Leading/trailing whitespace is removed before structural validation.
- Internal whitespace and punctuation are rejected unless a future version explicitly permits them.
- The authoritative server performs its own normalization and never trusts a client-normalized token as lexical authority.

## Malformed input

Examples include:

- empty input after normalization;
- wrong required letter length;
- disallowed whitespace/punctuation;
- invalid character class;
- malformed payload/encoding.

A malformed input is **not a committed gameplay guess**.

Therefore it:

- consumes no attempt;
- awards no score;
- opens no steal window;
- causes no letter reveal;
- does not transfer ownership;
- returns a structured rejection reason;
- remains subject to abuse/rate limits.

Malformed-input retry does not create an unlimited authority bypass because the authoritative stage deadline, where one exists, continues unchanged.

## Dictionary-invalid input

A structurally valid normalized word that is not accepted by the match-pinned Guess Dictionary is `DICTIONARY_INVALID`.

Its gameplay effect is stage-specific and is part of the stage contract below.

## Stage policy matrix

### Stage 1 — standard question

`DICTIONARY_INVALID`:

- does not consume an attempt;
- produces no reveal feedback;
- keeps the same player/question ownership;
- returns a reason code so the player understands the rejection.

The player may retry while the question remains active.

### Stage 1 — bonus question

The bonus allows exactly one **committed lexical answer**.

- `MALFORMED_INPUT`: rejected before commitment; timer continues.
- `DICTIONARY_INVALID`: consumes the one bonus answer commitment, stops the timer, awards 0, and ends the question.
- `VALID_WRONG`: awards 0 and ends the question.
- `VALID_CORRECT`: awards the currently available authoritative value.

### Stage 2 — standard primary question

`DICTIONARY_INVALID`:

- does not consume a normal attempt;
- does not open `STEAL_WINDOW`;
- produces no ordinary reveal sequence;
- leaves ownership with the primary player.

Only a **valid wrong** committed guess opens the steal flow.

### Stage 2 — bonus primary answer

The primary player has exactly one committed lexical answer.

- `MALFORMED_INPUT`: rejected before commitment; decision timer continues.
- `DICTIONARY_INVALID`: consumes the primary direct answer, freezes the current authoritative bonus value under ADR-004, awards 0 to the primary player, then opens the steal chain for eligible opponents.
- `VALID_WRONG`: same steal-chain transition as above.
- `VALID_CORRECT`: awards the current value and ends the bonus question.

### Stage 2 — steal answer

An accepted claimant owns one answer entitlement.

- `MALFORMED_INPUT`: rejected before lexical commitment while the existing 3-second deadline continues.
- `DICTIONARY_INVALID`: consumes that claimant's steal entitlement and is treated as a failed steal answer.
- `VALID_WRONG`: consumes the entitlement and fails.
- `VALID_CORRECT`: awards the applicable score/value and ends the question.

If eligible opponents remain after a failed steal, the normal fresh claim-window rule applies.

### Stage 3 — Duel

Stage 3 deliberately applies a stronger dictionary-invalid penalty.

- `MALFORMED_INPUT`: rejected before commitment; consumes no shared attempt and changes no ownership.
- `DICTIONARY_INVALID` by the current rotational owner: consumes the current shared attempt, then all **remaining** shared attempts transfer to the next player in canonical roster order; that receiver becomes exclusive owner.
- `DICTIONARY_INVALID` by the exclusive transferred-attempt owner: consumes the current remaining attempt and immediately terminates the question; any unused remainder is discarded.
- `VALID_WRONG`: follows the normal Stage 3 rotation/exclusive-owner rules.
- `VALID_CORRECT`: awards the fixed 3000 points to the actual solver and terminates the question.

## Reason codes

The implementation must expose stable machine-readable reason codes distinct from localized UI text. Minimum families:

- `INPUT_EMPTY`
- `INPUT_LENGTH_INVALID`
- `INPUT_CHARACTERS_INVALID`
- `WORD_NOT_IN_GUESS_DICTIONARY`
- `WORD_POLICY_VERSION_MISMATCH`
- `GUESS_VALID_WRONG`
- `GUESS_CORRECT`

Additional lexical exclusion reasons may be introduced as the dataset policy is finalized, but they must remain versioned and auditable.

## Answer eligibility

Guess validity and answer eligibility remain separate:

- Guess Dictionary answers whether a submitted token may be used as a guess.
- Answer Pool determines whether a word may be selected as a puzzle answer.

A word may be guess-valid without being answer-eligible.

## Invariants

- Input classification is deterministic for one effective word-policy version.
- A malformed input cannot mutate gameplay state.
- Stage 1/2 standard dictionary-invalid input cannot consume an attempt.
- Stage 2 standard dictionary-invalid input cannot open a steal window.
- Stage 3 dictionary-invalid behavior follows its explicit penalty/transfer semantics.
- Dataset/policy version cannot change mid-match.

## Verification

Golden tests must include:

- Turkish casing and Unicode normalization;
- malformed vs dictionary-invalid distinction;
- Stage 1 standard no-attempt invalid retry;
- Stage 1 bonus invalid one-shot failure;
- Stage 2 standard invalid not opening steal;
- Stage 2 bonus invalid opening steal after freezing value;
- Stage 2 steal invalid consuming entitlement;
- Stage 3 first invalid consuming one attempt and transferring the remainder;
- Stage 3 second invalid after transfer terminating the question;
- word-policy version mismatch fail-closed behavior.

## Related canonical sources

- `docs/03-word-platform/word-platform.md`
- `docs/01-game-design/classic-mode/stage-01-warmup.md`
- `docs/01-game-design/classic-mode/stage-02-steal.md`
- `docs/01-game-design/classic-mode/stage-03-duel.md`
- `docs/02-domain/invariants.md`
- ADR-004
- GitHub Issues #3, #13, #16, #21
