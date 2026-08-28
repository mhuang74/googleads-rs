# Domain Docs

Single-context layout:

- `CONTEXT.md` at repo root — the project glossary
- `docs/adr/` — architecture decision records (created lazily)

Consumer rules:

- Before working in an area, read `CONTEXT.md` and use its vocabulary in tickets, specs, and code.
- Terms conflicting with the glossary should be surfaced, not silently adopted.
- ADRs record decisions that are hard to reverse, surprising without context, and the result of a real trade-off.