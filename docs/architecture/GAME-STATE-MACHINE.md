# Game State Machine

`Idle -> RoundStarted -> AttemptOpen -> (GuessRejected | GuessResolved | AttemptExpired) -> AttemptOpen | RoundWon | RoundFailed -> RoundResult -> Idle`

- `GuessRejected`: invalid dictionary input; no attempt is consumed and the existing deadline remains.
- `GuessResolved`: valid input is evaluated once. Correct guesses enter `RoundWon`; incorrect guesses consume one attempt.
- `AttemptExpired`: consumes exactly one active attempt. The fifth consumption enters `RoundFailed`.
- `RoundResult`: shows score/answer and may show the definition. Only an explicit continue command starts a subsequent round.

Commands against terminal or stale state are no-ops or explicit errors; they never duplicate points, attempts, badges, or persistence records.
