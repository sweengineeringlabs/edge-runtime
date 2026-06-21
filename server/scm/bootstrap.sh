#!/usr/bin/env bash
set -euo pipefail
git config core.hooksPath server/scm/scripts/hooks
echo "bootstrap: hooks configured."
