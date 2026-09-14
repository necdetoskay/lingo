# Architecture Decision Records

Architecture decisions are recorded here before implementation when they materially constrain the system.

## Required pre-implementation ADRs

| ADR | Decision | Status |
|---|---|---|
| ADR-001 | Mobile application technology | Planned |
| ADR-002 | Backend/runtime architecture | Planned |
| ADR-003 | Realtime multiplayer transport | Planned |
| ADR-004 | Authoritative timer and latency policy | **Accepted** |
| ADR-005 | Persistence and event/audit strategy | Planned |
| ADR-006 | Local word dataset storage and synchronization | Planned |
| ADR-007 | PoC identity/authentication model | Planned |
| ADR-008 | Game Experience Event System | Proposed |

## Files

- `ADR-004-authoritative-timer-and-latency-policy.md`
- `ADR-008-game-experience-event-system.md`

The remaining required ADR files are intentionally absent until their decisions are actually drafted; registry status must not imply that a missing ADR has been accepted.

## ADR status model

- Planned — required decision tracked but no canonical ADR file accepted yet
- Proposed — ADR exists but is not yet authoritative
- Accepted — decision is canonical authority
- Superseded — replaced by a later accepted decision
- Rejected — considered and explicitly not adopted

Each ADR must include context, decision, alternatives considered, consequences, and verification/rollback notes where relevant.

A document may not cite a missing Planned ADR as though its details were already decided. Where a dependency is unresolved, the dependent document must either remain non-final or constrain itself only to already-established invariants.
