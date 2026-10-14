# Heldnav Risk Engine (Rust)

Library and command-line desk for heldnav collateral lending. Markets and positions are included. The borrow check is the unfinished piece.

Stable Rust, edition 2021. No external dependencies.

## Run

```
cargo test
cargo run -- markets
cargo run -- positions
cargo run -- check 1000 500 200 7500
```

Market and position tests pass. `evaluate_borrow` tests fail until the task below is implemented. `heldnav check` reports `invalid ltv` until then; `markets` and `positions` already print the sample book.

## Layout

- `src/lib.rs` exposes the borrow check and the public types
- `src/market.rs` and `src/position.rs` are the sample book
- `src/bin/heldnav.rs` is the `heldnav` command

## Candidate task: complete `evaluate_borrow`

Implement only `evaluate_borrow` in `src/lib.rs`. Return a `RiskResult` with `max_debt`, `total_debt`, and `allowed`. Return `RiskError::InvalidLtv` when LTV is above 10,000. Use checked addition for total debt and return `RiskError::Overflow` if it overflows.

1. `max_debt = collateral * ltv_bps / 10_000`, using integer division.
2. `total_debt = existing_debt + additional`.
3. `allowed` is true when `total_debt <= max_debt`. Borrowing exactly at the limit is allowed.

Do not add dependencies or build a full-precision math library.
