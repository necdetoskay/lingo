# Repository Reset Inventory

**Reviewed baseline:** `main` at `9e4460192adb1a8aabaeaa3d1ccbec4b8420efe3`  
**Preserved at:** `backup/main-pre-single-player-v1-2026-11-01`

## Assessed content

The baseline contains no application source, dependency manifests, generated binaries, assets, workflows, or workflow runs. It contains `README.md` and 33 design/ADR documents for an unreleased 2–4 player, server-authoritative Classic Mode with warm-up, steal, duel, final, identity, realtime transport, security, and AEGIS/MUR readiness work.

Those documents are valuable historical design material but incompatible with the approved single-player, offline, Flutter + Rust V1. They are preserved unchanged on the backup branch and intentionally removed from this reset branch.

## Branches and work tracking

Seven `aegis/*` branches were reviewed; each is already merged into the baseline and differs from `main` by no files. Historical merged PRs #27–#32 document that design work. Open issues #1–#24 track the superseded multiplayer plan; this reset does not delete, edit, or close them because their follow-up disposition needs explicit review.

## Safe-reset result

This branch replaces the active tree with the canonical single-player V1 documentation only. It does not rewrite history, mutate `main`, remove remote branches, alter issues/PRs, or delete any preserved content.
