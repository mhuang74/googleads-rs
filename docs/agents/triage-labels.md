# Triage Labels

Canonical triage roles map to these GitHub label strings, each label equal to its name:

| Role | Label |
|---|---|
| Ready for agent | `ready-for-agent` |
| In progress | `in-progress` |
| Upgrade failed | `api-upgrade-failed` |

The `api upgrade bot` marker label identifies issues created by the automated upgrade pipeline.

Per ADR 0001, the triage vocabulary is three state labels plus the `api upgrade bot` marker. Dropped labels (`needs-triage`, `needs-info`, `ready-for-human`, `wontfix`) are not used.

Labels are created on first use if missing.