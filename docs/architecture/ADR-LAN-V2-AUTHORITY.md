# ADR — LAN v2 authority, V1 isolation and fail-closed recovery
Status: PROPOSED FOR REVIEW · Decisions accepted in [#38](https://github.com/necdetoskay/lingo/issues/38), implementation tracked in [#39](https://github.com/necdetoskay/lingo/issues/39).

## Context
V1 `docs/canonical/PRODUCT-SCOPE.md` excludes multiplayer and networking. V1 `GAMEPLAY-SPEC.md` runs continuous single-player timers without pause in background; LAN v2 pauses only on defined disconnect/storage/proof events. V1 `Round` and `LocalStore` exist; source_version is provenance, not a versioned immutable dictionary snapshot. Existing README calls project pre-implementation despite Rust code.

## Decision
1. Keep V1 specifications intact; introduce opt-in LAN mode with its own SPEC and `lingo_core::lan` reducer. Reuse `normalize`, `evaluate`, `score_for`, but not V1 `Round` as multiplayer state machine.
2. One Rust-authoritative host, one SQLite access owner; Flutter transport and display only. Authenticated same-device sessions, generation fencing, idempotent commands, strict versioned JSON.
3. Pin approved immutable SQLite dictionary snapshot to each match; approval manifest required; migrate V1 words and restrictions transactionally without changing player progress. Do not treat `source_version` as snapshot identity.
4. Commit SQLite state and independent host-local durable high-watermark/digest proof before ACK. Because the two stores cannot be committed atomically, **mismatch or uncertain state fails closed**; no automatic proof reconstruction or optimistic rollback.
5. Rust host monotonic time governs turns. After restart, monotonic epoch changes require independent trusted elapsed evidence. No proof means no automatic resume or forfeit.
6. UX-01..04 are projections of authenticated host snapshots/immutable match events; no new scoring authority in Flutter.

## Alternatives rejected
- Separate multiplayer core or SQLite service: duplicate authority and migration burden.
- Flutter-managed scores/deadlines: race, replay and tamper risks.
- Copy dictionary per match or accept client's dictionary: duplication and divergent decisions.
- Automatic proof repair after partial cross-file commit: masks rollback and violates fail-closed guarantee.
- Broadening V1 canonical scope in place: makes V1 requirements ambiguous.

## Risks and open verification
- Strict fail-closed may permanently strand matches after power loss; user-visible locked/pending state required.
- Background networking and clock continuity differ across mobile platforms.
- V1 DB rows lack independent per-row approval evidence; manifest matching can block migration.
- JSON canonical digest, device-secret storage and actual host recovery evidence need implementation-level security review.
- README implementation-status drift should be corrected separately after validating current release status.

## Required evidence
EP-01: reconcile historic multiplayer inventory and document precedence; EP-02: T01–T10 DB migration; EP-03/04: deterministic and security tests; EP-05: fault-injected crash-window recovery; EP-06/08: two real devices and V1 regressions. No task marked complete merely by this ADR.
