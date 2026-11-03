# Acceptance Criteria

Before V1 is considered playable:

- A full 4-, 5-, and 6-letter round works offline.
- The first letter is fixed in input; later green positions remain visible hints but may be replaced in subsequent guesses.
- Duplicate-letter feedback follows green-first, presence-based yellow with no count limit (ALEM/AAAA => green/yellow/yellow/yellow).
- Invalid guesses preserve attempt count while their timer continues.
- Each timeout consumes exactly one attempt, including background restore.
- Five expired or failed attempts reveal the word, reset the streak, and award zero.
- Scores are exactly 1000/900/800/700/600 by solved attempt.
- Progress, preferences, badges, and a pending round restore consistently.
- Sound defaults to on and can be disabled without affecting rules.
- The bundled pilot dictionary passes licence/provenance and definition checks.
- Rust unit tests, Flutter static analysis, and an Android build pass in CI.
