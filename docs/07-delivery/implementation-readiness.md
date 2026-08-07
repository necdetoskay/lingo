# Implementation Readiness

Status: Canonical gate

Implementation must not begin until the following PoC decisions are closed or deliberately scoped out.

## Game design

- [x] Stage 1 canonical rules
- [x] Stage 2 canonical rules
- [x] Stage 3 canonical rules
- [ ] Final-stage canonical rules
- [ ] Match winner/tie-break rules
- [ ] Stage transition UX
- [ ] Exact invalid-input policy for standard questions
- [ ] Exact 10-second bonus deadline boundary
- [ ] Whether Stage 2 bonus value continues decaying during steal chain

## Word platform

- [x] Guess Dictionary vs Answer Pool separation
- [x] Turkish normalization principle
- [x] Duplicate-letter evaluation principle
- [ ] Source/licensing decision for initial dictionary
- [ ] Canonical invalid-word policy
- [ ] Initial dataset schema finalized
- [ ] Dataset build/version process

## Multiplayer

- [x] Server-authoritative principle
- [x] Claim-window / answer-window separation
- [ ] Network latency/grace policy
- [ ] Disconnect/reconnect rules per state
- [ ] Room lifecycle
- [ ] Player-ready/start rules
- [ ] Host privileges, if any

## Architecture

- [ ] Mobile technology ADR
- [ ] Backend/runtime ADR
- [ ] Realtime transport ADR
- [ ] Persistence ADR
- [ ] Authentication/identity decision for PoC
- [ ] Local word-dataset storage ADR

## Testing

- [x] Test strategy defined
- [ ] Canonical Stage 1 golden scenarios
- [ ] Canonical Stage 2 golden scenarios
- [ ] Canonical Stage 3 golden scenarios
- [ ] 2-player full-match fixture
- [ ] 3-player full-match fixture
- [ ] 4-player full-match fixture

## AI/ML

AI/ML experiments do not block the initial game-engine PoC unless explicitly moved into sprint scope.

- [x] On-device AI roadmap documented
- [ ] Gold dataset definition
- [ ] Baseline model benchmark protocol
- [ ] Specialized classifier experiment plan

## Gate rule

The first implementation sprint begins only after the Final rules and the architecture ADR minimum set are accepted, and critical `TBD` items affecting deterministic gameplay are resolved.
