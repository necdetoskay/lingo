# Lingo

Android-first, offline, single-player Turkish word game.

> Status: **Pre-implementation — canonical SPEC v1.0**

The repository contains the agreed product rules and development gates before application code is introduced. The legacy multiplayer-design baseline is preserved at `backup/main-pre-single-player-v1-2026-11-01`.

## V1 scope

- One player, continuous play, offline-first
- Turkish 4-, 5-, and 6-letter words
- Five attempts per word; 20 real-time seconds per attempt
- First letter visible; correctly placed letters stay locked
- No skip; sound effects enabled by default and configurable
- Local dictionary and progress storage

No accounts, server, cloud sync, leaderboard, advertising, purchases, or multiplayer are in V1.

## Canonical documents

- `docs/canonical/GAMEPLAY-SPEC.md`
- `docs/canonical/PRODUCT-SCOPE.md`
- `docs/canonical/WORD-DICTIONARY-SPEC.md`
- `docs/canonical/SCORING-AND-ACHIEVEMENTS.md`
- `docs/architecture/`
- `docs/quality/`

When implementation and canonical documentation disagree, canonical documentation wins. Changes to rules require the relevant document and acceptance criteria to be updated first.
