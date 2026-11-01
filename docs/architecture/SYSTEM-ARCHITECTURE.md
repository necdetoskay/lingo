# System Architecture

```
Flutter UI / input / animation / audio
             |
      typed Flutter-Rust bridge
             |
Rust game core: rules, timer, scoring, state
             |
   one SQLite data-access owner
             |
dictionary + progress + statistics + preferences
```

The Rust core accepts commands and returns state transitions; it has no direct Flutter dependency. It is deterministic when provided the same command stream and clock observations. Flutter renders returned state and must not recreate rule calculations.

The integration spike selects the SQLite owner and bridge generator, proves Android builds, and adds an end-to-end round before feature work expands.
