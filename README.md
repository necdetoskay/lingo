# Lingo

Android-first, offline, single-player Turkish word game.

> Status: **V1 prototype source present; release acceptance not verified. LAN v2 design is a draft.**

The repository contains an Android-first Flutter application scaffold, a Rust deterministic game core, local SQLite storage, canonical V1 specifications and CI configuration. Source presence or CI results do **not** imply a verified production release. The incompatible legacy multiplayer-design baseline is preserved at `backup/main-pre-single-player-v1-2026-11-01`.

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

## Proposed LAN v2 (opt-in; not implemented)

LAN v2 is a separately scoped two-player host-authoritative shared-board mode, tracked by [EPIC #39](https://github.com/necdetoskay/lingo/issues/39) and detailed decision ledger [Issue #38](https://github.com/necdetoskay/lingo/issues/38). Its draft requirements are in `docs/canonical/LAN-V2-SPEC.md` and `docs/architecture/ADR-LAN-V2-AUTHORITY.md`.

LAN v2 does not replace V1 canonical rules. Historical 2–4-player Classic Mode documents are **not** the authority for this mode; see `docs/quality/LAN-V2-LEGACY-COMPATIBILITY.md` and the current V1/UI conflict register `docs/quality/LAN-V2-CANONICAL-DIVERGENCES.md`. Implementation and two-device acceptance have not been completed.
