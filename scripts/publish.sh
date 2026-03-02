#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

# ── helpers ──────────────────────────────────────────────────────────────────
info()    { echo "ℹ️  $*"; }
success() { echo "✅ $*"; }
abort()   { echo "🚫 $*" >&2; exit 1; }
step()    { echo ""; echo "🔷 $*"; }

# ── preflight ────────────────────────────────────────────────────────────────
step "Preflight checks"

command -v cargo >/dev/null 2>&1 || abort "cargo not found in PATH"
command -v git   >/dev/null 2>&1 || abort "git not found in PATH"
command -v curl  >/dev/null 2>&1 || abort "curl not found in PATH"

# Must be on main branch
current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$current_branch" != "main" ]]; then
  abort "Must publish from 'main' branch (currently on '$current_branch')"
fi

# Working tree must be clean
if ! git diff --quiet || ! git diff --cached --quiet; then
  abort "Working tree has uncommitted changes — commit or stash them first"
fi

# Extract crate version from Cargo.toml
version="$(grep '^version' Cargo.toml | head -1 | sed 's/.*= *"\(.*\)"/\1/')"
info "Crate version: $version"

# Check whether this version is already published on crates.io
crate_name="$(grep '^name' Cargo.toml | head -1 | sed 's/.*= *"\(.*\)"/\1/')"
published="$(curl -sf "https://crates.io/api/v1/crates/${crate_name}/${version}" \
  -H "User-Agent: publish-script" | grep -c '"num":"'"${version}"'"' || true)"
if [[ "$published" -gt 0 ]]; then
  abort "v${version} is already published on crates.io — bump the version first"
fi
info "v${version} not yet published — proceeding"

# ── quality gates ─────────────────────────────────────────────────────────────
step "Checking formatting"
if ! cargo fmt --all -- --check; then
  abort "Formatting check failed — run 'just rust-fix' and commit the changes"
fi
success "Formatting OK"

step "Linting"
cargo clippy --all-targets -- -D warnings
success "Clippy OK"

step "Tests"
cargo test --all-targets
success "All tests passed"

step "Publish dry run"
cargo publish --dry-run --allow-dirty
success "Dry run OK"

# ── confirm and publish ───────────────────────────────────────────────────────
echo ""
echo "📦 Ready to publish ${crate_name} v${version} to crates.io."
read -r -p "   Continue? (y/N) " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
  echo "🚫 Aborted."
  exit 0
fi

step "Publishing"
cargo publish
success "${crate_name} v${version} published to crates.io"

# ── git tag ───────────────────────────────────────────────────────────────────
step "Tagging release"
tag="v${version}"
if git rev-parse "$tag" >/dev/null 2>&1; then
  info "Tag $tag already exists — skipping"
else
  git tag -a "$tag" -m "Release $tag"
  git push origin "$tag"
  success "Tagged and pushed $tag"
fi
