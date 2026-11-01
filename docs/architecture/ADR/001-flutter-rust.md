# ADR-001: Flutter UI with Rust Game Core

**Status:** Accepted for V1.

Flutter provides Android-first interface and presentation. Rust is the authoritative gameplay core because its rules can be independently tested and shared without UI duplication. The bridge must be typed and generated or otherwise contract-tested.

This decision does not approve a particular SQLite owner or a dictionary source; those remain integration and data gates.
