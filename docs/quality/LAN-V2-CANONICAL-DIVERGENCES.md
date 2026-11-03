# LAN v2 — canonical divergence and resolution register
Status: OPEN / architecture review required · Source of authority: accepted product decisions in [#38](https://github.com/necdetoskay/lingo/issues/38), EPIC [#39](https://github.com/necdetoskay/lingo/issues/39), active V1 canonical docs/code, and the local 2026-11-03 **DRAFT** `LINGO_UI_GAME_SPEC_v1.0.md`. The local document is a draft; its proposals are not automatically GitHub canonical decisions. Distinguish its explicitly user-approved rules from proposed behavior.

| ID | Evidence of divergence | Decision / implementation gate |
| --- | --- | --- |
| DIV-01 CRITICAL | V1 `GAMEPLAY-SPEC.md`: two-pass count-limited duplicate-letter yellow. `lingo_core::evaluate` matches it. 2026-11-03 UI draft §2.6 and approved gameplay intent: a wrong-position letter present anywhere in target can be yellow in **each** repeat, irrespective of counts. #38 calls for V1 rule reuse without acknowledging conflict. | **BLOCKED** until active V1 canonical document, Rust algorithm, fixtures and LAN reuse are reconciled in one explicit versioned change. Don't change core behavior just via Flutter color. Test target=ALEM, guess=AAAA: first A green and remaining wrong-position As yellow under intended new policy. |
| DIV-02 MAJOR | V1 `GAMEPLAY-SPEC.md`: green letters other than first remain visible as hints but guesses may overwrite them; `Round::submit_guess` implements it. 2026-11-03 UI draft §2.1 and §5: previously green positions are non-editable/fixed on next input row. | **BLOCKED** for shared-board input lock semantics. Explicitly decide V1/LAN scope and enforce in both Rust validation and Flutter input if necessary; note whether first-letter-only validation still applies. |
| DIV-03 MAJOR | Current V1 `Round` constant 20s and `score_for` fixed 1000..600; 2026-11-03 UI draft §§1,3,4 requests configurable time and per-attempt scores, snapshotted per round. #38 LAN fixes 20s/1000..600 and explicitly leaves configurable LAN scoring undecided. | Keep LAN v2 **fixed official match rules** until a separate explicit LAN settings decision. Personal V1 settings implementation and migration are a separate tracked requirement; do not silently inherit custom values into competitive LAN. |
| DIV-04 MAJOR | V1 `GAMEPLAY-SPEC.md` and ADR-002: timer advances in background / when closed. 2026-11-03 UI draft §6 describes **recommended**, not finally accepted, pause of remaining time on app termination. LAN #38: active background does not pause, disconnect/storage/proof events pause. | Do **not** silently reinterpret draft recommendation as accepted. Preserve V1 canonical behavior pending explicit rule update; keep LAN-specific clock policy and test lock/background/disconnect separately. |
| DIV-05 UI | 2026-11-03 UI draft §2.7/2.10: past yellow stays on its row but is not copied to a new row; sixth row after five failures displays correct answer green. Existing V1 `Round` stores only revealed green positions; LAN #38 says 5 shared attempts with answer reveal but does not define UI presentation rows. | Add Flutter UI contract and golden/widget tests; sixth **display** row must never create a sixth authoritative attempt. Persist/reconstruct per-guess feedback for match reconnect and match stats. |
| DIV-06 PRODUCT | Current main `README.md` calls project 'Pre-implementation' though Rust core and Flutter app source exist; README is stale. | Change wording to 'V1 prototype source and CI; release not confirmed' and link opt-in LAN v2 draft; no false release declaration. |
| DIV-07 POLICY | V1 scope excludes multiplayer; LAN #38 explicitly adds new opt-in v2. Historic Classic Mode is completely different. | Keep version/scope separation; no legacy mode import and no parallel core/SQLite authority. |

## Required acceptance workflow
1. Treat DIV-01/DIV-02 as **unresolved compatibility gates** for LAN tile semantics and its reuse of V1 logic. Their resolution needs a code+SPEC+test PR with explicit rule versioning; a documentation-only EP-01 PR does not resolve them.
2. Keep V1 and LAN clock lifecycle semantics separate. Do not silently implement a draft V1 pause suggestion.
3. Match-scoped settings: deterministic effective rules snapshot, host-authoritative, validated and locked before match. For LAN v2 the currently accepted values are fixed.
4. Prioritize deterministic test fixtures and UI golden/widget tests before checking off tasks.
5. Do not label a draft proposal an accepted decision or assert that this register fixes the code.

## Pilot dictionary constraint measurement (EP-01, reproducible inspection)
Source: `data/pilot-candidates.tsv` on `main`, blob `5758e9e8c5a0f995050bdf3a2cd857a03d5e5320`; 150 rows with `review_state=approved`, 50 per word length. For each **target / distinct valid first guess with the same initial letter**, count remaining **non-answer exploratory** approved guesses after green positions become mandatory. The target answer itself always remains legal; zero below means **no exploratory alternative besides the answer**, not an unwinnable round. This measurement considers only the pilot set, not a future larger guess-only lexicon.

| Letters | Target/first-guess pairs | No remaining exploratory guess if all greens locked | Rate | Same count with only first-letter requirement |
| --- | ---: | ---: | ---: | ---: |
| 4 | 266 | 38 | 14.3% | 2 |
| 5 | 174 | 50 | 28.7% | 10 |
| 6 | 160 | 54 | 33.8% | 10 |

Example: target `AİLE`, first guess `ANNE`; with both first/last correct positions locked, none of the **other approved non-answer 4-letter A-words** fits, though correct `AİLE` is still a legal answer. This measurement supports a **playability review** before hard-locking all green positions or broadening approved guess dictionary. It does not authorize overriding the user's green-lock request.
