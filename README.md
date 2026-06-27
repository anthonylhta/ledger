# ledger

A small, local-file **finance / portfolio tracker** CLI written in Rust.

It was built as a hands-on way to learn Rust fundamentals (ownership, borrowing,
`Result`/`?`, enums, traits, modules) without the extra complexity of async. The
source is heavily commented to explain *why* each ownership/borrowing/error
decision was made.

Entries are stored as JSON in `~/.ledger/data.json`.

## Stack

- **Rust** + **cargo**
- [`clap`](https://docs.rs/clap) (derive) — argument parsing
- [`serde`](https://serde.rs) + [`serde_json`](https://docs.rs/serde_json) — JSON (de)serialization
- [`anyhow`](https://docs.rs/anyhow) — ergonomic application error handling

## Install Rust (if needed)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

## Build & test

```sh
cargo build      # compile
cargo test       # run the report unit tests
cargo run -- --help
```

## Usage

```sh
# Add entries (note is optional)
ledger add --amount 500 --category etf --note "IOZ"
ledger add --amount 250 --category etf
ledger add --amount 1000 --category cash

# List every entry
ledger list

# Totals per category
ledger report
```

Running via cargo during development (note the `--` separating cargo's args from
the program's args):

```sh
cargo run -- add --amount 500 --category etf --note "IOZ"
cargo run -- list
cargo run -- report
```

Example `report` output:

```
CATEGORY              TOTAL
cash               1000.00
etf                 750.00
TOTAL              1750.00
```

## Data file

All state lives in a single JSON file at `~/.ledger/data.json`, created on first
`add`. It's plain text and safe to read or hand-edit:

```json
{
  "entries": [
    { "id": 1, "amount": 500.0, "category": "etf", "note": "IOZ" }
  ]
}
```

## Project structure

| File             | Responsibility                                              |
| ---------------- | ----------------------------------------------------------- |
| `src/main.rs`    | CLI definition (clap) + command dispatch                    |
| `src/models.rs`  | `Entry` / `Ledger` data types + serde derives               |
| `src/storage.rs` | Load/save the JSON file at `~/.ledger/data.json`            |
| `src/report.rs`  | Pure `totals_by_category` logic + unit tests                |

## Rust concepts this project demonstrates

- **Ownership & moves** — `add` moves parsed flags into an `Entry`, then moves
  the `Entry` into the `Vec`.
- **Borrowing** — `save(&Ledger)` and `totals_by_category(&[Entry])` only read,
  so they borrow instead of taking ownership.
- **`Result` & `?`** — every fallible step (file I/O, JSON parsing) returns
  `Result`; `?` propagates errors up to `main`, which prints them via anyhow.
- **`Option<T>`** — the optional `--note` is modelled as `Option<String>`,
  forcing the absent case to be handled explicitly.
- **Enums + exhaustive `match`** — the subcommand is a sum type the compiler
  forces us to handle completely.
- **Traits via `derive`** — `Serialize`, `Deserialize`, `Debug`, `Clone`,
  `Default` are generated for us.
- **Modules** — the program is split into focused, testable units.

## License

[MIT](LICENSE)
