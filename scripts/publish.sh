#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "==> Preflight checks"
command -v cargo >/dev/null 2>&1 || {
  echo "error: cargo not found in PATH" >&2
  exit 1
}

echo "==> Formatting"
cargo fmt --all

echo "==> Linting"
cargo clippy --all-targets -- -D warnings

echo "==> Tests"
cargo test --all

echo "==> Publish dry run"
cargo publish --dry-run

echo "==> Publish"
echo "This will upload the crate to crates.io."
read -r -p "Continue? (y/N) " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
  echo "Aborted."
  exit 0
fi

cargo publish
