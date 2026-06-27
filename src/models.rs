//! Data models for the ledger.
//!
//! These are plain data structs. The interesting Rust ideas here are:
//! - `#[derive(...)]`, which auto-generates trait implementations.
//! - `String` vs `&str` (owned vs borrowed text).
//! - `Option<T>`, Rust's null-free way to say "maybe a value".

use serde::{Deserialize, Serialize};

/// A single ledger entry — one financial transaction or holding.
///
/// The `#[derive(...)]` line asks the compiler to write several trait impls
/// for us:
/// - `Serialize` / `Deserialize`: serde uses these to convert an `Entry` to and
///   from JSON. Without them, `serde_json` wouldn't know how to handle `Entry`.
/// - `Clone`: lets us make an explicit, owned *copy* of an `Entry` when we need
///   one in more than one place at once.
/// - `Debug`: enables `{:?}` formatting, which we lean on in tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// A simple incrementing id so each entry is addressable.
    pub id: u64,

    /// The amount of money. We use `f64` for simplicity while learning.
    /// (A production finance app would store integer *cents* to avoid binary
    /// floating-point rounding error — noted here as a deliberate trade-off.)
    pub amount: f64,

    /// We own the category text. `String` is a growable, heap-allocated,
    /// owned string — the `Entry` is responsible for freeing it when dropped.
    /// We use `String` rather than `&str` because an `Entry` must *own* its
    /// data to be stored in a `Vec` and serialized independently of any
    /// borrowed source it came from.
    pub category: String,

    /// `Option<String>` models "there may or may not be a note".
    /// `Some(text)` carries a note; `None` means there isn't one. Rust has no
    /// null, so this is how absence is represented — and the compiler forces us
    /// to handle the `None` case explicitly wherever we read it.
    pub note: Option<String>,
}

/// The whole on-disk document: just a list of entries, wrapped in a struct so
/// the JSON has a stable top-level shape we can extend later (e.g. add a
/// `version` field) without breaking existing files.
///
/// `Default` is derived so `Ledger::default()` gives us an empty ledger — handy
/// when the data file doesn't exist yet. A derived `Default` fills every field
/// with *its* default, and `Vec`'s default is the empty vector.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ledger {
    pub entries: Vec<Entry>,
}
