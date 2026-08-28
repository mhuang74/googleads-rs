#!/usr/bin/env python3
"""Validate google-ads-upgrade.yml job-condition branching logic.

Verifies:
- detect and upgrade jobs are mutually exclusive under workflow_dispatch
  with issue=0 vs issue!=0
- Branch-name construction follows bot/google-ads-vNN-issue-N pattern
- No job condition allows both jobs to run simultaneously under the same dispatch

Zero external dependencies: uses only stdlib (yaml parsed manually via
simple text scanning, since PyYAML may not be installed).
"""

import re
import sys
from pathlib import Path


def extract_job_conditions(yaml_text: str) -> dict[str, str]:
    """Extract `if:` expressions for each job in a GitHub Actions workflow.

    Returns a dict mapping job name -> raw if expression (or empty string
    if the job has no if condition).
    """
    jobs: dict[str, str] = {}
    lines = yaml_text.splitlines()

    # Find the `jobs:` section
    in_jobs = False
    current_job: str | None = None

    for line in lines:
        # Detect `jobs:` top-level key
        if line.startswith("jobs:"):
            in_jobs = True
            continue

        if not in_jobs:
            continue

        # A job definition is a non-empty key at the same indent level
        # as the first job we see, indented by 2 spaces under `jobs:`
        # e.g. "  detect:" or "  upgrade:"
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


def evaluate_condition(condition: str, event_name: str, issue: int) -> bool:
    """Evaluate a GitHub Actions `if:` expression for given inputs.

    Supports the subset of expressions used in this repo's workflows:
    - github.event_name == 'value'
    - inputs.issue == N
    - && (logical AND)
    - || (logical OR)
    """
    if not condition:
        # No condition means the job always runs
        return True

    # Replace GitHub context references with Python-evaluable expressions
    expr = condition

    # github.event_name == 'schedule' -> check against event_name
    expr = expr.replace("github.event_name", f"'{event_name}'")

    # inputs.issue == 0 -> check against issue value
    expr = expr.replace("inputs.issue", str(issue))

    # && -> and, || -> or
    expr = expr.replace("&&", " and ").replace("||", " or ")

    try:
        return bool(eval(expr, {"__builtins__": {}}, {}))
    except Exception as exc:
        print(f"Warning: could not evaluate condition: {condition!r} -> {expr!r}: {exc}", file=sys.stderr)
        return False


def validate_mutual_exclusivity(conditions: dict[str, str]) -> list[str]:
    """Verify detect and upgrade are never both true under the same dispatch."""
    errors: list[str] = []

    test_cases = [
        ("workflow_dispatch", 0),
        ("workflow_dispatch", 76),
        ("workflow_dispatch", 1),
    ]

    for event, issue in test_cases:
        detect_runs = evaluate_condition(
            conditions.get("detect", ""), event, issue
        )
        upgrade_runs = evaluate_condition(
            conditions.get("upgrade", ""), event, issue
        )

        if detect_runs and upgrade_runs:
            errors.append(
                f"Both detect and upgrade run for event={event}, issue={issue}"
            )

    return errors


def validate_issue_zero_guard(conditions: dict[str, str]) -> list[str]:
    """Verify upgrade does NOT run when issue=0 (detector-only test path)."""
    errors: list[str] = []

    upgrade_runs = evaluate_condition(
        conditions.get("upgrade", ""), "workflow_dispatch", 0
    )
    if upgrade_runs:
        errors.append(
            "upgrade job runs on workflow_dispatch with issue=0 "
            "(should be excluded — detector-only path)"
        )

    return errors


def validate_detector_runs_on_zero(conditions: dict[str, str]) -> list[str]:
    """Verify detect DOES run when issue=0 (detector-only test path)."""
    errors: list[str] = []

    detect_runs = evaluate_condition(
        conditions.get("detect", ""), "workflow_dispatch", 0
    )
    if not detect_runs:
        errors.append(
            "detect job does NOT run on workflow_dispatch with issue=0 "
            "(detector-only test path broken)"
        )

    return errors


def validate_upgrade_runs_on_real_issue(
    conditions: dict[str, str]
) -> list[str]:
    """Verify upgrade DOES run when issue != 0."""
    errors: list[str] = []

    upgrade_runs = evaluate_condition(
        conditions.get("upgrade", ""), "workflow_dispatch", 76
    )
    if not upgrade_runs:
        errors.append(
            "upgrade job does NOT run on workflow_dispatch with issue=76 "
            "(real upgrade dispatch broken)"
        )

    return errors


def validate_branch_name_pattern() -> list[str]:
    """Verify the branch-name construction follows bot/google-ads-vNN-issue-N."""
    errors: list[str] = []

    yaml_text = Path(
        ".github/workflows/google-ads-upgrade.yml"
    ).read_text()

    # The branch name is constructed as bot/google-ads-${TARGET_VERSION}-issue-${ISSUE_NUMBER}
    pattern = r"bot/google-ads-\$\{.*?TARGET_VERSION.*?\}-issue-\$\{.*?ISSUE_NUMBER.*?\}"
    if not re.search(pattern, yaml_text):
        errors.append(
            "Branch-name construction does not follow "
            "bot/google-ads-vNN-issue-N pattern"
        )

    return errors


def main() -> int:
    workflow_path = Path(".github/workflows/google-ads-upgrade.yml")
    if not workflow_path.exists():
        print(f"Error: {workflow_path} not found", file=sys.stderr)
        return 1

    yaml_text = workflow_path.read_text()
    conditions = extract_job_conditions(yaml_text)

    print("Job conditions:")
    for job, cond in conditions.items():
        print(f"  {job}: {cond}")

    all_errors: list[str] = []
    all_errors.extend(validate_mutual_exclusivity(conditions))
    all_errors.extend(validate_issue_zero_guard(conditions))
    all_errors.extend(validate_detector_runs_on_zero(conditions))
    all_errors.extend(validate_upgrade_runs_on_real_issue(conditions))
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