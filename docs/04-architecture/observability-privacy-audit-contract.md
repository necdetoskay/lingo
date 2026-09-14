# Observability, Privacy & Audit Contract

Status: **Canonical / Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #22

## Purpose

Make authoritative multiplayer decisions explainable and diagnosable without turning observability into an uncontrolled collection of user data.

This contract defines:

- structured authoritative audit fields;
- gameplay/security/operational event boundaries;
- correlation and stable reason-code rules;
- privacy/minimum-data/redaction requirements;
- PoC retention/cleanup classes;
- protections against sensitive-data leakage and log-flood abuse.

It complements ADR-005 persistence, ADR-007 identity and the Competitive Security & Abuse Threat Model.

## Core principle

**Record the minimum structured evidence required to explain and protect an authoritative decision; do not log raw user/device content merely because it is available.**

Observability is not gameplay authority. Logs/metrics/traces cannot override committed state.

## 1. Evidence classes

### A — Authoritative Domain Transition Audit

Durable evidence for committed competitive state transitions under ADR-005.

Examples:

- guess committed/evaluated;
- steal claim accepted;
- ownership changed;
- attempt/entitlement consumed;
- timer expired and transition committed;
- score awarded;
- question/stage/match completed;
- lifecycle suspend/resume/abandon transition.

This class is append-only transition evidence and correlates to authoritative revisions.

### B — Security / Decision Audit

Bounded evidence for meaningful security/integrity decisions that may not mutate gameplay.

Examples:

- wrong-player/cross-match mutation rejected;
- stale/revoked session rejected;
- stale connection generation rejected;
- idempotency conflict;
- malformed/oversized input class;
- rate/abuse control triggered;
- unauthorized snapshot/read attempt.

Hostile repeated traffic may be aggregated/sampled after a bounded threshold to prevent logging itself becoming a denial-of-service vector.

### C — Operational Diagnostics

Short-lived structured logs/traces for service health and debugging.

Examples:

- request/operation duration;
- worker/reconnect recovery timing;
- outbox retry count;
- database/transport dependency failures;
- internal exception category.

Operational diagnostics are not the durable gameplay journal.

### D — Metrics

Aggregated counters/histograms/gauges such as:

- intent accept/reject counts by reason family;
- latency distributions;
- reconnect success/failure counts;
- outbox backlog;
- error/failure rates.

Metrics should avoid unnecessary high-cardinality user identifiers.

## 2. Required correlation model

Where relevant, authoritative audit records carry the minimum applicable subset of:

- `eventId` / audit record ID;
- `occurredAt` authoritative timestamp;
- `matchId`;
- `matchSequence`;
- `revisionBefore` / `revisionAfter`;
- `stageId`;
- `questionId`;
- pseudonymous `playerId`;
- pseudonymous `playerSessionId` or security lineage reference when needed;
- `connectionGeneration` category/value when security-relevant;
- `actionId` or timer/system action identity;
- intent/transition/event type;
- stable reason/result code;
- attempt/claim/score-award identity where relevant;
- timer ID/deadline context where relevant;
- effective word-policy/dataset version;
- randomization version/reference where relevant;
- trace/correlation ID for operational linkage when needed.

Do not mechanically include every field in every record. Minimum necessary data applies.

## 3. Stable reason codes

Machine-readable reason codes are canonical and independent of localized UI text.

Examples include families already defined by domain/security contracts:

- `UNAUTHENTICATED`
- `SESSION_STALE_OR_REVOKED`
- `STALE_CONNECTION`
- `NOT_AUTHORIZED_OR_NOT_FOUND`
- `NOT_AUTHORIZED_OWNER`
- `ILLEGAL_STATE`
- `STALE_INTENT`
- `DEADLINE_EXPIRED`
- `IDEMPOTENCY_CONFLICT`
- `ALREADY_COMPLETED`
- `ENTITLEMENT_CONSUMED`
- word-validation reason codes.

Operational dashboards/tests may aggregate by reason family without needing raw request content.

## 4. Credential and secret redaction — absolute rule

The following are never written to normal gameplay/security/operational logs or audit payloads:

- access/session credentials;
- reconnect credentials/tokens;
- password or future OAuth authorization secrets;
- authorization headers/cookies;
- private keys/API keys;
- raw token hashes/verifiers when exposure would help credential attack;
- database/service connection secrets.

Redaction is applied before structured logging sinks receive the event where practical.

An exception/error object must not be blindly serialized if it can contain request headers, token values or secrets.

## 5. Microphone / speech privacy boundary

Future speech input does not change the default logging rule.

By default the system does **not** persist or log:

- raw microphone audio;
- audio buffers;
- full speech recordings;
- device microphone metadata beyond what is technically required for the feature.

If future functionality needs audio retention for an explicit product purpose, it requires a separate privacy/product decision, user-facing consent where required, retention policy and AEGIS review.

The PoC word-game audit does not need raw audio.

## 6. Guess/input content policy

### Committed lexical guesses

For deterministic gameplay audit, the system may persist the canonical normalized lexical token or a stable lexical-record/word identifier where needed to explain validation/feedback/scoring.

Preferred rule:

- when the guess maps to a known Guess Dictionary record, retain the canonical word/record identity required to reproduce the decision;
- avoid duplicating raw presentation input when normalized canonical identity is sufficient.

### Malformed/free-form rejected input

Do **not** persist arbitrary raw malformed text by default because a user may accidentally enter personal or sensitive content.

Retain bounded metadata instead, such as:

- reason code;
- normalized length bucket/character-class result where useful;
- payload fingerprint only if needed for abuse/idempotency diagnosis and designed so it is not reversible;
- action/session/match correlation IDs.

## 7. Answer and hidden-state logging

Canonical puzzle answer data may exist in authoritative persistence, but general operational logs must not leak unrevealed answers or future reveal order.

Use answer/word identifiers rather than plaintext hidden answer in broad logs where possible.

Client-visible diagnostics must never expose future hidden randomization/answer state.

## 8. Display name and profile data

Display name is untrusted presentation data and is not required for most authoritative audit records.

Prefer stable pseudonymous internal IDs in audit/logs.

If display name is needed for a specific support/debug view, escape/sanitize it and keep access restricted. Do not treat it as an identity key.

## 9. Network/device metadata

IP address, user agent/device metadata and similar network identifiers are collected only when needed for security/operations and are not copied into every gameplay event.

If retained, they belong to short-lived security/operational evidence classes and are subject to stricter access/retention minimization.

Device identifiers are not authentication authority under ADR-007.

## 10. Retention classes — PoC defaults

These are initial PoC defaults, not claims about every future production/legal jurisdiction. A future release may shorten or change them based on product/legal requirements, but must not weaken correctness retention while a match is recoverable.

### R0 — secrets/raw credentials

Retention in logs/audit: **0 days / prohibited**.

Credential stores may retain necessary verifier/state according to ADR-007 while the credential is valid; this is not logging retention.

### R1 — active authoritative state and correctness-critical replay evidence

Retain for the entire active/recoverable match lifecycle.

After terminal/abandoned state, retain correctness-critical idempotency/session recovery evidence for **at least 24 hours** unless a longer operational window is configured.

Never delete while legitimate reconnect/recovery can still occur.

### R2 — domain transition / score / gameplay audit

PoC default: **30 days after match termination**.

Purpose: qualification, correctness debugging, dispute reproduction and AEGIS/MUR evidence.

Where a more durable product history is intentionally offered later, product data and operational audit retention must be separated rather than silently extending all logs forever.

### R3 — security decision audit

PoC default: **30 days**.

Repeated abuse events may be compacted into bounded aggregates while preserving meaningful investigation signals.

### R4 — operational diagnostics / traces

PoC default: **7 days**.

Longer retention requires documented operational need and redaction review.

### R5 — aggregate metrics

May be retained longer when they no longer contain or allow practical reconstruction of individual player/session activity. High-cardinality pseudonymous labels should be removed before long-term aggregation.

## 11. Deletion / cleanup behavior

Cleanup is explicit and idempotent.

Rules:

- never delete active/recoverable correctness evidence;
- pending outbox records are not deleted before successful/terminal handling under ADR-005;
- deletion of audit/diagnostic classes follows their retention class;
- guest display-name/profile mapping may be removed earlier than short-term pseudonymous integrity evidence when no longer needed;
- future user/account deletion workflows must distinguish user-facing product data from short-lived security/correctness evidence and follow applicable product/legal requirements;
- deletion jobs themselves must not corrupt score/match integrity or resurrect data through stale replicas/cache.

## 12. Access control

Operational/audit data is not a public client API.

Access is limited to authorized operational/development roles appropriate to the environment.

Production-equivalent audit access should be least-privilege and auditable when practical.

A player's normal game client receives only its authorized gameplay view, not internal security/audit fields or other players' private session metadata.

## 13. Environment separation

Development/test and production-equivalent observability are logically separated.

- Test fixtures must not require production credentials.
- Production secrets must not appear in local test snapshots.
- Verbose debug logging acceptable in isolated tests must not silently remain enabled in production.
- Synthetic identifiers/data should be preferred in qualification artifacts where real user data is unnecessary.

## 14. Log injection / untrusted strings

User-controlled strings are never used as raw log format/templates.

Structured fields are escaped/encoded by the logging system.

Newlines/control characters in display names or malformed input cannot forge separate log records or fields.

## 15. Error reporting

External error responses expose stable safe reason categories, not stack traces, SQL details, credential state or hidden match information.

Internal diagnostics may include stack traces for implementation errors but must pass redaction rules and avoid dumping full request/session objects.

## 16. Audit questions that must be answerable

For a retained match/evidence window, authorized investigation must be able to determine:

1. Why did a question/stage/match end?
2. Which authoritative revision received the action/trigger?
3. Which Player owned or was eligible for the action?
4. Was an action a first delivery, exact replay, conflict or stale mutation?
5. Why was a claim accepted/rejected and which authoritative order decided it?
6. Why was score awarded, to whom, and under which unique award identity?
7. Which timer ID/deadline and exact-boundary rule applied?
8. Which word-policy/dataset version produced validation?
9. Which randomization reference/order was authoritative without leaking future hidden state to clients?
10. Why was a request rejected as unauthorized/stale/revoked/rate-limited?
11. Did recovery/restart/outbox retry alter gameplay state? It must be provably no unless a new canonical transition occurred.

## 17. Metrics / alert candidates

Implementation should expose metrics suitable for bounded operational alerts, for example:

- mutation reject counts by reason family;
- idempotency conflict count;
- stale/revoked session attempts;
- stale connection attempts;
- duplicate timeout/outbox deliveries;
- outbox backlog/oldest pending age;
- reconnect failure rate;
- persistence transaction failure rate;
- timer deadline processing lag;
- authoritative mutation latency;
- invalid/malformed payload rate.

Exact alert thresholds belong to implementation/performance policy (#20).

## 18. AEGIS/MUR verification

At minimum verify:

- raw reconnect/access token is absent from logs after success and failure paths;
- malformed free-form input is not persisted raw;
- committed lexical decision remains explainable from canonical/audit evidence;
- wrong-player/stale-session/idempotency-conflict events have stable reason/correlation fields;
- display name containing newline/control characters cannot forge logs;
- hidden answer/future reveal order is absent from client diagnostics;
- repeated abuse logging remains bounded;
- retention cleanup does not delete active/recoverable correctness evidence;
- expired R4 operational logs are deletable without affecting gameplay recovery;
- audit access/read endpoint, if any, is not reachable as normal player data;
- consumer/telemetry outage cannot change authoritative gameplay.

## 19. Canonical observability/privacy invariants

- `OBS-001` Logs/audit never contain raw access/reconnect credentials or secrets.
- `OBS-002` Raw microphone/audio is not logged or retained by default.
- `OBS-003` Malformed arbitrary user text is not persisted raw by default.
- `OBS-004` Gameplay audit prefers pseudonymous internal IDs over display names.
- `OBS-005` Retention is finite by evidence class; active correctness evidence is never prematurely deleted.
- `OBS-006` Operational logs/traces are not authoritative gameplay state.
- `OBS-007` Hidden answer/future randomization data is not exposed through client-facing diagnostics.
- `OBS-008` Security/logging paths are bounded against abuse/flood amplification.
- `OBS-009` Error responses do not disclose internal stack/secret/protected-state detail.
- `OBS-010` Audit evidence can explain committed score/ownership/timer/validation decisions during its retention window.

## Related canonical sources

- ADR-005 — Persistence, Snapshot and Event/Audit Strategy
- ADR-007 — PoC Identity and Authentication Model
- `docs/04-architecture/competitive-security-and-abuse-model.md`
- `docs/02-domain/intent-contract.md`
- `docs/02-domain/failure-and-replay-semantics.md`
- `docs/02-domain/authoritative-randomness.md`
- GitHub Issues #20, #21, #22, #23
