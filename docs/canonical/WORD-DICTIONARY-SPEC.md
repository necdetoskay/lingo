# Word Dictionary Specification v1.0

## Required datasets

The application packages a versioned offline dictionary with separate roles:

- **Guess dictionary**: words that make a player submission valid.
- **Answer pool**: curated words eligible as round targets.
- **Restricted entries**: words excluded from either role with a recorded reason.

Every answer-pool entry must have at least one Turkish definition.

## Record contract

Each word record includes normalized word, length, definition(s), part of speech when known, source, source version, licence, difficulty, `can_be_guess`, and `can_be_answer`.

Only normalized Turkish words of exactly 4, 5, or 6 letters qualify. The pipeline must define Unicode normalization and Turkish casing for `I/ı`, `İ/i`, `Ç`, `Ğ`, `Ö`, `Ş`, and `Ü`. Proper nouns, malformed tokens, abbreviations, foreign-only usages, and unsuitable inflections require explicit policy before inclusion.

## Pilot and gate

The first pilot contains 150 reviewed answers: 50 per length. It is not a production-sized vocabulary. A source is not accepted until licence, attribution, redistribution, transformation, and share-alike implications have been reviewed. Kaikki/Turkish Wiktionary remains a candidate, not an approved source.

The approved word-form source is [tdd-ai/hunspell-tr](https://github.com/tdd-ai/hunspell-tr), revision `7302eca5f3652fe7ae3d3ec06c44697c97342b4e` (2023-12-30), which declares MPL-2.0. It supplies word-form candidates only. It does not supply the required Turkish definitions for answers, so it cannot by itself approve an answer-pool entry. The 150 entries in `data/pilot-candidates.tsv` use newly written Turkish candidate definitions and remain excluded until their individual review state is `approved`.

The installed runtime selects only approved rows marked for both guessing and answers, with a non-empty definition and no restriction record; it has no fixture or online fallback.

The runtime never fetches or scrapes words. Dataset build provenance and version are retained so an installed dictionary can be identified and reproduced.
