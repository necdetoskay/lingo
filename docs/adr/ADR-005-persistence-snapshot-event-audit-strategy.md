# ADR-005 — Persistence, Snapshot and Event/Audit Strategy

Status: **Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #9

## Context

Lingo's authoritative multiplayer engine must survive retries, response loss, reconnect, worker restart and partial failure without creating duplicate score, duplicate attempts, duplicate steal entitlement or divergent state.

The system also needs an audit trail capable of explaining why a question changed state, which player owned an action, which timer/deadline applied, which word-policy version was used and why score was awarded.

The Experience Event System (ADR-008) is explicitly non-authoritative and cannot become the persistence source of truth.

## Decision summary

Lingo will use a **state-first transactional persistence model with append-only domain/audit evidence and a transactional outbox (or equivalent same-transaction durable delivery record) for post-commit durable publication**.

This is **not full event sourcing**.

The canonical recovery source is the latest committed authoritative materialized state/revision. Append-only event/audit records explain and verify transitions but the PoC does not require rebuilding the entire match by replaying every event.

## 1. Durable authoritative state

Every authoritative gameplay mutation that changes competitive state must durably commit the resulting current state before it is considered accepted.

Logical durable state includes, where applicable:

- match lifecycle state and revision;
- frozen roster/order and player membership;
- stage/question identity and state;
- current owner / eligibility;
- remaining attempts;
- steal entitlement state;
- score totals and immutable score-award records;
- current timer identity and `deadlineAt`;
- effective word-policy/dataset version;
- authoritative derived randomness such as reveal order and cursor;
- terminal/completion state;
- required session-security/reconnect lineage state owned by ADR-007/#17;
- action/idempotency evidence needed for replay safety.

Exact schema/table decomposition is an implementation decision as long as these logical invariants are preserved.

## 2. Materialized state is recovery authority

The active runtime may keep an in-memory actor/cache/working copy for performance, but volatile memory is not the source of truth after a commit.

On worker restart or ownership transfer, the authoritative runtime reloads the latest committed materialized state and revision.

A stale in-memory copy cannot overwrite a newer committed revision.

## 3. Revision / optimistic concurrency

Each match aggregate or equivalent authoritative concurrency boundary has a monotonically advancing revision.

A state-changing transaction checks the expected/current revision and advances it atomically.

Concurrent mutations targeting the same revision cannot both commit. One wins; the other must reload and then either:

- become an exact idempotent replay; or
- fail as stale/illegal under current state.

Lost-update behavior is not permitted.

## 4. Atomic mutation bundle

For an accepted authoritative mutation, the following logical effects belong to one atomic commit boundary where applicable:

1. current aggregate/materialized state update;
2. revision advance;
3. attempt/entitlement/score state changes;
4. idempotency/action record;
5. immutable score-award/business-key record if score changes;
6. append-only committed domain-transition/audit record;
7. transactional outbox entry for any durable post-commit publication that must eventually occur.

The implementation may use one database transaction or another storage primitive with equivalent all-or-nothing semantics.

It is not acceptable for state to commit while the required idempotency/audit evidence silently fails, or for audit/score evidence to commit while state rolls back.

## 5. Idempotency ledger

The canonical gameplay idempotency key is scoped so one logical action cannot be treated as two actions within the same match. The recommended logical uniqueness is:

`(matchId, actionId)`

The record stores enough information to distinguish exact replay from conflict, including at least:

- `matchId`
- `actionId`
- intent type
- authoritative actor/player identity where applicable
- canonical payload fingerprint/hash
- processing outcome
- committed revision/result reference when accepted
- stable reason/result fingerprint needed to reproduce an equivalent response
- creation/commit time

### Same action ID + same payload

If the authorized caller retries an already committed action, return/reconstruct the prior result without repeating side effects.

### Same action ID + different payload

Reject fail-closed as `IDEMPOTENCY_CONFLICT`. Never create a new action.

### Concurrent duplicate first delivery

A uniqueness/serialization guard ensures only one transaction establishes the action record. Competing duplicates converge on the stored result rather than both mutating state.

## 6. Rejected intents

After current session/membership authorization succeeds, a deterministic gameplay/domain rejection may be stored in the idempotency ledger so replay returns the same logical decision and the same action ID cannot later be reused with a different payload.

Malformed or unauthenticated traffic rejected before a protected gameplay context is established need not create a gameplay idempotency record; it may produce bounded security/diagnostic evidence according to #17/#22.

A stale/revoked session is never granted access to a protected prior idempotent result merely because it knows an old action ID.

## 7. Score integrity

Score is derived server-side from canonical rules.

Every awarded score must have an immutable logical award identity/business key sufficient to prevent duplicate award.

Recommended examples include a unique award key derived from:

- question + award type; or
- canonical score-award event identity.

Updating score totals and establishing the corresponding immutable award record occur in the same atomic mutation transaction.

A response retry, duplicate domain event or duplicate outbox delivery cannot create a second award.

## 8. Attempt and entitlement integrity

Consumed attempts and steal entitlements are authoritative persisted state.

Consumption and the state transition caused by that consumption commit atomically.

Reconnect/retry/restart cannot restore them from client state.

A uniqueness/business invariant prevents the same player/question steal entitlement from being consumed twice.

## 9. Durable domain-transition journal

Each committed authoritative transition appends a durable domain/audit record with a per-match ordered identity.

Minimum logical fields where relevant:

- `eventId`
- `matchId`
- monotonic `matchSequence`
- `revisionBefore`
- `revisionAfter`
- `actionId` / timer action identity
- actor/player identity or system actor
- event/transition type
- authoritative timestamp
- reason/result code
- stage/question IDs
- attempt/claim/score-award identifiers
- timer ID/deadline context where relevant
- effective word-policy/dataset version
- randomization version/reference where relevant
- bounded/minimal transition payload needed for audit

The journal is append-only for committed transition facts. Corrections are represented by later records rather than silently editing historical facts.

## 10. Decision/security audit vs domain transitions

Not every rejected request is a domain event.

The implementation may keep separate logical categories:

- **DomainTransitionEvent** — a committed authoritative state transition;
- **Decision/Security Audit Record** — important accepted/rejected authorization or integrity decisions that do not change gameplay state;
- **Outbox Message** — durable instruction to publish a post-commit fact to another consumer.

This separation prevents noisy/hostile traffic from polluting the authoritative domain event stream while still allowing security diagnostics.

## 11. Experience Event System boundary

ADR-008 Experience Events are presentation/feedback facts, not persistence authority.

Rules:

- gameplay state commits first;
- durable domain/audit evidence commits with the state;
- experience events are emitted after commit;
- an experience consumer failure cannot roll back gameplay;
- reconnect snapshot comes from authoritative persisted state, not from replaying transient audio/animation feedback;
- transient experience feedback does not have to replay after reconnect unless a specific consumer contract requires it.

If durable eventual publication of an experience/telemetry notification is required, it is driven from a transactional outbox/equivalent, not from an uncommitted in-memory callback.

## 12. Transactional outbox requirement

Any post-commit external publication that **must eventually happen** and must correspond exactly to a committed transition uses a transactional outbox or equivalent same-commit durable delivery record.

The outbox entry is written inside the mutation transaction.

After commit, a dispatcher publishes it asynchronously.

### Publish failure

Gameplay remains committed. The outbox entry remains pending and is retried.

### Duplicate publish

Consumers must tolerate duplicate delivery using stable message/event identity.

### Dispatcher restart

Pending outbox items remain recoverable; no gameplay rollback occurs.

Purely optional diagnostics that may be safely dropped are not required to use the durable outbox.

## 13. Reconnect snapshot

Reconnect snapshot is generated from the latest committed authoritative materialized state/revision.

The snapshot includes only the caller-authorized view and enough state to continue deterministically, such as:

- lifecycle/stage/question state;
- frozen roster/order;
- current owner;
- attempts/eligibility;
- score;
- visible/revealed board state;
- active timer IDs/deadlines;
- effective word-policy version;
- current authoritative revision.

Future hidden answer/randomness data is filtered according to the security/randomness contracts.

The PoC does not require periodic event-sourcing snapshots because current materialized state is already durably maintained on each mutation.

## 14. Snapshot/cache staleness

A snapshot/cache with revision lower than current durable state is stale.

Stale state can be used only as a client-observed hint; it cannot overwrite or roll back authoritative state.

Reconnect always resolves against the latest committed durable revision.

## 15. Timer persistence

ADR-004 deadlines and timer identity are persisted with authoritative state.

A scheduler callback is only a trigger. On delivery it reloads/checks current timer identity, revision and deadline before transition.

Restart never recomputes a timer as `now + originalDuration`.

Duplicate/stale timeout delivery becomes a no-op.

## 16. Randomness persistence

The derived authoritative randomization result (for example Stage 1/2 reveal order) is persisted before the randomized question becomes active.

Recovery restores that result. If an active question's canonical randomization result cannot be recovered, fail closed; never silently draw a replacement order.

## 17. Worker crash / response-loss matrix

### Crash before atomic commit

No authoritative mutation is considered committed. The transaction rolls back/does not become visible. A retry may process the action normally.

### Crash during commit

Storage atomicity determines one of two observable states only: fully committed or not committed. Partial authoritative mutation is not allowed.

### Commit succeeds, process crashes before response

Retry of the same action ID/payload returns/reconstructs the committed result. No duplicate mutation/score/entitlement occurs.

### Commit succeeds, outbox publication fails

Gameplay remains committed. Publication retries from the durable outbox.

### Response lost in network

Same as post-commit crash: client retry converges through the idempotency ledger.

### Cache/snapshot stale after commit

Durable higher revision wins. Stale cache is discarded/reloaded and cannot overwrite current state.

### Audit/outbox consumer fails

Authoritative state remains valid; consumer recovery occurs independently according to its contract.

## 18. Recovery invariant

After restart/recovery, the system must be able to determine unambiguously whether an action committed.

"Unknown, maybe committed" is not an acceptable steady-state result for a mutation with an action ID.

The idempotency ledger + authoritative revision/state + durable transition evidence together provide the decision.

## 19. Active state vs durable storage

A match actor/process may own serialization and keep active state in memory for speed, but every accepted competitive mutation is durably committed before success is acknowledged externally.

Memory-only accepted gameplay is not permitted for the PoC authoritative path.

## 20. Retention and cleanup classes

Retention is policy-driven, but cleanup must respect correctness dependencies.

### Active/recoverable match state

Never cleaned while the match can still reconnect/recover.

### Idempotency evidence

Retained at least as long as any credential/session/reconnect path can legitimately replay or query the action and through the operational recovery/debug horizon.

It must not be deleted while the match remains active/recoverable.

### Domain/audit events

Retained according to operational/privacy policy (#22) long enough to support required dispute/debug/qualification evidence.

### Outbox

Pending items are never removed before successful/terminal delivery handling. Successfully handled items may be compacted after the audit/diagnostic need is satisfied.

Exact calendar durations are set with #22/privacy policy and deployment requirements; this ADR forbids correctness-breaking early deletion.

## 21. PII/minimum-data principle

Domain/audit persistence stores the minimum information needed to explain and protect gameplay correctness.

Do not persist raw credentials, reconnect tokens, secrets or unnecessary microphone/audio data in gameplay audit records.

Display names are not used where stable pseudonymous/internal identifiers are sufficient.

Detailed redaction/retention rules are owned by #22.

## 22. Non-goals

This ADR does not require:

- full event sourcing;
- Kafka/Redis Streams/distributed broker;
- a specific SQL/NoSQL product;
- synchronous analytics/telemetry in the gameplay transaction;
- replaying transient presentation effects on reconnect.

## Required MUR / failure-injection cases

The executable qualification suite must include at minimum:

1. same action replay after successful commit;
2. same action ID + different payload conflict;
3. two concurrent deliveries of the same action;
4. two concurrent legal mutations on the same revision;
5. state/score transaction rollback before commit;
6. simulated crash after commit before response;
7. response lost then retry;
8. outbox publish failure then retry;
9. duplicate outbox delivery;
10. stale snapshot attempting overwrite;
11. restart during active timer;
12. restart between random reveals;
13. duplicate score-award attempt;
14. duplicate steal-entitlement consumption;
15. terminal-state mutation after recovery;
16. missing/corrupt active-question randomization recovery fail-closed.

## Canonical invariants

- `PERSIST-001` Accepted competitive mutation is durable before success acknowledgement.
- `PERSIST-002` State + required idempotency + required domain/audit evidence commit atomically.
- `PERSIST-003` Same action replay cannot repeat side effects.
- `PERSIST-004` Same action ID with different payload fails closed.
- `PERSIST-005` Concurrent mutations cannot create lost updates across one authoritative revision.
- `PERSIST-006` Score award has an immutable unique business identity and cannot double-commit.
- `PERSIST-007` Consumed entitlement cannot be recreated by reconnect/restart.
- `PERSIST-008` Latest committed revision wins over stale cache/snapshot.
- `PERSIST-009` Durable outbox failure cannot roll back committed gameplay.
- `PERSIST-010` Active-question randomization cannot be regenerated during recovery.
- `PERSIST-011` Timer deadline is restored, not restarted, after recovery.
- `PERSIST-012` Experience events are never authoritative recovery source.

## Consequences

### Positive

- Response loss/retry converges safely.
- Double-award and duplicate entitlement can be prevented at storage boundary.
- Restart/reconnect has a deterministic recovery source.
- Audit evidence and presentation events are cleanly separated.
- External publication failure does not corrupt gameplay.

### Costs

- Every accepted competitive mutation requires durable commit latency.
- Storage schema must support revision/idempotency/business uniqueness.
- An outbox dispatcher/cleanup path is required for durable eventual publication.

## Related canonical sources

- `docs/02-domain/domain-model.md`
- `docs/02-domain/invariants.md`
- `docs/02-domain/intent-contract.md`
- `docs/02-domain/failure-and-replay-semantics.md`
- `docs/02-domain/authoritative-randomness.md`
- `docs/02-domain/multiplayer-lifecycle.md`
- `docs/04-architecture/competitive-security-and-abuse-model.md`
- ADR-004 — Authoritative Timer and Latency Policy
- ADR-008 — Game Experience Event System
- GitHub Issues #9, #17, #18, #21, #22
