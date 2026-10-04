# 0004 — Release flow: release-plz + vet + semver + attestation

Date: 2026-10-04

## Status

Accepted

## Context

Estate releases follow: bump → gates → semver-checks → dry-run → publish →
tag → registry verify (engineering-standards RELEASES.md). Manual repetition
of that flow does not survive ten crates.

## Decision

- release-plz opens the versioning/CHANGELOG PR on every push to main; its
  merge publishes (command: release) with `CARGO_REGISTRY_TOKEN`.
- Gates upstream of publish: tier-a CI, `cargo vet check`, semver-checks
  against the previous tag.
- Weekly `vet-refresh.yml` keeps imports.lock aligned with Dependabot waves.
- `scripts/reproducible-build.sh` provides the estate's determinism check.

## Consequences

- Required secrets: `RELEASE_PLZ_TOKEN`, `CARGO_REGISTRY_TOKEN`.
- First release is the only manual-ish one: create tag `v0.1.0` after the
  first publish so semver-checks gains its baseline.
