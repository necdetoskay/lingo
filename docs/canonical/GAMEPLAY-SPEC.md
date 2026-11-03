# Gameplay Specification v1.1

## Core loop

Lingo is a continuous single-player Turkish word game. The player chooses a 4-, 5-, or 6-letter word and attempts to solve it in at most five attempts.

| Rule | Canonical behavior |
| --- | --- |
| First letter | Revealed at round start and required in every guess |
| Attempts | Five per target word |
| Attempt timer | 20 seconds of real elapsed time |
| Skip | Not available |
| Valid wrong guess | Consumes one attempt and yields tile feedback |
| Invalid guess | Does not consume an attempt; its timer continues |
| Correct guess | Ends round and awards score |
| Fifth failed attempt | Ends round, reveals answer, resets streak |
| Next round | Starts only after the player selects continue |

The UI may animate outcomes, but the game core is the sole authority for acceptance, timer expiry, attempts, scores, revealed-letter hints, and terminal state.

## Guess input and feedback

A guess is a complete dictionary word that begins with the revealed first letter. It must be present in the guess dictionary to be valid. A valid guess can differ from the answer.

Feedback is **green-first and presence-based**: a guessed letter at the exact answer position is green; otherwise, if the letter occurs anywhere in the answer, that position is yellow, **even when the guess repeats the letter more times than the answer contains it**; otherwise it is gray. Yellow occurrences are not count-limited. Example: answer ALEM, guess AAAA => green, yellow, yellow, yellow. This follows the accepted 2026-11-03 Lingo UI decision; it intentionally differs from two-pass Wordle-style accounting.

Every green position remains visibly revealed as a persistent hint. Other than the first letter, revealed green letters are not mandatory in later guesses: the player may submit another word with a different letter in that position to test alternatives. Yellow letters are also hints only.

## Time and lifecycle

Each attempt has an authoritative deadline. While the application is active, elapsed time should use a monotonic source. The active round stores its attempt start/deadline durably.

The timer does not pause in the background, when the screen locks, or while the app is terminated. On restore, the core reconciles real elapsed time chronologically: it expires as many outstanding attempts as needed, at most once each. If all attempts expire, the round becomes failed and waits at the result screen; it does not automatically begin another word.

If guess submission races deadline expiry, the core accepts it only when received at or before the stored deadline. A resolved attempt cannot be consumed or scored twice.

## Results and preferences

At round end, the answer is shown and its definition is available on demand. Sound effects default to enabled. Players may disable sounds; sound failure or mute must never change gameplay.
