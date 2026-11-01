# Test Strategy

## Rust core

Unit and state-machine tests cover Turkish normalization, dictionary acceptance, duplicate letters, locked positions, five-attempt scoring, idempotent terminal actions, concurrent guess/expiry ordering, and background reconciliation.

## Data pipeline

Validate field completeness, 4/5/6-letter length, normalization, duplicate entries, answer definitions, licence/provenance, and separation of answer versus guess roles. The 150-word pilot receives manual review.

## Flutter and Android

Widget/integration tests cover editable versus locked cells, result views, muted/unmuted feedback, persistence restore, and offline use. CI runs formatting/linting as configured, Rust tests, Flutter analysis/tests, and a debug Android build after application code exists.
