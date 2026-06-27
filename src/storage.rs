//! Persistence: loading and saving the ledger as JSON at ~/.ledger/data.json.
//!
//! This module is where `Result` and the `?` operator earn their keep: every
//! step (reading $HOME, reading the file, parsing JSON, writing the file) can
//! fail, and we want to bubble those failures up with a helpful message rather
//! than panic.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::models::Ledger;

/// Build the path to the data file: `~/.ledger/data.json`.
///
/// Ownership note: we construct and return an *owned* `PathBuf`, not a borrowed
/// `&Path`. A `&Path` would have to point at memory owned by something — but the
/// only candidate here is `path`, a local variable destroyed when this function
/// returns. The borrow checker rejects returning references to locals, so we
/// hand the caller an owned value it can keep.
fn data_path() -> Result<PathBuf> {
    // `std::env::var` returns `Result<String, VarError>`. `.context(...)` turns
    // any error into an anyhow error with this friendly message, and `?` then
    // returns early if it failed. On success, `?` unwraps the `String`.
    let home = std::env::var("HOME").context("could not read the $HOME environment variable")?;

    // `PathBuf` is the owned, growable cousin of `&Path` (much like `String`
    // is to `&str`). We push components onto it to build the full path.
    let mut path = PathBuf::from(home);
    path.push(".ledger");
    path.push("data.json");
    Ok(path)
}

/// Load the ledger from disk, returning an empty ledger if no file exists yet.
///
/// Returns an *owned* `Ledger`: the caller becomes the owner and is free to
/// mutate it (e.g. to add an entry) without affecting anything else.
pub fn load() -> Result<Ledger> {
    let path = data_path()?;

    // First run: there's no file yet. That's not an error — it's just an empty
    // ledger. Returning early keeps the happy path below simple.
    if !path.exists() {
        return Ok(Ledger::default());
    }

    // We pass `&path` (a borrow). `read_to_string` only needs to *look at* the
    // path to open the file; it doesn't need to own it. Lending a reference
    // means we still own `path` and can reuse it in the error message below.
    let text =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;

    // Parse the JSON text into a `Ledger`. The `: Ledger` annotation tells serde
    // which type to produce. `with_context` uses a closure so the (slightly
    // costly) string formatting only runs if there's actually an error.
    let ledger: Ledger = serde_json::from_str(&text)
        .with_context(|| format!("{} is not valid ledger JSON", path.display()))?;

    Ok(ledger)
}

/// Save the ledger to disk, creating the `~/.ledger` directory if needed.
///
/// Borrowing note: we take `&Ledger` (a *shared/immutable borrow*). Saving only
/// needs to *read* the data, not own or change it. By borrowing, the caller
/// keeps ownership of its `Ledger` and can keep using it after `save` returns.
pub fn save(ledger: &Ledger) -> Result<()> {
    let path = data_path()?;

    // `path.parent()` yields `Option<&Path>` for "~/.ledger". `if let Some(..)`
    // runs the body only when a parent exists, binding it to `parent`.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    // Serialize to pretty-printed JSON so the file is human-readable.
    let json =
        serde_json::to_string_pretty(ledger).context("failed to serialize ledger to JSON")?;

    fs::write(&path, json).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}
