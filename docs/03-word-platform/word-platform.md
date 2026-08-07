# Word Platform

Status: Canonical foundation

## Purpose

The Word Platform separates three concerns that must never be conflated:

1. Is an input a valid Turkish guess?
2. Is a valid word suitable to become a game answer?
3. How difficult, common, safe, or interesting is that word for a specific game context?

## Core datasets

### Guess Dictionary

Large acceptance dictionary used to validate player guesses.

A word can be valid for guessing without being eligible as an answer.

### Answer Pool

Curated subset of words that may be selected as actual puzzle answers.

Answer eligibility is stricter than guess validity.

### Restricted / Excluded entries

Explicit exclusions may include:

- proper names
- disallowed foreign-language forms
- forms without standalone lexical meaning
- inflected forms that violate game policy
- abbreviations
- offensive or family-inappropriate entries
- malformed entries

The exact linguistic policy must be versioned and auditable.

## Data-source policy

TDK may be used as a linguistic reference subject to source/licensing/usage verification. The runtime game must not depend on scraping a public web page for every guess.

The preferred runtime design is a versioned local word dataset, initially SQLite-compatible, with update metadata.

## Initial word record

Suggested canonical fields:

- `id`
- `surface`
- `normalized`
- `length`
- `valid_guess`
- `answer_candidate`
- `part_of_speech`
- `proper_noun`
- `inflected_form`
- `foreign_usage`
- `standalone_meaning`
- `family_safe`
- `difficulty`
- `usage_frequency_band`
- `source`
- `source_version`
- `classification_confidence`
- `review_status`

Runtime statistics should be stored separately from lexical truth, e.g. solve rate, average successful attempt, response time, and last-used time.

## Normalization

Turkish casing rules are mandatory. Validation must correctly handle `I/ı` and `İ/i` and must not rely on English/default-locale lowercasing.

Whitespace and punctuation policies must be deterministic.

## Duplicate-letter evaluation

Green/orange/wrong feedback must use count-aware matching so a guessed duplicate cannot receive orange/green credit more times than that letter occurs in the answer.

Recommended deterministic order:

1. Mark exact-position matches green and consume those answer-letter occurrences.
2. Evaluate remaining guessed letters against remaining unconsumed answer-letter counts.
3. Mark available occurrences orange.
4. Mark the rest wrong.

## AI classification roadmap

AI-generated labels are metadata proposals, not unquestionable lexical truth.

Recommended pipeline:

1. Import source records.
2. Derive deterministic fields in code.
3. Classify subjective fields with an LLM or on-device model.
4. Attach confidence and model version.
5. Route low-confidence or conflicting records to human review.
6. Build a gold dataset from reviewed examples.
7. Train and benchmark a small specialized classifier.

## Versioning

Every shipped word dataset must have a dataset version. Competitive multiplayer sessions must use the same effective word-policy version for all players.
