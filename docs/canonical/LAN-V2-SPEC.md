# LAN v2 — Canonical Multiplayer Specification (Draft)
Status: DRAFT; source decision ledger: [Issue #38](https://github.com/necdetoskay/lingo/issues/38); delivery [EPIC #39](https://github.com/necdetoskay/lingo/issues/39).

## Scope and precedence
This document defines an **opt-in LAN v2** mode and does not supersede V1 single-player `PRODUCT-SCOPE.md` or `GAMEPLAY-SPEC.md`. For LAN mode, this SPEC and the LAN architecture ADR apply; for V1, existing canonical specifications apply. Disagreement between this draft and accepted Issue #38 decisions blocks implementation pending reconciliation.

## Game
Two players, one authoritative host, shared board, no remote/cloud service. LAN discovery via mDNS and joining via QR/manual room code with a resolvable endpoint. Lobby requires both ready flags, host start and 3-second countdown; departure during countdown cancels it. Room is locked on match start.

Standard match is 10 unique targets in shuffled length distribution: three 4-letter, four 5-letter and three 6-letter. Host randomly selects first starter once and persists it; starter alternates per target, including overtime. Each target has five **shared** attempts, 20 seconds per turn. Invalid word does not consume an attempt or reset time; valid incorrect word or timeout consumes one and switches player. Out-of-turn commands are rejected. Correct word scores exclusively for the submitting player: attempt 1–5 yields 1000/900/800/700/600. Both score zero for an unsolved word. First-letter lock, green hints and repeated-letter evaluation are **not yet cleared for V1 reuse**: the user-approved newer UI draft conflicts with the current V1 two-pass yellow-count and editable-green behavior. Implement no LAN guess validation until DIV-01/DIV-02 in `docs/quality/LAN-V2-CANONICAL-DIVERGENCES.md` are resolved in a versioned V1+LAN rule contract with tests. Existing `normalize` and `score_for` may be reused after their own compatibility checks. After 10 targets, if tied, continue with unused 4/5/6-letter approved targets until the lead changes; if pool exhausted, record draw_pool_exhausted.

## Match lifecycle
Normal disconnect pauses the match/turn clock, allows 60 seconds to reconnect with same authenticated device identity; two separate disconnect allowances per player per match, third is forfeit. Duplicate transport signals are one disconnect. Voluntary exit is immediate forfeit without consuming reconnect quota. Host fully offline means client result is pending_verification, never a locally certified win. Resume after restart only with trustworthy elapsed-time and persisted-proof evidence; otherwise fail closed. Rematch needs mutual agreement, fresh match identity/ready/start/countdown, new starter, and resets match-specific score/quotas.

## Dictionary
Host-approved immutable SQLite dictionary snapshot, pinned by snapshot_id and canonical digest, supplies all answers and accepted guesses; client dictionary never decides validity. Check 3/4/3 pool before starting and unique target selection. Reject unapproved, mismatched, missing or mutated snapshot. V1 dictionary/restriction migration is one SQLite transaction with approval manifest validation, rollback, idempotency and preservation of player progress, badges, length stats and pending round. See #38 T01–T10.

## Gameplay authority and network
Rust `lingo_core::lan` reducer owns game state, score, host monotonic deadlines, command validation and durable decisions. Flutter owns transport/discovery/UI, not authority. Versioned strict JSON protocol, exact protocol version match, authenticated same-device reconnect, session fencing, replay/idempotency protection, secret-filtered state snapshots. Host receives commands and commits authoritative state before ACK.

## Persistence and recovery
SQLite state and separately protected host-local high-watermark/digest proof must both be durable before ACK. Cross-file atomicity cannot be assumed. Mismatch, uncertain crash window, proof write failure or corrupted DB blocks automated recovery. Storage error pauses authoritative deadlines. A disconnected host cannot be declared forfeited by an untrusted client.

## Accepted UX features
UX-01: live two-player scores, lead, turn and remaining words; host snapshot only.
UX-02: last-five-second emphasis, optional sound/haptics, reduced-motion/mute support; UI timer is not authoritative.
UX-03: critical word/comeback/overtime messaging derived from committed scores and provable remaining maximum, no extra scoring.
UX-04: immutable-event-based match statistics (won targets, correct guesses, timeouts, best-scoring target, lead changes), separate pending_verification, draw and forfeit outcomes.
All four features are presentation/analytics; they do not change match rules or commit state on every visual tick.

## Acceptance gates
No V1 regression; deterministic fake-clock reducer tests; invalid/duplicate/out-of-order command tests; JSON protocol/security fuzz and bounds; SQLite snapshot integrity and fault-injection tests; commit/proof crash-window tests; real two-device full 10-target match, overtime, disconnect, rematch, lock/background; accessibility, latency and gameplay-balance checks. **Not implemented or tested yet.**

## Documented unresolved dependencies
- `docs/quality/LAN-V2-LEGACY-COMPATIBILITY.md` inventories 26 historic Classic Mode files and rejects their direct normative import.
- `docs/quality/LAN-V2-CANONICAL-DIVERGENCES.md` records active V1 vs local UI draft differences, including duplicate-letter yellow, persistent green input, optional settings, background time, sixth display row and README status. **DIV-01/DIV-02 block LAN tile gameplay implementation**, not documentation review.
