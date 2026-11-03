# Test Strategy

## Rust core

Unit and state-machine tests cover Turkish normalization, dictionary acceptance, repeated present letters with unbounded yellow (green priority), first-letter enforcement, persistent revealed-letter hints with flexible later guesses, five-attempt scoring, idempotent terminal actions, concurrent guess/expiry ordering, and background reconciliation.

## Data pipeline

Validate field completeness, 4/5/6-letter length, normalization, duplicate entries, answer definitions, licence/provenance, and separation of answer versus guess roles. The 150-word pilot receives manual review.

## Flutter and Android

Widget/integration tests cover the required first-letter affordance, editable alternative guesses despite revealed hints, result views, muted/unmuted feedback, persistence restore, and offline use. CI runs formatting/linting as configured, Rust tests, Flutter analysis/tests, and a debug Android build after application code exists.
