# Pilot Dictionary Review

`pilot-candidates.tsv` contains 150 source-verified candidate words: 50 each
for four, five, and six letters. The root forms were checked against
`tdd-ai/hunspell-tr` revision `7302eca5f3652fe7ae3d3ec06c44697c97342b4e`
(2023-12-30), published under MPL-2.0.

Definitions in this file are newly written, concise Turkish descriptions. They
are candidates, not a claim of human approval. A reviewer must check that each
word is common, appropriate for the game, not a proper noun or an unsuitable
inflection, and that its definition is clear. Change a row's `review_state`
from `candidate` to `approved` only after that review. Approved rows are
seeded into the offline SQLite dictionary on the next app launch; `rejected`
rows are never loaded.

The app records the source, pinned revision, MPL-2.0 license, definition,
part of speech, and difficulty for each approved entry. The runtime does not
contact the source or scrape any online service.
