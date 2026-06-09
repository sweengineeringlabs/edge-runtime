#!/usr/bin/env bash
# edge-runtime bootstrap — installs git hooks and fetches dependencies.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "==> Installing git hooks"
git -C "$REPO_ROOT" config core.hooksPath scm/scripts/hooks
echo "    core.hooksPath -> scm/scripts/hooks (pre-commit, commit-msg)"
echo "==> Fetching dependencies"
(cd "$REPO_ROOT/scm" && cargo fetch --locked)
echo "Bootstrap complete."
