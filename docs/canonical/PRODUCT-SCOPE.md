# Product Scope v1.0

## In scope

- Android-first Flutter client
- Rust game core reached through a typed bridge
- Offline single-player continuous rounds
- 4/5/6-letter Turkish words
- Local SQLite dictionary, progress, statistics, and preferences
- Locked-green-letter input, tile feedback, scoring, streaks, badges
- Optional definition display after every completed round
- Sound effects and visual feedback

## Explicitly out of scope

- Multiplayer, rooms, turns, steals, duels, finals, and realtime networking
- User accounts, authentication, backend services, cloud synchronization
- Global leaderboards, tournaments, daily events
- Advertising, payments, shops, currencies
- Runtime web scraping or online dictionary dependency
- AI-generated live word validation

## Technology boundary

Flutter/Dart owns screens, input affordances, presentation animation, audio, and accessibility. Rust owns deterministic gameplay decisions and exposes an independently testable API. Exactly one data-access layer owns SQLite access; its placement must be chosen during the integration spike and documented before production persistence is added.

No implementation may broaden this scope without a canonical-spec change.
