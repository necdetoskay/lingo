# ADR-003: Rust owns the local SQLite database

**Status:** Accepted for V1.

The Rust core is the sole data-access owner for the packaged dictionary, player progress, statistics, badges, sound preference, and resumable round state. Flutter passes an application-support database path through the typed bridge and renders returned models; it does not open SQLite independently.

This prevents duplicate migrations and divergent rule/persistence behavior across UI paths. The schema deliberately includes provenance fields for every dictionary record and restrictions for excluded entries.

This decision does not approve a word-data source. The required 150-answer pilot remains gated on licence, attribution, redistribution, transformation, and definition review.
