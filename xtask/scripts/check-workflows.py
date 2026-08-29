#!/usr/bin/env python3
"""Validate the post-ADR-0002 workflow split.

Since ADR 0002 (commit 569da0f), the detector lives in its own workflow
(`google-ads-detect.yml`), and `google-ads-upgrade.yml` is dispatched
exclusively by the issue worker (plus manual `resume=true` re-dispatch).
The old `inputs.issue == 0` routing logic and its `if:` guards are retired.

Verifies (structural, cross-file):
- google-ads-upgrade.yml: `upgrade` job has NO `if:` guard
- google-ads-detect.yml: `detect` job has NO `if:` guard
- google-ads-upgrade.yml: triggers are `workflow_dispatch` only (no schedule)
- google-ads-detect.yml: triggers are exactly `schedule` AND `workflow_dispatch`
- Branch-name construction follows bot/google-ads-vNN-issue-N pattern

Zero external dependencies: yaml parsed manually via simple text scanning,
since PyYAML may not be installed.
"""

import re
import sys
from pathlib import Path

UPGRADE_PATH = ".github/workflows/google-ads-upgrade.yml"
DETECT_PATH = ".github/workflows/google-ads-detect.yml"


def extract_job_conditions(yaml_text: str) -> dict[str, str]:
    """Extract `if:` expressions for each job in a GitHub Actions workflow.

    Returns a dict mapping job name -> raw if expression (or empty string
    if the job has no if condition). Used here to assert the ABSENCE of
    `if:` guards (retired by ADR 0002).
    """
    jobs: dict[str, str] = {}
    lines = yaml_text.splitlines()

    in_jobs = False
    current_job: str | None = None

    for line in lines:
        if line.startswith("jobs:"):
            in_jobs = True
            continue

        if not in_jobs:
            continue

        # A job definition is a non-empty key at the same indent level
        # as the first job we see, indented by 2 spaces under `jobs:`
        # e.g. "  upgrade:" or "  detect:"
        match = re.match(r"^  (\w+):\s*$", line)
        if match:
            current_job = match.group(1)
            assert current_job is not None
            jobs[current_job] = ""
            continue

        if current_job is None:
            continue

        if_match = re.match(r"^    if:\s*(.+)$", line)
        if if_match and current_job is not None:
            jobs[current_job] = if_match.group(1).strip()

    return jobs


def extract_triggers(yaml_text: str) -> set[str]:
    """Extract trigger names from the top-level `on:` block.

    Handles the block form used by this repo's workflows:

        on:
          schedule:
            - cron: ...
          workflow_dispatch:
            inputs: ...

    as well as inline forms (`on: push`, `on: [push, workflow_dispatch]`).
    Returns a set of trigger names. No PyYAML dependency: trigger keys are
    matched at exactly 2-space indent inside the `on:` block, which excludes
    nested keys like `inputs:` (4-space indent).
    """
    triggers: set[str] = set()
    lines = yaml_text.splitlines()

    i = 0
    while i < len(lines):
        line = lines[i]
        if re.match(r"^on:\s*$", line):
            # Block form: scan indented keys until the next top-level key.
            i += 1
            while i < len(lines):
                inner = lines[i]
                if inner.strip() and not inner.startswith((" ", "\t")):
                    break  # next top-level key: `on:` block ended
                m = re.match(r"^  ([\w-]+):\s*(?:#.*)?$", inner)
                if m:
                    triggers.add(m.group(1))
                i += 1
            return triggers
        inline = re.match(r"^on:\s*(.+)$", line)
        if inline:
            rest = inline.group(1).strip()
            if rest.startswith("[") and rest.endswith("]"):
                triggers |= {
                    t.strip().strip("'\"") for t in rest[1:-1].split(",") if t.strip()
                }
            elif rest:
                triggers.add(rest.strip("'\""))
            return triggers
        i += 1

    return triggers


def validate_upgrade_has_no_if_guard(conditions: dict[str, str]) -> list[str]:
    """Verify the `upgrade` job has no `if:` condition (retired by ADR 0002)."""
    errors: list[str] = []
    cond = conditions.get("upgrade")
    if cond is None:
        errors.append("google-ads-upgrade.yml: no `upgrade` job found")
    elif cond:
        errors.append(
            "upgrade job has an `if:` guard — retired by ADR 0002 "
            f"(routing is structural now): {cond!r}"
        )
    return errors


def validate_detect_has_no_if_guard(conditions: dict[str, str]) -> list[str]:
    """Verify the `detect` job has no `if:` condition (retired by ADR 0002)."""
    errors: list[str] = []
    cond = conditions.get("detect")
    if cond is None:
        errors.append("google-ads-detect.yml: no `detect` job found")
    elif cond:
        errors.append(
            "detect job has an `if:` guard — retired by ADR 0002 "
            "(the job lives in its own schedule-triggered workflow): "
            f"{cond!r}"
        )
    return errors


def validate_upgrade_triggers(triggers: set[str]) -> list[str]:
    """Verify google-ads-upgrade.yml has `workflow_dispatch` only (no schedule).

    Scheduled detection moved to google-ads-detect.yml (ADR 0002); a
    scheduled upgrade dispatch would re-run an upgrade unattended. Any
    other trigger (push/pull_request/…) is equally unauthorized: the
    workflow must be dispatch-only.
    """
    errors: list[str] = []
    if "schedule" in triggers:
        errors.append(
            "google-ads-upgrade.yml has a `schedule` trigger — the schedule "
            "moved to google-ads-detect.yml (ADR 0002)"
        )
    if "workflow_dispatch" not in triggers:
        errors.append(
            "google-ads-upgrade.yml is missing the `workflow_dispatch` trigger "
            "(worker dispatch + manual resume=true requires it)"
        )
    if triggers - {"workflow_dispatch"}:
        errors.append(
            "google-ads-upgrade.yml has unauthorized extra triggers "
            f"{sorted(triggers - {'workflow_dispatch'})} — dispatch-only "
            "per ADR 0002"
        )
    return errors


def validate_detect_triggers(
    triggers: set[str], allowed: frozenset[str] | None = None
) -> list[str]:
    """Verify google-ads-detect.yml triggers are `schedule` AND `workflow_dispatch`.

    Anything else (e.g. push/pull_request) is unauthorized: the detector
    runs on the weekly cadence or on a manual testing dispatch.
    """
    if allowed is None:
        allowed = frozenset({"schedule", "workflow_dispatch"})
    errors: list[str] = []
    missing = allowed - triggers
    if missing:
        errors.append(
            f"google-ads-detect.yml is missing required triggers {sorted(missing)} "
            "(weekly cadence + manual detector-only runs, ADR 0002)"
        )
    extra = triggers - allowed
    if extra:
        errors.append(
            f"google-ads-detect.yml has unauthorized extra triggers {sorted(extra)} "
            "— schedule/workflow_dispatch only"
        )
    return errors


def validate_branch_name_pattern() -> list[str]:
    """Verify the branch-name construction follows bot/google-ads-vNN-issue-N."""
    errors: list[str] = []

    yaml_text = Path(UPGRADE_PATH).read_text()

    # The branch name is constructed as bot/google-ads-${TARGET_VERSION}-issue-${ISSUE_NUMBER}
    pattern = r"bot/google-ads-\$\{.*?TARGET_VERSION.*?\}-issue-\$\{.*?ISSUE_NUMBER.*?\}"
    if not re.search(pattern, yaml_text):
        errors.append(
            "Branch-name construction does not follow "
            "bot/google-ads-vNN-issue-N pattern"
        )

    return errors


def main() -> int:
    upgrade_path = Path(UPGRADE_PATH)
    detect_path = Path(DETECT_PATH)
    for path in (upgrade_path, detect_path):
        if not path.exists():
            print(f"Error: {path} not found", file=sys.stderr)
            return 1

    upgrade_text = upgrade_path.read_text()
    detect_text = detect_path.read_text()

    print("Job conditions:")
    for label, text in (
        (UPGRADE_PATH, upgrade_text),
        (DETECT_PATH, detect_text),
    ):
        for job, cond in extract_job_conditions(text).items():
            print(f"  {label}#{job}: {cond or '(none)'}")

    print("\nTriggers:")
    upgrade_triggers = extract_triggers(upgrade_text)
    detect_triggers = extract_triggers(detect_text)
    print(f"  {UPGRADE_PATH}: {sorted(upgrade_triggers)}")
    print(f"  {DETECT_PATH}: {sorted(detect_triggers)}")

    all_errors: list[str] = []
    all_errors.extend(validate_upgrade_has_no_if_guard(extract_job_conditions(upgrade_text)))
    all_errors.extend(validate_detect_has_no_if_guard(extract_job_conditions(detect_text)))
    all_errors.extend(validate_upgrade_triggers(upgrade_triggers))
    all_errors.extend(validate_detect_triggers(detect_triggers))
    all_errors.extend(validate_branch_name_pattern())

    if all_errors:
        print("\nFAILURES:")
        for err in all_errors:
            print(f"  ✗ {err}")
        return 1

    print("\nAll workflow condition checks passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())