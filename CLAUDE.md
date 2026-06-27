# CLAUDE.md

## What this is

`ledger` is a small, local-file finance/portfolio tracker CLI written in Rust. It
stores entries (amount, category, optional note) as JSON at `~/.ledger/data.json`
and prints per-category reports. It exists primarily as a hands-on project for
learning Rust fundamentals — ownership, borrowing, `Result`/`?`, enums, traits,
and modules — without async. The source is deliberately over-commented to explain
the *why* behind each ownership/borrowing/error decision.

## Stack

- **Rust** (stable) + **cargo**
- **clap** (derive) — CLI argument parsing
- **serde** + **serde_json** — JSON persistence
- **anyhow** — error handling

## Commands

| Task       | Command                                                              |
| ---------- | ------------------------------------------------------------------- |
| Run (dev)  | `cargo run -- <args>` — e.g. `cargo run -- add --amount 500 --category etf` |
| Typecheck  | `cargo check --all-targets`                                         |
| Lint       | `cargo clippy --all-targets -- -D warnings`                         |
| Format     | `cargo fmt --all` (check only: `cargo fmt --all --check`)           |
| Test       | `cargo test` (includes a CLI smoke test under `tests/`)            |
| Build      | `cargo build --release`                                            |

`/check` runs typecheck + lint + test together; `/make-pr` runs the full CI before
opening a PR (see [Local automation](#local-automation-not-committed)).

## Repository rules

**Commit voice.** First person, plain capitalized sentences. **One logical change
per commit.** Never narrate the process or who found what. **No AI/Claude
attribution anywhere** in commit messages or PR bodies — no `Co-Authored-By`, no
"Generated with…" lines.

**Branching & PRs.** `main` is branch-protected. Every change goes on a
`<type>/<slug>` branch (`feat/`, `fix/`, `refactor/`, `chore/`) → PR → green CI →
merge. **Never push directly to `main`.**

**Merging.** **Never merge a PR** — the maintainer merges on GitHub. Stop at:
"PR open, green CI, tested, ready for review." Merge permission is granted
per-PR and is never assumed to carry over to the next change.

**No local-notes references.** `notes/` is gitignored (local ADRs/scratch). Don't
reference `notes/` files or ADR numbers in commit messages or PR bodies — they'd
resolve to nothing in the public history that recruiters may read.

## Guardrails

- **Stay in the tree.** Only touch files inside this project (`ledger/`).
- **Confirm before irreversible or outward-facing actions** — deploys,
  force-push, deleting/overwriting files you didn't create, or sending data to
  external services. (Opening a PR for a change is part of the normal flow and is
  expected; *merging* is not.)
- **Read the real docs first.** Before writing code against a fast-moving
  dependency, read its *installed* docs/changelog — the current API may differ
  from training data. Heed deprecation notices (e.g. prefer maintained GitHub
  Actions over deprecated ones).

## Local automation (not committed)

These live under `.claude/` (gitignored) and load when Claude Code runs with
`ledger/` as its project root (launch Claude from inside `ledger/`):

- **Slash commands:** `/check` (typecheck + lint + test in parallel),
  `/feature <slug>` (start a `<type>/<slug>` branch), `/make-pr` (run full CI,
  then open the PR — never merge).
- **Hooks:** format-on-edit (rustfmt every `.rs` file edited), pre-PR CI gate
  (blocks opening a red PR), post-commit nudge to record a `notes/` entry.
