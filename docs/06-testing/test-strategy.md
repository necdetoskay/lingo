# Test Strategy

Status: Canonical foundation

## Test layers

### Rule-engine unit tests

Cover:

- green/orange/wrong duplicate-letter evaluation
- attempt decrement rules
- score calculations
- completion bonuses
- fixed-value and decaying-value bonuses
- invalid-word outcomes
- Stage 3 transfer semantics

### State-transition tests

Every allowed transition must be tested together with forbidden transitions.

Examples:

- primary wrong guess -> steal window
- steal claim -> 3-second answering state
- failed steal -> fresh claim window for remaining eligible players
- Stage 3 valid wrong guess -> next player
- Stage 3 invalid guess -> remaining-attempt transfer

### Timer tests

Use a fake/virtual clock. Do not rely on real sleeps in rule tests.

Test:

- deadline before/at/after boundary
- timer cancellation on answer commit
- repeated timeout delivery
- reconnect during active timer
- simultaneous steal claims

### Multiplayer determinism tests

Given identical authoritative event order, all clients must reconstruct identical:

- active player
- remaining attempts
- eligibility
- visible letters
- score
- question state

### Word-platform tests

Cover:

- Turkish casing
- Unicode normalization
- dictionary membership
- answer-pool separation
- prohibited forms
- duplicate letters
- dataset version compatibility

### Property/invariant tests

Examples:

- a question cannot award its score twice
- attempts can never become negative
- a player cannot consume the same steal entitlement twice
- a completed question cannot accept another guess
- total awarded points must equal the sum of score events

## Golden game scenarios

Maintain complete scripted matches for 2, 3 and 4 players. These fixtures should exercise all stages and be executable headlessly before UI tests.

## PoC exit criteria

The Classic Mode PoC is not considered successful merely because screens work. It must demonstrate:

- deterministic rules
- correct multiplayer ownership
- timer correctness
- reconnect-safe state
- word-validation correctness
- reproducible scoring
- no double-award race conditions
