//! End-to-end smoke test: run the real compiled binary and check that an
//! `add` is persisted and shows up in `report`.
//!
//! Files in `tests/` are *integration tests* — each is compiled as its own
//! crate that links against `ledger` from the outside, exactly as a user would.
//! Cargo exposes the built binary's path via the `CARGO_BIN_EXE_<name>` env var.

use std::process::Command;

/// Run the `ledger` binary with `args`, pointing `$HOME` at an isolated temp
/// directory so the test never reads or clobbers the real `~/.ledger/data.json`.
fn run(home: &std::path::Path, args: &[&str]) -> std::process::Output {
    // `env!(...)` reads the var at *compile* time; Cargo guarantees it for tests.
    let bin = env!("CARGO_BIN_EXE_ledger");
    Command::new(bin)
        .args(args)
        .env("HOME", home) // overrides where storage::data_path() looks
        .output()
        .expect("failed to spawn the ledger binary")
}

#[test]
fn add_is_persisted_and_appears_in_report() {
    // A unique throwaway HOME for this test process.
    let home = std::env::temp_dir().join(format!("ledger-smoke-{}", std::process::id()));
    std::fs::create_dir_all(&home).expect("could not create temp HOME");

    let added = run(
        &home,
        &[
            "add",
            "--amount",
            "500",
            "--category",
            "etf",
            "--note",
            "IOZ",
        ],
    );
    assert!(added.status.success(), "add should exit 0");

    let report = run(&home, &["report"]);
    assert!(report.status.success(), "report should exit 0");

    let stdout = String::from_utf8_lossy(&report.stdout);
    assert!(
        stdout.contains("etf"),
        "report should list the etf category"
    );
    assert!(stdout.contains("500.00"), "report should total the amount");

    // Best-effort cleanup; ignore errors so a failed assert still reports clearly.
    std::fs::remove_dir_all(&home).ok();
}
