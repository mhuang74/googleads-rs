#!/usr/bin/env bash
#
# detect-google-ads-version.sh — release detector for new Google Ads API
# major and minor versions.
#
# Spec: specs/googleads-rs-automated-upgrade-implementation-plan-3.md §4.1;
# minor support per docs/adr/0003-minor-version-upgrades.md.
#
# Mechanisms (ADR 0003):
# - Majors: one GitHub git/trees API request against googleapis/googleapis
#   (no clone, no HTML scraping); lists google/ads/googleads/v* directories
#   and takes the highest major.
# - Minors: the Google Ads API release-notes page — the same source
#   utils/update.sh parses at migration time, so detector and migration
#   agree by construction. The proto directory layout carries no minor
#   component, so release notes are the only minor signal.
#
# On a new major (or a new minor of the current major, when the crate is
# already on the latest major), creates (or skips if already present) a
# GitHub issue labeled `ready-for-agent` + `api upgrade bot` with body
# marker `google-ads-api-upgrade: vNN` (majors) or `google-ads-api-upgrade:
# vNN.N` (minors), so the worker pipeline can pick it up.
#
# A new major suppresses minor detection (ADR 0003): upgrading to v25.3 is
# wasted work the moment v26 lands. Minor issues superseded by a newer
# minor (same major) or orphaned by a major upgrade (issue major < crate
# major) are auto-closed here — newer-wins.

set -euo pipefail

# --- Current version: Cargo.toml major (same authoritative value Phase 0 established) ---
CARGO_MAJOR="$(sed -nE 's/^version = "([0-9]+)\.[0-9]+\.[0-9]+"/\1/p' Cargo.toml | head -1)"
if [[ -z "${CARGO_MAJOR}" ]]; then
  echo "ERROR: could not read crate major version from Cargo.toml" >&2
  exit 1
fi
CARGO_MINOR="$(sed -nE 's/^version = "[0-9]+\.([0-9]+)\.[0-9]+"/\1/p' Cargo.toml | head -1)"
if [[ -z "${CARGO_MINOR}" ]]; then
  echo "ERROR: could not read crate minor version from Cargo.toml" >&2
  exit 1
fi

# DRY_RUN=1 exercises the full detection path (real release-notes fetch and
# parse, real git/trees fetch, real read-only issue scans, real comparisons)
# but prints mutations (issue create/close) instead of executing them.
if [[ "${DRY_RUN:-0}" == "1" ]]; then
  gh_mutate() { echo "[DRY_RUN] gh $*"; }
else
  gh_mutate() { gh "$@"; }
fi

# --- Latest major: GitHub git/trees API, one request, recursive listing ---
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

# --- Release-notes minor parser ---
# TWIN WARNING: this function is deliberately duplicated from
# utils/update.sh (parse_release_notes_minor) per ADR 0003 — ten lines of
# bash do not justify coupling the two upgrade-critical scripts via a
# sourced lib. Keep the parsing logic identical in both places.
RELEASE_NOTES_URL="https://developers.google.com/google-ads/api/docs/release-notes"

parse_release_notes_minor() {
  local major=$1 html minors best
  if ! html=$(curl -fsSL --max-time 30 "$RELEASE_NOTES_URL"); then
    return 1
  fi
  minors=$(printf '%s' "$html" | grep -oE "data-text=\"v${major}\.[0-9]+" | grep -oE '[0-9]+$' | sort -n)
  if [ -z "$minors" ]; then
    return 1
  fi
  best=$(printf '%s\n' "$minors" | tail -n1)
  printf '%s\n' "$best"
}

# --- Stale minor issue sweep (ADR 0003 newer-wins) ---
# Closes open MINOR issues (dot-carrying markers) that are stale:
#   same-major mode: target minor <= crate minor (already shipped or
#     landed by an in-flight run's advisory marker), or target minor <
#     detected latest minor (superseded)
#   all mode: any issue major < crate major (orphaned by a major upgrade)
# Major issues (vNN, no dot) are never touched here.
# In-progress protection: issues labeled in-progress are left for the
# actively running upgrade to resolve — it owns its outcome.
close_stale_minor_issues() {
  local mode=$1 latest_minor=$2 open_issues issue_num issue_labels issue_body issue_marker issue_major issue_minor
  open_issues="$(gh issue list \
    --label "api upgrade bot" \
    --state open \
    --json number,body,labels \
    --jq '.[] | "\(.number)\t\(.body)"')"
  [[ -z "${open_issues}" ]] && return 0
  while IFS=$'\t' read -r issue_num issue_body; do
    # Read-only guard: fresh label lookup per issue (list snapshot may be stale).
    issue_labels="$(gh issue view "${issue_num}" --json labels --jq '.labels[].name' || true)"
    [[ "${issue_labels}" == *in-progress* ]] && {
      echo "Issue #${issue_num} is in-progress; leaving it for the active run"
      continue
    }
    issue_marker="$(printf '%s' "${issue_body}" | grep -oE 'google-ads-api-upgrade: v[0-9]+\.[0-9]+' | grep -oE 'v[0-9]+\.[0-9]+' || true)"
    [[ -z "${issue_marker}" ]] && continue # major-issue markers (vNN, no dot) are not minors
    issue_major="${issue_marker#v}"; issue_major="${issue_major%%.*}"
    issue_minor="${issue_marker##*.}"
    local close=false reason
    if [[ "${mode}" == "all" ]]; then
      if (( issue_major < CARGO_MAJOR )); then
        close=true; reason="orphaned: issue major v${issue_major} is below the crate's current major v${CARGO_MAJOR}"
      fi
    else
      if (( issue_major == CARGO_MAJOR )); then
        if (( issue_minor <= CARGO_MINOR )); then
          close=true; reason="stale: v${issue_major}.${issue_minor} is already covered by the crate's current version v${CARGO_MAJOR}.${CARGO_MINOR} (possibly landed by an in-flight upgrade's advisory marker)"
        elif (( issue_minor < latest_minor )); then
          close=true; reason="superseded: v${issue_major}.${issue_minor} is older than the newly detected v${issue_major}.${latest_minor}"
        fi
      fi
    fi
    if [[ "${close}" == "true" ]]; then
      gh_mutate issue close "${issue_num}" \
        --comment "Stale minor upgrade issue (ADR 0003 newer-wins): ${reason}. Closing; the detector will file an issue for the current target."
      echo "Closed stale minor issue #${issue_num} (${issue_marker}): ${reason}"
    fi
  done <<<"${open_issues}"
}

# --- New major path (unchanged behavior) ---
if (( LATEST_MAJOR > CARGO_MAJOR )); then
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

  # A new major orphans every open minor issue for older majors — sweep
  # before creating the major issue so the worker's oldest-first order
  # never processes a stale minor ahead of the major.
  close_stale_minor_issues "all" ""

  # --- Create the upgrade issue (born ready-for-agent; worker's sole go signal) ---
  gh_mutate issue create \
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
  exit 0
fi

echo "No new major version detected (current: v${CARGO_MAJOR}, latest: v${LATEST_MAJOR})"

# --- Minor path: only meaningful when the crate is already on the latest
# major (ADR 0003 suppression) ---
if (( LATEST_MAJOR != CARGO_MAJOR )); then
  echo "Crate major (${CARGO_MAJOR}) ahead of latest googleapis major (${LATEST_MAJOR}); skipping minor check."
  exit 0
fi

LATEST_MINOR="$(parse_release_notes_minor "${CARGO_MAJOR}")" || {
  echo "Warning: minor detection degraded — could not fetch or parse release notes for v${CARGO_MAJOR} at ${RELEASE_NOTES_URL}. Skipping minor check this run (majors remain detected via git/trees)." >&2
  exit 0
}
echo "Latest minor for v${CARGO_MAJOR} per release notes: v${CARGO_MAJOR}.${LATEST_MINOR}"

if (( LATEST_MINOR <= CARGO_MINOR )); then
  echo "No new minor version detected (current: v${CARGO_MAJOR}.${CARGO_MINOR}, latest: v${CARGO_MAJOR}.${LATEST_MINOR})"
  exit 0
fi

echo "New minor version detected (current: v${CARGO_MAJOR}.${CARGO_MINOR}, latest: v${CARGO_MAJOR}.${LATEST_MINOR})"

TARGET="v${CARGO_MAJOR}.${LATEST_MINOR}"
MARKER="google-ads-api-upgrade: ${TARGET}"
TITLE="Upgrade Google Ads API v${CARGO_MAJOR}.${CARGO_MINOR} → ${TARGET}"

# --- Idempotency: skip if an open upgrade issue already targets this version ---
EXISTING="$(gh issue list --search "\"${MARKER}\" in:body" --state open --json number)"
if [[ "$(jq 'length' <<<"${EXISTING}")" -gt 0 ]]; then
  EXISTING_NUMBERS="$(jq -r 'map(.number) | join(", ")' <<<"${EXISTING}")"
  echo "Upgrade issue already exists (#${EXISTING_NUMBERS}); skipping creation."
  exit 0
fi

# Newer-wins: close minor issues for this major that are already covered by
# the crate version or superseded by the newly detected minor.
close_stale_minor_issues "same-major" "${LATEST_MINOR}"

# --- Create the minor upgrade issue (born ready-for-agent; worker's sole go signal) ---
# NOTE (ADR 0003): the marker's minor component is ADVISORY. update.sh
# downloads the googleapis master tip, so the landed minor is whatever the
# release notes report at migration time and may exceed ${LATEST_MINOR}.
gh_mutate issue create \
  --title "${TITLE}" \
  --body "$(cat <<EOF
${MARKER}

Target version: ${TARGET}
Previous version: v${CARGO_MAJOR}.${CARGO_MINOR}
Current state: pending migration

A new Google Ads API minor version (${TARGET}) is available per the [release notes](${RELEASE_NOTES_URL}). This upgrade was detected by the weekly release-detector workflow. The migration lands the latest minor of v${CARGO_MAJOR} at migration time, which may exceed ${TARGET} (ADR 0003).
EOF
)" \
  --label "ready-for-agent" \
  --label "api upgrade bot"

echo "Created upgrade issue: ${TITLE}"
