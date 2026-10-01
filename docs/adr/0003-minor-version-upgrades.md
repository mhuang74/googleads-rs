# 0003 — Minor-version upgrades through the same pipeline

Date: 2026-10-01 (grilling session, Q1–Q12)

## Status

Accepted

## Context

The automated upgrade pipeline (detect → issue-worker → upgrade) handled major versions only: the detector compared proto-directory majors via the git/trees API, and the marker format `google-ads-api-upgrade: vNN` carried no minor component. Google also ships minor releases (v25.0 → v25.1 → v25.2), mostly additive, and the crate's version convention (crate major.minor mirrors API major.minor) implies minor upgrades should be published too. A constraint makes exact minor pinning impossible: `utils/update.sh` downloads the `googleapis` **master tip** (`master.zip`), so once v25.2 lands upstream, protos "as of v25.1" cannot be fetched — the landed minor is whatever the release notes report at migration time.

## Decision

Minor upgrades flow through the **same pipeline** as majors (issue → worker → upgrade → AI repair), with these specifics:

1. **Detection signal: release-notes HTML.** The detector parses the same release-notes page `utils/update.sh` parses (`data-text="vM.N"` headings); detector and migration agree by construction. Minor detection runs only when the crate is already on the latest major.
2. **Marker format `vM[.N]`.** Minor issues carry `google-ads-api-upgrade: v25.2`; the worker regex accepts the optional `.N`. Branch/PR/commit names carry the full marker value.
3. **The marker's minor component is ADVISORY.** The upgrade workflow strips minor-carrying targets to major and calls `update.sh vM --force` (bypassing the same-major guard, since master.zip only has the tip). A minor upgrade may land a newer minor than the marker says (marker v25.2, landed 25.3) — titles are left as marker values; the migration commit's `Cargo.toml` diff shows the landed version.
4. **Major-pending suppression.** When a new major exists, no minor issues are filed for older majors — a v25.3 upgrade is wasted the moment v26 lands.
5. **Newer-wins supersession.** The detector auto-closes open minor issues superseded by a newer minor of the same major, and orphaned minor issues whose major is below the crate's current major.

## Consequences

- `update.sh` is untouched; all changes live in the detector script, the worker's extraction regex, and the upgrade workflow's migration step.
- The release-notes parser is deliberately **duplicated** between detector and `update.sh` (not extracted into a sourced lib): ten lines of bash don't justify coupling two upgrade-critical scripts; a twin-warning comment cross-references them.
- Unparsable release notes degrade minor detection only (warn + skip that run); majors remain detected via git/trees. HTML format drift is a known accepted risk shared with `update.sh`.
- A minor upgrade takes the AI-repair path with a full 5-attempt budget like a major; for additive minors the repair loop is typically idle.
