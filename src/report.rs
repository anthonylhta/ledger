//! Reporting logic: pure functions over entries.
//!
//! This module is deliberately free of I/O. Keeping the *computation* separate
//! from *loading/printing* is what makes it easy to unit-test (see the `tests`
//! module at the bottom) — the tests build entries in memory and never touch
//! the filesystem.

use std::collections::BTreeMap;

use crate::models::Entry;

/// Sum the amounts in each category.
///
/// Borrowing note: the parameter is `&[Entry]` — a *slice*, i.e. a borrowed,
/// read-only view over a run of entries. We don't take `Vec<Entry>` (which would
/// move/consume the caller's vector) because totalling only needs to *read* the
/// entries. A `&Vec<Entry>` would also work, but `&[Entry]` is the idiomatic,
/// more general choice: it accepts a `Vec`, an array, or any other slice.
///
/// We return a `BTreeMap`, which keeps its keys in sorted order. That makes the
/// report output deterministic — the same input always prints in the same order,
/// which is friendly to both humans and assertions in tests.
pub fn totals_by_category(entries: &[Entry]) -> BTreeMap<String, f64> {
    let mut totals: BTreeMap<String, f64> = BTreeMap::new();

    // Iterating over `&[Entry]` yields `&Entry` — each `entry` is a *borrow* of
    // an element, not a moved-out copy. The original slice stays intact.
    for entry in entries {
        // The map must *own* its `String` keys; it can't borrow them from
        // `entries`, which the caller still owns and may drop independently.
        // So we `.clone()` the category to hand the map its own copy.
        //
        // `.entry(key)` finds the slot for that key (creating it if missing).
        // `.or_insert(0.0)` returns a *mutable reference* to the running total,
        // inserting 0.0 first if this category is new.
        let running_total = totals.entry(entry.category.clone()).or_insert(0.0);

        // `*running_total` dereferences the mutable reference so we add into the
        // value living inside the map.
        *running_total += entry.amount;
    }

    totals
}

// `#[cfg(test)]` means this module is compiled *only* when running `cargo test`,
// so test code never bloats the shipped binary.
#[cfg(test)]
mod tests {
    // Pull in everything from the parent module (notably `totals_by_category`).
    use super::*;

    /// Small helper to build an `Entry` tersely in tests. `&str` in, owned
    /// `String` stored — `.to_string()` allocates the owned copy the field needs.
    fn entry(amount: f64, category: &str) -> Entry {
        Entry {
            id: 0,
            amount,
            category: category.to_string(),
            note: None,
        }
    }

    #[test]
    fn totals_of_empty_ledger_is_empty() {
        let entries: Vec<Entry> = vec![];
        // `&entries` borrows the Vec as a slice to match `&[Entry]`.
        let totals = totals_by_category(&entries);
        assert!(totals.is_empty());
    }

    #[test]
    fn sums_amounts_within_each_category() {
        let entries = vec![
            entry(500.0, "etf"),
            entry(250.0, "etf"),
            entry(100.0, "cash"),
        ];
        let totals = totals_by_category(&entries);

        // `.get` returns `Option<&f64>`, so we compare against `Some(&750.0)`.
        assert_eq!(totals.get("etf"), Some(&750.0));
        assert_eq!(totals.get("cash"), Some(&100.0));
        assert_eq!(totals.len(), 2);
    }

    #[test]
    fn negative_amounts_reduce_a_category_total() {
        // e.g. a buy of 500 followed by a sell/withdrawal of 200.
        let entries = vec![entry(500.0, "etf"), entry(-200.0, "etf")];
        let totals = totals_by_category(&entries);
        assert_eq!(totals.get("etf"), Some(&300.0));
    }
}
