# LAN v2 — historical multiplayer compatibility audit
Status: SOURCE INVENTORY REVIEWED / NO LEGACY SPEC IMPORTED
Evidence source: `backup/main-pre-single-player-v1-2026-11-01`; discovered by comparing that ref to current `main`. All **26 historic Markdown documents** below were opened through GitHub and their sections and introductory rule contracts inspected. High-risk time, connection, identity and persistence sections were specifically cross-checked. This is a **compatibility and reuse inventory**, not a claim of line-by-line security certification or code reuse.

| Historical file | Classification | LAN v2 disposition |
| --- | --- | --- |
| `docs/00-project/project-charter.md` | Different product baseline | Historical: 2–4 player Classic Mode with 3 stages, no LAN gameplay authority |
| `docs/01-game-design/classic-mode/stage-01-warmup.md` | Incompatible gameplay | Reject stage-specific bonus/reveal/scoring |
| `docs/01-game-design/classic-mode/stage-02-steal.md` | Incompatible gameplay | Reject steals/claim windows/bonus-value rules |
| `docs/01-game-design/classic-mode/stage-03-duel.md` | Different turn policy | Do not reuse 2–4 player transfer and invalid-guess semantics |
| `docs/02-domain/README.md` | Legacy domain index | Historical only; not an active authoritative doc |
| `docs/02-domain/authoritative-randomness.md` | Reusable principle, not contract | Once-only authoritative random decisions worth retaining; new host-scope and selection tests required |
| `docs/02-domain/domain-model.md` | Different aggregate model | Do not import Stage, StealEntitlement, StealClaim; new Match/Word/Turn explicitly modelled in Rust |
| `docs/02-domain/failure-and-replay-semantics.md` | Reusable safety principle | Duplicate/out-of-order/replay fail-closed, rederive host-local protocol; legacy server recovery not inherited |
| `docs/02-domain/intent-contract.md` | Reusable safety principle | Identity/command idempotence, generation fencing, payload bounds; LAN auth is not legacy server-auth |
| `docs/02-domain/invariants.md` | Mixed | Unique score and monotonic attempt are useful; steal and legacy stage invariants rejected |
| `docs/02-domain/match-state-machine.md` | Different state machine | Separate LAN state model, no warm-up/steal/duel stages |
| `docs/02-domain/multiplayer-lifecycle.md` | Direct conflict | **Legacy competitive timers continue during disconnect**; LAN v2 approved decision **pauses** match and attempt timer with 60 s reconnect |
| `docs/03-word-platform/word-platform.md` | Partly aligned data separation | Guess vs answer vs restricted roles consistent; LAN needs immutable *approved* SQLite version/digest |
| `docs/03-word-platform/word-validation-policy.md` | Different invalid-input policies | Legacy per-stage dictionary-invalid penalties cannot overwrite LAN: invalid guess consumes neither attempt nor turn |
| `docs/04-architecture/architecture-principles.md` | Partially aligned authority | Deterministic authoritative state useful; central online server replaced by host Rust on LAN |
| `docs/04-architecture/competitive-security-and-abuse-model.md` | Reusable threat categories | Host authentication, replay, tampering, input bounds to be separately revalidated; online trust topology differs |
| `docs/04-architecture/observability-privacy-audit-contract.md` | Reusable audit/privacy guidance | Local minimum data, redaction and event correlation; no presumption of cloud telemetry |
| `docs/05-ai-ml/on-device-word-ai.md` | Out of scope | No runtime AI validation, offline approved dictionary only |
| `docs/06-testing/test-strategy.md` | Reusable method | Fake clock/property/transition tests useful; Stage 1/2/3 golden tests irrelevant |
| `docs/07-delivery/aegis-mur-hardening-plan.md` | Historical release gate | Security/reliability categories can inform v2 acceptance; cannot reuse prior PoC PASS |
| `docs/07-delivery/implementation-readiness.md` | Obsolete baseline | Previous Classic Mode readiness does not certify new LAN EPIC |
| `docs/adr/ADR-004-authoritative-timer-and-latency-policy.md` | Mixed, direct conflict | Server monotonic deadline/authoritative sequencing principles useful; **'reconnect never extends deadline'** conflicts with LAN pause; write LAN-specific policy |
| `docs/adr/ADR-005-persistence-snapshot-event-audit-strategy.md` | Reusable atomicity principles only | Materialized recovery, idempotency and audited awards useful; independent external high-watermark proof is **new** and cross-file atomicity unresolved |
| `docs/adr/ADR-007-poc-identity-authentication-model.md` | Reusable guest/session principle | High-entropy credential, rotation, one active session useful; requires LAN *same-device* proof and host-local secure storage |
| `docs/adr/ADR-008-game-experience-event-system.md` | Compatible projection pattern | Presentation events not authoritative; UX-01..04 may project host events without new persistent game engine |
| `docs/adr/README.md` | Obsolete index | ADR numbers and statuses were for old Classic Mode and must not be inherited |

## Blocking/important differences
1. **Disconnect deadlines:** old `multiplayer-lifecycle.md` and old ADR-004 explicitly say that active competitive timers continue and reconnect does not extend; LAN v2 explicitly pauses on disconnect and storage/proof errors. Prior texts are **not** normative.
2. **Game model:** old 2–4 player server-based staged mode is distinct from 2-player 10-word shared-board LAN. Stage/steal/final bonuses must not leak into Rust reducer.
3. **Authority:** host-local Rust/SQLite with external proof has a distinct crash window from centralized server optimistic revision/persistence. Old ADR-005 does not prove recovery safety here.
4. **Identity:** LAN same-device reconnect cannot rely on display name, local IP/MAC, or old online session assumptions.
5. **Dictionary/tile policy:** old stage-specific word-validation rules do not supersede current V1 canonical dictionary policy, nor the separately accepted current product decisions. Ensure repeat-yellow semantics are reconciled before reusing `lingo_core::evaluate` in LAN; see EPIC #39.
6. **Experience event separation:** useful pattern for last-5-second feedback and match events, but no second authority or persistent per-tick writes.

## Release gate
- [x] 26 historical files enumerated and **source headings / rule introductions reviewed**; specific high-risk decisions checked.
- [x] Legacy-vs-v2 direct conflicts classified; no old file copied into current canonical directory.
- [ ] Formal Rust/Flutter implementation/ADR review of each reused concept, including threat model and proof assumptions.
- [ ] V1 repeated-letter rule conflict resolved in canonical spec, core and tests.
- [ ] Host clock/recovery and cross-file proof contract verified in executable tests.
Historical documents are **source material only**. Their historical 'Accepted' and 'LOCKED' tags cannot mark LAN v2 EPIC tasks complete.
