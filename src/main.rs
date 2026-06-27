//! ledger — a local-file finance/portfolio tracker CLI.
//!
//! Entry point and command dispatch. This file wires together the three modules:
//! - `models`  — the data types (Entry, Ledger)
//! - `storage` — load/save the ledger as JSON on disk
//! - `report`  — pure reporting logic (totals per category)

// Declaring the modules makes their code part of this crate. Each `mod foo;`
// pulls in `src/foo.rs`.
mod models;
mod report;
mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand};

use models::{Entry, Ledger};

/// Top-level CLI definition. `#[derive(Parser)]` makes clap generate an argument
/// parser from this struct: flags, `--help`, `--version`, and error messages.
#[derive(Parser)]
#[command(
    name = "ledger",
    version,
    about = "A local-file finance/portfolio tracker"
)]
struct Cli {
    /// The chosen subcommand (add / list / report).
    #[command(subcommand)]
    command: Command,
}

/// The set of subcommands. Each variant becomes `ledger <name>`, and its fields
/// become that subcommand's flags. This enum is a great example of Rust's sum
/// types: a `Command` is *exactly one* of these variants, and `match` below is
/// forced by the compiler to handle every one.
#[derive(Subcommand)]
enum Command {
    /// Add a new entry, e.g. `ledger add --amount 500 --category etf --note IOZ`.
    Add {
        /// The amount of money for this entry.
        #[arg(long)]
        amount: f64,

        /// The category bucket, e.g. "etf", "cash", "crypto".
        #[arg(long)]
        category: String,

        /// An optional note. Because the field is `Option<String>`, clap treats
        /// `--note` as optional and gives us `None` when it's omitted.
        #[arg(long)]
        note: Option<String>,
    },

    /// List every entry in the ledger.
    List,

    /// Show totals per category.
    Report,
}

/// `main` returns `Result<()>`. Returning an `Err` makes the process exit with a
/// non-zero status and prints the error (and its `.context(...)` chain) via
/// anyhow — so a failure deep inside `storage` surfaces as a readable message
/// instead of a panic or a silent wrong answer.
fn main() -> Result<()> {
    // Parse argv. On bad input clap prints help and exits before we get here.
    let cli = Cli::parse();

    // `match` *moves* the fields out of the parsed command into each arm. Since
    // we use each command at most once, moving (rather than borrowing) is the
    // simplest choice — there's no later use of `cli.command` to conflict with.
    match cli.command {
        Command::Add {
            amount,
            category,
            note,
        } => cmd_add(amount, category, note),
        Command::List => cmd_list(),
        Command::Report => cmd_report(),
    }
}

/// Handle `ledger add`.
fn cmd_add(amount: f64, category: String, note: Option<String>) -> Result<()> {
    // `load` hands us an *owned* `Ledger`. We bind it as `mut` because we're
    // about to mutate it by pushing a new entry. `?` propagates any load error.
    let mut ledger: Ledger = storage::load()?;

    // Compute the next id: the max existing id + 1 (or 1 for an empty ledger).
    // `.iter()` borrows the vec, `.map(...)` projects each entry to its id, and
    // `.max()` returns `Option<u64>` — `None` when there are no entries, which
    // `.unwrap_or(0)` turns into 0.
    let next_id = ledger.entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;

    // Build the new entry. The `amount`, `category`, and `note` values we were
    // given are *moved* into the struct's fields — `category`/`note` are owned
    // `String`s, so this is a move (no copy), and we can't use those bindings
    // by those names afterwards.
    let entry = Entry {
        id: next_id,
        amount,
        category,
        note,
    };

    // Print a confirmation *before* the push: the next line moves `entry` into
    // the vector, after which the `entry` binding is no longer usable.
    println!(
        "Added entry #{}: {:.2} [{}]",
        entry.id, entry.amount, entry.category
    );

    // `push` *moves* `entry` into the vec; its data now lives inside `ledger`.
    ledger.entries.push(entry);

    // Persist. We lend `save` a reference (`&ledger`); we keep ownership.
    storage::save(&ledger)?;
    Ok(())
}

/// Handle `ledger list`.
fn cmd_list() -> Result<()> {
    let ledger = storage::load()?;

    if ledger.entries.is_empty() {
        println!("No entries yet. Add one with: ledger add --amount <N> --category <NAME>");
        return Ok(());
    }

    // `&ledger.entries` borrows the vector, so the loop iterates over `&Entry`
    // references. We're only reading, so borrowing (not consuming) is right.
    for entry in &ledger.entries {
        // `entry.note` is `Option<String>`. `.as_deref()` borrows it as
        // `Option<&str>` (without moving the String out), and `.unwrap_or("")`
        // substitutes an empty string when the note is `None`.
        let note = entry.note.as_deref().unwrap_or("");
        println!(
            "#{:<3} {:>12.2}  {:<10} {}",
            entry.id, entry.amount, entry.category, note
        );
    }
    Ok(())
}

/// Handle `ledger report`.
fn cmd_report() -> Result<()> {
    let ledger = storage::load()?;

    // Hand the report function a borrowed slice of the entries. `report` owns
    // nothing here; it just reads and returns a fresh map of totals.
    let totals = report::totals_by_category(&ledger.entries);

    if totals.is_empty() {
        println!("No entries to report.");
        return Ok(());
    }

    println!("{:<14} {:>12}", "CATEGORY", "TOTAL");

    let mut grand_total = 0.0;
    // Iterating a `&BTreeMap` yields `(&String, &f64)` pairs, already sorted by
    // key. We dereference `total` (a `&f64`) with `*` when adding it up.
    for (category, total) in &totals {
        println!("{:<14} {:>12.2}", category, total);
        grand_total += *total;
    }

    println!("{:<14} {:>12.2}", "TOTAL", grand_total);
    Ok(())
}
