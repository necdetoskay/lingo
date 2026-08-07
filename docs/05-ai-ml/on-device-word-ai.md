# On-device Word AI Roadmap

Status: Experimental roadmap

## Goal

Use the Lingo word dataset as a practical edge-AI learning project: first classify words with small on-device language models, then create a reviewed gold dataset and train a compact specialized classifier suitable for phones.

## Phase A — Baseline classification

Start with a compact schema rather than free-form explanations.

Candidate labels:

- valid guess
- answer candidate
- daily-use band
- family-safe
- part of speech
- difficulty
- confidence

Deterministic information such as character count and normalization must be computed in code, not guessed by a model.

## Phase B — Mobile LLM experiment

Benchmark small quantized models on representative Android hardware.

Measure:

- model download size
- peak RAM
- cold load time
- tokens/second
- words/minute
- JSON/schema validity
- classification agreement with reviewed labels
- battery consumption
- thermal throttling

Process small batches and checkpoint progress after every batch.

## Phase C — Gold dataset

Create a reviewed dataset from:

- deterministic lexical rules
- high-confidence model labels
- human-reviewed ambiguous examples
- disagreement cases between models

Every gold label should retain provenance and review version.

## Phase D — Specialized classifier

Train a compact Turkish word classifier rather than using a generative LLM for every runtime classification.

Possible multi-task outputs:

- guess validity probability
- answer suitability probability
- difficulty class
- family-safety probability
- usage-frequency class

The specialized model must be benchmarked against both the gold dataset and the larger baseline model.

## Phase E — Active learning

Prioritize human review for:

- low-confidence predictions
- model disagreements
- rare words
- borderline foreign usage
- ambiguous proper-noun/common-noun forms
- labels that violate consistency rules

## Runtime principle

AI classification is a content-pipeline concern, not a dependency for validating every live guess. Live gameplay uses the shipped, versioned dataset and deterministic rule engine.
