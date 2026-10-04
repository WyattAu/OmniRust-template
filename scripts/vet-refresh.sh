#!/usr/bin/env bash
# Regenerate cargo-vet imports after a Dependabot wave. CI (vet-refresh.yml)
# opens the PR; humans review it like any other.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo vet regenerate imports
cargo vet fix-imports
