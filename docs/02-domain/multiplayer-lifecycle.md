# Multiplayer Lifecycle

Status: **Canonical / Accepted**  
Version: 1.0  
Date: 2026-09-14  
Tracking: GitHub Issue #5

## Purpose

Define transport-independent room, roster, ready/start, disconnect/reconnect, host, and abandonment semantics for the 2-4 player Classic Mode PoC.

Identity proof and session security are owned by ADR-007 / Issue #11 and the AEGIS/MUR security threat model. This document defines gameplay lifecycle semantics once a player/session identity has been resolved.

## Canonical lifecycle states

- `LOBBY_OPEN`
- `LOBBY_READY`
- `MATCH_ACTIVE`
- `MATCH_SUSPENDED_RECONNECT`
- `MATCH_COMPLETE`
- `MATCH_ABANDONED`

These are lifecycle states around the stage/question state machines. They do not replace stage/question states.

## Room creation and join

- A room is created in `LOBBY_OPEN`.
- Classic Mode requires at least 2 and at most 4 rostered players.
- Players may join only before match start.
- No late join is allowed after the authoritative roster is locked for match start.
- A player identity may occupy at most one roster slot.
- Duplicate network connections cannot create duplicate player identities or duplicate gameplay entitlements.

## Ready/start contract

- Each rostered player has an explicit ready flag.
- The match may start only when the current roster contains 2-4 players and every rostered player is ready.
- Starting the match atomically locks the roster and creates the initial authoritative match/stage state.
- Joining/leaving before start is evaluated against the current roster; start cannot use stale ready state from a removed identity.
- Once the match starts, ready flags have no gameplay authority.

## Lobby owner / host

A lightweight lobby owner exists only for pre-match coordination.

Allowed PoC privileges:

- initiate match start when the ready/start guard is satisfied;
- close an unstarted lobby.

The lobby owner cannot:

- change score;
- change attempts;
- choose claim winners;
- extend timers;
- mutate another player's gameplay state;
- alter the authoritative word-policy version.

If the lobby owner leaves before match start, ownership transfers deterministically to the longest-present connected roster member. If no player remains, the room closes.

After match start, host/owner status has **no competitive gameplay privilege**.

## Roster lock

At match start:

- player order is frozen;
- stage/question ownership derives from the frozen roster/order;
- no replacement player may enter an existing match;
- reconnect resolves to an existing roster identity rather than creating a new roster entry.

## Disconnect principles

A transport disconnect by itself does not:

- consume or restore an attempt;
- create a new steal entitlement;
- change score;
- change player order;
- reset a timer;
- change the effective word-policy/dataset version.

Connectivity is not a gameplay entitlement.

## Competitive timers continue

When an authoritative competitive timer is already active, disconnect does not pause or extend it.

Examples:

- Stage 1 bonus timer continues.
- Stage 2 steal-claim window continues.
- Stage 2 accepted claimant's 3-second answer timer continues.

If a disconnected claimant owns an active Stage 2 steal-answer window and misses its deadline, the normal failed-steal rule applies and the entitlement is consumed.

## Blocking disconnect outside an active competitive timer

If authoritative progression requires input from a disconnected player and no existing competitive timer can resolve that state, the match enters `MATCH_SUSPENDED_RECONNECT`.

The server establishes a **30-second authoritative reconnect deadline** under ADR-004.

During suspension:

- no new gameplay intent that would bypass the missing required owner is accepted;
- reconnect does not recreate consumed state;
- the authoritative snapshot/revision is preserved;
- stage/question state is not rewound.

If the required player reconnects at or before the reconnect deadline, the match resumes from the authoritative snapshot.

If the reconnect deadline expires, the match becomes `MATCH_ABANDONED`.

## Why PoC abandonment instead of player removal

The initial PoC does not dynamically rebalance a running match after a player disappears. Silent player removal would change turn order, question ownership, scoring opportunity, and later-stage fairness.

Therefore an unrecovered blocking disconnect abandons the match rather than inventing new mid-match rules.

Scores/events produced before abandonment remain audit data but the abandoned match does not produce a normal competitive winner result.

Future competitive continuation-with-fewer-players behavior requires a canonical rule revision and Golden/MUR coverage.

## Non-blocking disconnect

If the disconnected player is not currently required to advance authoritative state, play may continue until either:

- the player reconnects; or
- that player becomes the required owner and the blocking-disconnect rule applies; or
- the match completes first.

## Stage 2 connectivity semantics

- A disconnected eligible opponent receives no automatic steal claim.
- Claim eligibility is not duplicated by reconnect.
- A reconnect during a claim window sees the existing deadline; it does not reopen the window.
- A reconnect after that player's steal entitlement was consumed does not restore it.
- An accepted claimant that disconnects remains bound by the original 3-second answer deadline.

## Stage 3 connectivity semantics

`next eligible player` means the next player in the frozen canonical roster order who remains gameplay-eligible under Stage 3 rules. **Network connectivity does not silently remove a player from that order.**

Therefore:

- a disconnect alone does not transfer remaining attempts;
- if the next required owner is disconnected and no active competitive timer resolves the situation, the match enters reconnect suspension;
- reconnect resumes with the same owner/remaining-attempt state;
- timeout of the reconnect suspension abandons the match rather than skipping the player.

This keeps Stage 3 rotation deterministic and prevents strategic disconnect from changing ownership.

## Explicit leave / forfeit

During an active PoC match, an explicit permanent leave is treated as unrecoverable abandonment and transitions the match to `MATCH_ABANDONED`.

The PoC does not award a normal match victory merely because another participant leaves. A future ranked/production forfeit policy must be separately versioned.

## Duplicate connection

Lifecycle semantics require one gameplay identity/roster membership regardless of transport count.

Which connection remains active, takeover/revocation behavior, and credential/session proof are security/identity decisions owned by ADR-007 and Issue #17. Regardless of that policy, duplicate connections cannot duplicate attempts, claims, score awards, or ready identities.

## Snapshot required for reconnect

A reconnect snapshot must contain enough authoritative information to continue without reconstructing rights from the client, including where relevant:

- match/lifecycle state and revision;
- frozen roster/order;
- current stage/question;
- active/required owner;
- remaining attempts;
- steal eligibility/consumed entitlement state;
- visible/revealed letters and prior guesses;
- score;
- active timer identities and deadlines;
- effective word-policy/dataset version.

## Invariants

- Reconnect maps to the same player/roster identity.
- Roster is immutable after start.
- Reconnect does not reset deadlines.
- Reconnect does not restore consumed attempts or entitlements.
- Disconnect alone does not alter Stage 3 order.
- Host privilege never overrides authoritative gameplay.
- An abandoned match cannot accept further gameplay mutation.

## Verification

Tests must cover at minimum:

- 2/3/4-player lobby ready/start;
- join rejected after roster lock;
- lobby owner transfer before start;
- host has no gameplay mutation privilege;
- reconnect during active Stage 1 bonus timer;
- reconnect during Stage 2 claim window;
- accepted Stage 2 claimant disconnect/timeout;
- reconnect after consumed steal entitlement;
- Stage 3 required-owner disconnect/resume;
- reconnect deadline exact boundary;
- unrecovered blocking disconnect -> `MATCH_ABANDONED`;
- explicit leave during active match -> abandoned;
- duplicate connection cannot create duplicate entitlement.

## Related canonical sources

- `docs/02-domain/domain-model.md`
- `docs/02-domain/invariants.md`
- `docs/02-domain/match-state-machine.md`
- ADR-004
- ADR-007 (planned)
- GitHub Issues #5, #11, #16, #17, #21
