# ADR-007 — PoC Identity and Authentication Model

Status: **Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #11

## Context

Lingo Classic Mode PoC needs enough identity assurance to bind realtime intents, reconnect the same player, enforce one mutation-authoritative connection, and preserve idempotency/security semantics without prematurely building a full account/password/social-login system.

The Competitive Security & Abuse Threat Model already requires server-resolved player identity, session security/revocation versioning and current connection generation. This ADR chooses the minimal PoC credential/identity model that satisfies those requirements.

## Decision summary

The PoC uses **guest-first, server-issued match/player sessions**.

Persistent user accounts are **not required** for the first gameplay implementation.

A guest may enter a display name and join/create a lobby. The server creates durable internal session/player identities and issues high-entropy bearer credentials needed for the active/reconnect lifecycle.

Future persistent accounts may be linked to guest/player history later without changing the active match's canonical Player identity.

## 1. Identity layers

The following concepts are separate:

### Installation / Device identifier

Optional, best-effort local installation identifier used for UX/diagnostics only when needed.

It is **not authentication authority** and is not trusted to prove player identity.

### Guest Identity

A server-created pseudonymous identity that may span more than one room/session if the PoC implementation chooses to retain it locally.

It does not require email, phone number, password or real name.

### PlayerSession

The security/session lineage representing one guest/user's participation context. It owns:

- stable internal `playerSessionId`;
- current `securityVersion` or equivalent revocation generation;
- current `connectionGeneration` or equivalent mutation lease generation;
- reconnect credential state;
- expiry/revocation state.

### Match Player

The authoritative player identity within one match/roster, with stable `playerId` bound server-side to one PlayerSession.

Gameplay ownership, score, attempts and entitlements attach to this match-scoped Player identity.

### Display Name

Presentation metadata only. It is not unique and has no authentication/authorization meaning.

## 2. Guest play

Guest/anonymous play is supported and is the default PoC path.

The initial PoC does not require:

- password registration;
- email verification;
- social login;
- phone/SMS verification;
- cross-device account recovery.

This avoids unnecessary PII and credential-management complexity while retaining authoritative multiplayer security.

## 3. Session creation

On valid lobby create/join flow, the server establishes or resolves a PlayerSession and match-scoped Player binding.

The server generates security-sensitive identifiers/credentials using cryptographically secure randomness.

Client-selected fields such as display name or optional local device ID never become the authoritative player identity.

## 4. Credential model

The logical model uses two separate concepts even if one implementation can technically encode them together:

### Active session/connection credential

Proves the current session lineage to establish a connection/request context.

It must be bound by server validation to:

- PlayerSession;
- current `securityVersion`;
- expiry/revocation policy.

### Reconnect credential

A high-entropy secret/capability used to prove continuity of an existing PlayerSession after transport loss/app restart within the allowed recovery window.

It is:

- server-issued;
- unguessable;
- stored client-side only in platform-appropriate secure storage where available;
- never logged;
- never accepted from URL/query-string channels where avoidable;
- stored server-side as a verifier/hash or equivalent non-plaintext representation when using opaque secrets;
- rotatable/revocable.

Exact signed-token vs opaque-token technology is a backend ADR/implementation choice. Security semantics in this ADR are mandatory either way.

## 5. Reconnect rotation

A successful reconnect/takeover performs one atomic security transition:

1. presented reconnect/session proof is validated;
2. current `securityVersion` is checked;
3. current PlayerSession and match membership are resolved;
4. server advances `connectionGeneration`;
5. server rotates/renews the reconnect credential when the credential design supports rotation;
6. new connection/request context becomes mutation-authoritative;
7. older connection generations immediately lose mutation authority.

If credential rotation is used, the previous reconnect secret is invalid after successful rotation.

This makes replay of a copied old reconnect credential fail closed after takeover.

## 6. Connection takeover

For the PoC, possession of a current valid reconnect/session credential may intentionally support same-player takeover after a dropped/stale connection.

The newest successfully validated takeover becomes the only mutation-authoritative connection generation.

An old socket may remain physically alive briefly, but every mutation from it fails with stale-connection semantics.

No two simultaneous connections can independently mutate the same PlayerSession.

## 7. Explicit revocation / sign-out

When a PlayerSession is explicitly revoked/signed out or security policy invalidates it:

- `securityVersion` advances or the lineage is marked revoked;
- all current active/reconnect credentials under the old version become invalid;
- existing connection generations lose mutation authority;
- future reconnect requires a new authorized lineage, not replay of an old secret.

For a guest with no persistent account, revocation may mean the old guest session is permanently unrecoverable after its allowed recovery window.

## 8. Expiry and lifecycle

Credential/session lifetime is bounded.

Minimum PoC policy:

- active/reconnect authority remains valid only through the active match plus a bounded post-disconnect recovery window;
- terminal/abandoned matches do not keep indefinite mutation-capable reconnect authority;
- exact wall-clock durations may be tuned with implementation/operational policy, but no credential is intentionally perpetual.

A later persistent account system may issue account-level credentials independently of match PlayerSession credentials.

## 9. Room/join code boundary

A room/join code is not an authentication credential for an existing PlayerSession.

It may allow a new guest to request membership before roster lock, subject to room/rate rules, but it cannot:

- impersonate an existing player;
- reconnect an existing PlayerSession;
- take over a player's match identity;
- bypass session credential checks.

## 10. Server intent authorization

After transport authentication, every gameplay mutation still follows the Competitive Security & Abuse authorization chain:

`current valid session -> current securityVersion -> current connectionGeneration -> match membership -> bound playerId -> current revision/state -> ownership/eligibility -> deadline -> idempotency -> domain guards`

Authentication is necessary but never sufficient to authorize a gameplay action.

## 11. Idempotency identity

`actionId` remains client/generated-or-resolved logical action identity within the match according to ADR-005.

The server also records/resolves the authoritative actor PlayerSession/Player when the action commits.

A copied `actionId` cannot grant another session access to the prior result because current authorization is evaluated before protected idempotent-result disclosure.

## 12. Privacy/minimum data

The default guest profile requires only the minimum UX metadata needed to play, such as a display name.

The PoC does not require collection of:

- real name;
- email;
- phone number;
- date of birth;
- address;
- social account identifiers.

Stable internal IDs are pseudonymous technical identifiers.

Raw access/reconnect credentials are secrets and are never included in gameplay audit logs.

Retention/deletion details are finalized by #22.

## 13. Display-name policy

Display name:

- is non-unique;
- is not an authentication factor;
- cannot be used to select another player's session;
- is length/character bounded;
- is treated as untrusted user content for rendering/logging;
- may be changed before match start according to UX rules without changing `playerId`.

Moderation/profanity UX policy may be layered later; it does not define identity authority.

## 14. Future account linking

The domain leaves an optional future `accountId` association outside match gameplay identity.

A later account-link operation may associate historical guest/player data with an authenticated account, but:

- it cannot change an active match's `playerId` or ownership;
- it cannot merge two simultaneous active player identities;
- it must preserve audit provenance;
- account credential compromise/recovery is handled by a future account-auth ADR/revision.

No account-specific assumptions are embedded into the Classic Mode rule engine.

## 15. Cross-device behavior

The PoC does not promise user-friendly cross-device recovery when the reconnect credential is unavailable.

If the user securely transfers/presents the current valid reconnect credential, a new device may perform a valid takeover under the same connection-generation rules. Otherwise a display name/room code/device ID is insufficient to recover the existing PlayerSession.

## 16. Credential storage guidance

Implementation must use platform-appropriate secure credential storage where available and must avoid plaintext persistence in application logs/config/files.

Examples may include mobile secure keystore/keychain or secure browser credential mechanisms depending on ADR-001/runtime choice.

The canonical requirement is protection against accidental exposure; no client storage mechanism is treated as unbreakable against a fully compromised device.

## 17. Failure behavior

### Missing credential

Protected reconnect/mutation fails unauthenticated.

### Invalid/expired credential

Fails without revealing protected match/player state.

### Old securityVersion

Fails `SESSION_STALE_OR_REVOKED` or equivalent.

### Old connectionGeneration

Fails `STALE_CONNECTION` or equivalent.

### Reused rotated reconnect credential

Fails closed; it cannot create a second authoritative connection.

### Credential validated but match already terminal/abandoned

Reconnect may return an authorized read-only terminal snapshot where policy permits, but mutation remains rejected.

## 18. Canonical identity invariants

- `AUTH-001` Display name and device ID are never authentication authority.
- `AUTH-002` One match Player is server-bound to one PlayerSession lineage.
- `AUTH-003` Guest play does not require persistent account PII.
- `AUTH-004` Reconnect requires a current high-entropy server-issued credential or equivalent proof.
- `AUTH-005` Successful takeover advances the current mutation-authoritative connection generation.
- `AUTH-006` Old connection generation cannot mutate after takeover.
- `AUTH-007` Revocation/security-version change invalidates older credentials/authority.
- `AUTH-008` Room/join code cannot reconnect/impersonate an existing player.
- `AUTH-009` Authentication does not bypass match/state/ownership/idempotency authorization.
- `AUTH-010` Raw credentials are not persisted in gameplay audit logs.
- `AUTH-011` Future account linking cannot rewrite active match identity/ownership.
- `AUTH-012` Credential/session authority is bounded in lifetime; no perpetual PoC bearer secret is intended.

## 19. Verification / MUR cases

At minimum test:

- guest join creates server-owned PlayerSession/Player binding;
- duplicate display names do not collide identity;
- forged client `playerId` cannot impersonate another player;
- reconnect with current valid credential preserves same player identity;
- successful reconnect advances connection generation;
- old socket mutation after takeover is rejected;
- replay of rotated/old reconnect credential fails;
- securityVersion revocation invalidates active/reconnect authority;
- room code alone cannot take over existing player;
- device ID alone cannot reconnect existing player;
- action replay by a different session cannot disclose protected prior result;
- terminal match cannot be mutated with otherwise valid old credential;
- raw credential never appears in structured audit/log fixture.

## Alternatives considered

### Mandatory persistent accounts from the start

Rejected for PoC: creates password/OAuth/recovery/PII complexity not required to validate Classic Mode gameplay architecture.

### Display-name + room-code identity only

Rejected: trivially impersonable and insufficient for reconnect/idempotency/security.

### Device ID as player identity

Rejected: device IDs are spoofable/unstable, create privacy coupling and do not model multi-session/match identity correctly.

### Multiple simultaneous mutation-authoritative connections

Rejected for PoC because it complicates ordering/takeover/replay and creates avoidable duplicate authority.

## Consequences

### Positive

- Secure-enough reconnect/session identity without full account system.
- Minimal PII.
- Clean compatibility with securityVersion/connectionGeneration model.
- Future account system can be added without rewriting gameplay identity.

### Costs / limitations

- Guest session recovery depends on possession of the reconnect credential.
- Cross-device recovery is intentionally limited before accounts exist.
- Credential rotation/storage/revocation logic must still be implemented correctly.

## Related canonical sources

- `docs/04-architecture/competitive-security-and-abuse-model.md`
- `docs/02-domain/intent-contract.md`
- `docs/02-domain/multiplayer-lifecycle.md`
- ADR-005 — Persistence, Snapshot and Event/Audit Strategy
- GitHub Issues #11, #17, #21, #22
