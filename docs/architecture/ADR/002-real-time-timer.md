# ADR-002: Real-Time Attempt Timer

**Status:** Accepted for V1.

Each attempt has a stored 20-second deadline. The timer continues while backgrounded and across application termination. Restore reconciles elapsed wall time against stored state while preventing duplicate expiration. Active in-process timing uses a monotonic clock where available; restore must defend against device-clock anomalies and record its policy in tests.

A guess at the deadline is accepted only under the game core's single authoritative ordering rule. Presentation ticks are not authority.
