set shell := ["bash", "-eu", "-c"]

# Show all available recipes.
default:
  just --list

# Switch to main branch and update it.
sync-main:
  git switch main
  git pull --ff-only

# Sync main and create a new branch.
new-work branch:
  just sync-main
  git switch -c {{branch}}

# Commit all changes, push, and open a PR.
up-for-review message:
  if git diff --quiet && git diff --cached --quiet; then echo "No changes to commit; skipping commit."; else git add -A; git commit -m "{{message}}"; fi
  if git rev-parse --abbrev-ref --symbolic-full-name @{u} >/dev/null 2>&1; then git push; else git push -u origin HEAD; fi
  gh pr create --fill

# Commit all changes locally only.
commit-local message:
  git add -A
  git commit -m "{{message}}"

# Commit all changes locally, then push to remote.
commit-push message:
  git add -A
  git commit -m "{{message}}"
  if git rev-parse --abbrev-ref --symbolic-full-name @{u} >/dev/null 2>&1; then git push; else git push -u origin HEAD; fi

# Check formatting and clippy warnings.
rust-check:
  if ! cargo fmt --all -- --check; then echo "Hint: run 'just rust-fix' to attempt auto-fixes."; exit 1; fi
  if ! cargo clippy --all-targets -- -D warnings; then echo "Hint: run 'just rust-fix' to attempt auto-fixes."; exit 1; fi

# Fix formatting and apply clippy auto-fixes.
rust-fix:
  cargo fmt --all
  cargo clippy --all-targets --fix --allow-dirty --allow-staged

# Run all tests.
rust-test:
  cargo test --all-targets
