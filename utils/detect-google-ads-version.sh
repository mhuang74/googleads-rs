#!/usr/bin/env bash
#
# detect-google-ads-version.sh — release detector for new Google Ads API majors.
#
# Spec: specs/googleads-rs-automated-upgrade-implementation-plan-3.md §4.1.
# Mechanism: one GitHub git/trees API request against googleapis/googleapis
# (no clone, no HTML scraping); lists google/ads/googleads/v* directories and
# takes the highest major. Major versions only — minor bumps are inferred from
# release notes by utils/update.sh at migration time.
#
# On a new major, creates (or skips if already present) a GitHub issue labeled
# `ready-for-agent` + `api upgrade bot` with body marker
# `google-ads-api-upgrade: vNN`, so the worker pipeline can pick it up.

set -euo pipefail

# --- Current version: Cargo.toml major (same authoritative value Phase 0 established) ---
CARGO_MAJOR="$(sed -nE 's/^version = "([0-9]+)\.[0-9]+\.[0-9]+"/\1/p' Cargo.toml | head -1)"
if [[ -z "${CARGO_MAJOR}" ]]; then
  echo "ERROR: could not read crate major version from Cargo.toml" >&2
  exit 1
fi

# --- Latest version: GitHub git/trees API, one request, recursive listing ---
API_URL="https://api.github.com/repos/googleapis/googleapis/git/trees/master?recursive=1"
TREE_JSON="$(curl -fsSL "${API_URL}")" || {
  echo "ERROR: git/trees API request failed: ${API_URL}" >&2
  exit 1
}
LATEST_MAJOR="$(jq -r '
  [.tree[].path
    | select(test("^google/ads/googleads/v[0-9]+/?$"))
    | capture("v(?<v>[0-9]+)").v
    | tonumber] | max
' <<<"${TREE_JSON}")"
if [[ -z "${LATEST_MAJOR}" || "${LATEST_MAJOR}" == "null" ]]; then
  echo "ERROR: could not extract latest Google Ads major version from git/trees response" >&2
  exit 1
fi

# --- Comparison: major versions only ---
if (( LATEST_MAJOR <= CARGO_MAJOR )); then
  echo "No new major version detected (current: v${CARGO_MAJOR}, latest: v${LATEST_MAJOR})"
  exit 0
fi

echo "New major version detected (current: v${CARGO_MAJOR}, latest: v${LATEST_MAJOR})"

MARKER="google-ads-api-upgrade: v${LATEST_MAJOR}"
TITLE="Upgrade Google Ads API v${CARGO_MAJOR} → v${LATEST_MAJOR}"

# --- Idempotency: skip if an open upgrade issue already targets this version ---
EXISTING="$(gh issue list --search "\"${MARKER}\" in:body" --state open --json number)"
if [[ "$(jq 'length' <<<"${EXISTING}")" -gt 0 ]]; then
  EXISTING_NUMBERS="$(jq -r 'map(.number) | join(", ")' <<<"${EXISTING}")"
  echo "Upgrade issue already exists (#${EXISTING_NUMBERS}); skipping creation."
  exit 0
fi

# --- Create the upgrade issue (born ready-for-agent; worker's sole go signal) ---
gh issue create \
  --title "${TITLE}" \
  --body "$(cat <<EOF
${MARKER}

Target version: v${LATEST_MAJOR}
Previous version: v${CARGO_MAJOR}
Current state: pending migration

A new Google Ads API major version (v${LATEST_MAJOR}) is available in [googleapis/googleapis](https://github.com/googleapis/googleapis/tree/master/google/ads/googleads). This upgrade was detected by the weekly release-detector workflow.
EOF
)" \
  --label "ready-for-agent" \
  --label "api upgrade bot"

echo "Created upgrade issue: ${TITLE}"