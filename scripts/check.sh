#!/usr/bin/env bash
# Local quality gate: formatting, lints, tests and a dependency audit.
# Run it manually, or automatically before every push via the hook in
# .githooks/pre-push (enable once with: git config core.hooksPath .githooks).
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

step() {
    printf '\n\033[1m── %s\033[0m\n' "$1"
    shift
    "$@"
}

step "rustfmt" cargo fmt --all --check
step "clippy"  cargo clippy --workspace --all-targets -- -D warnings
step "tests"   cargo test --workspace

if command -v cargo-audit >/dev/null 2>&1; then
    step "audit" cargo audit
else
    printf '\n── audit: cargo-audit not installed, skipping (cargo install cargo-audit)\n'
fi

printf '\n\033[1;32m✓ all checks passed\033[0m\n'
