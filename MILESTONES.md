# Milestones

## Milestone 1 — Core CLI (local JSON store + report)

- [x] add
- [x] list
- [x] persistence
- [x] report
- [x] tests
- [x] error-handling polish — every fallible step returns `Result` with
      `.context(...)`; `main` surfaces the full error chain via anyhow

## Ideas for later

- [ ] `remove` / `edit` commands
- [ ] dates/timestamps on entries
- [ ] filter `list`/`report` by category
- [ ] store amounts as integer cents to avoid float rounding
- [ ] `--json` output mode for scripting
