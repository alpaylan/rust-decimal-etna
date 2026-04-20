# rust-decimal — ETNA Tasks

Total tasks: 32

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task: the command executes the framework-specific adapter against the buggy variant branch and should report a counterexample.

Run against a variant by first checking out its branch (`git checkout etna/<variant>`) or applying its patch on a clean tree (`git apply patches/<variant>.patch`). The `maths`-gated variant requires `--features maths`.

## Task Index

| Task | Variant | Framework | Property | Witness(es) | Command |
|------|---------|-----------|----------|-------------|---------|
| 001 | `abs_sub_returns_abs_dd711db_1` | proptest | `property_abs_sub_difference` | `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124` | `cargo run --release --bin etna -- proptest AbsSubDifference` |
| 002 | `abs_sub_returns_abs_dd711db_1` | quickcheck | `property_abs_sub_difference` | `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124` | `cargo run --release --bin etna -- quickcheck AbsSubDifference` |
| 003 | `abs_sub_returns_abs_dd711db_1` | crabcheck | `property_abs_sub_difference` | `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124` | `cargo run --release --bin etna -- crabcheck AbsSubDifference` |
| 004 | `abs_sub_returns_abs_dd711db_1` | hegel | `property_abs_sub_difference` | `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124` | `cargo run --release --bin etna -- hegel AbsSubDifference` |
| 005 | `is_integer_scale_decrement_951512d_1` | proptest | `property_is_integer_matches_string` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high` | `cargo run --release --bin etna -- proptest IsIntegerMatchesString` |
| 006 | `is_integer_scale_decrement_951512d_1` | quickcheck | `property_is_integer_matches_string` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high` | `cargo run --release --bin etna -- quickcheck IsIntegerMatchesString` |
| 007 | `is_integer_scale_decrement_951512d_1` | crabcheck | `property_is_integer_matches_string` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high` | `cargo run --release --bin etna -- crabcheck IsIntegerMatchesString` |
| 008 | `is_integer_scale_decrement_951512d_1` | hegel | `property_is_integer_matches_string` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high` | `cargo run --release --bin etna -- hegel IsIntegerMatchesString` |
| 009 | `from_i128_negation_overflow_6f7d295_1` | proptest | `property_from_i128_extremes` | `witness_from_i128_no_panic_case_i128_min` | `cargo run --release --bin etna -- proptest FromI128Extremes` |
| 010 | `from_i128_negation_overflow_6f7d295_1` | quickcheck | `property_from_i128_extremes` | `witness_from_i128_no_panic_case_i128_min` | `cargo run --release --bin etna -- quickcheck FromI128Extremes` |
| 011 | `from_i128_negation_overflow_6f7d295_1` | crabcheck | `property_from_i128_extremes` | `witness_from_i128_no_panic_case_i128_min` | `cargo run --release --bin etna -- crabcheck FromI128Extremes` |
| 012 | `from_i128_negation_overflow_6f7d295_1` | hegel | `property_from_i128_extremes` | `witness_from_i128_no_panic_case_i128_min` | `cargo run --release --bin etna -- hegel FromI128Extremes` |
| 013 | `round_dp_early_return_reorder_c205456_1` | proptest | `property_round_dp_preserves_when_dp_exceeds_scale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15` | `cargo run --release --bin etna -- proptest RoundDpPreserves` |
| 014 | `round_dp_early_return_reorder_c205456_1` | quickcheck | `property_round_dp_preserves_when_dp_exceeds_scale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15` | `cargo run --release --bin etna -- quickcheck RoundDpPreserves` |
| 015 | `round_dp_early_return_reorder_c205456_1` | crabcheck | `property_round_dp_preserves_when_dp_exceeds_scale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15` | `cargo run --release --bin etna -- crabcheck RoundDpPreserves` |
| 016 | `round_dp_early_return_reorder_c205456_1` | hegel | `property_round_dp_preserves_when_dp_exceeds_scale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15` | `cargo run --release --bin etna -- hegel RoundDpPreserves` |
| 017 | `checked_ln_zero_panic_092fdf8_1` | proptest | `property_checked_ln_no_panic` | `witness_checked_ln_no_panic_case_zero` | `cargo run --release --features maths --bin etna -- proptest CheckedLnNoPanic` |
| 018 | `checked_ln_zero_panic_092fdf8_1` | quickcheck | `property_checked_ln_no_panic` | `witness_checked_ln_no_panic_case_zero` | `cargo run --release --features maths --bin etna -- quickcheck CheckedLnNoPanic` |
| 019 | `checked_ln_zero_panic_092fdf8_1` | crabcheck | `property_checked_ln_no_panic` | `witness_checked_ln_no_panic_case_zero` | `cargo run --release --features maths --bin etna -- crabcheck CheckedLnNoPanic` |
| 020 | `checked_ln_zero_panic_092fdf8_1` | hegel | `property_checked_ln_no_panic` | `witness_checked_ln_no_panic_case_zero` | `cargo run --release --features maths --bin etna -- hegel CheckedLnNoPanic` |
| 021 | `scientific_fmt_zero_d0f2a64_1` | proptest | `property_scientific_fmt_roundtrip` | `witness_scientific_fmt_roundtrip_case_zero`, `witness_scientific_fmt_roundtrip_case_zero_scale_5` | `cargo run --release --bin etna -- proptest ScientificFmtRoundtrip` |
| 022 | `scientific_fmt_zero_d0f2a64_1` | quickcheck | `property_scientific_fmt_roundtrip` | `witness_scientific_fmt_roundtrip_case_zero`, `witness_scientific_fmt_roundtrip_case_zero_scale_5` | `cargo run --release --bin etna -- quickcheck ScientificFmtRoundtrip` |
| 023 | `scientific_fmt_zero_d0f2a64_1` | crabcheck | `property_scientific_fmt_roundtrip` | `witness_scientific_fmt_roundtrip_case_zero`, `witness_scientific_fmt_roundtrip_case_zero_scale_5` | `cargo run --release --bin etna -- crabcheck ScientificFmtRoundtrip` |
| 024 | `scientific_fmt_zero_d0f2a64_1` | hegel | `property_scientific_fmt_roundtrip` | `witness_scientific_fmt_roundtrip_case_zero`, `witness_scientific_fmt_roundtrip_case_zero_scale_5` | `cargo run --release --bin etna -- hegel ScientificFmtRoundtrip` |
| 025 | `scientific_scale_overflow_722af9f_1` | proptest | `property_from_scientific_no_panic` | `witness_from_scientific_no_panic_case_neg_u32_max`, `witness_from_scientific_no_panic_case_neg_u32_max_digit7` | `cargo run --release --bin etna -- proptest FromScientificNoPanic` |
| 026 | `scientific_scale_overflow_722af9f_1` | quickcheck | `property_from_scientific_no_panic` | `witness_from_scientific_no_panic_case_neg_u32_max`, `witness_from_scientific_no_panic_case_neg_u32_max_digit7` | `cargo run --release --bin etna -- quickcheck FromScientificNoPanic` |
| 027 | `scientific_scale_overflow_722af9f_1` | crabcheck | `property_from_scientific_no_panic` | `witness_from_scientific_no_panic_case_neg_u32_max`, `witness_from_scientific_no_panic_case_neg_u32_max_digit7` | `cargo run --release --bin etna -- crabcheck FromScientificNoPanic` |
| 028 | `scientific_scale_overflow_722af9f_1` | hegel | `property_from_scientific_no_panic` | `witness_from_scientific_no_panic_case_neg_u32_max`, `witness_from_scientific_no_panic_case_neg_u32_max_digit7` | `cargo run --release --bin etna -- hegel FromScientificNoPanic` |
| 029 | `div_remainder_overflow_a231fbf_1` | proptest | `property_checked_div_no_panic` | `witness_checked_div_no_panic_case_issue_392` | `cargo run --release --bin etna -- proptest CheckedDivNoPanic` |
| 030 | `div_remainder_overflow_a231fbf_1` | quickcheck | `property_checked_div_no_panic` | `witness_checked_div_no_panic_case_issue_392` | `cargo run --release --bin etna -- quickcheck CheckedDivNoPanic` |
| 031 | `div_remainder_overflow_a231fbf_1` | crabcheck | `property_checked_div_no_panic` | `witness_checked_div_no_panic_case_issue_392` | `cargo run --release --bin etna -- crabcheck CheckedDivNoPanic` |
| 032 | `div_remainder_overflow_a231fbf_1` | hegel | `property_checked_div_no_panic` | `witness_checked_div_no_panic_case_issue_392` | `cargo run --release --bin etna -- hegel CheckedDivNoPanic` |

## Witness catalog

Each witness is a deterministic concrete test in `tests/etna_witnesses.rs`. Base build: passes. Variant-active build: fails.

- `witness_abs_sub_difference_case_123_122` — `property_abs_sub_difference(123, 0, 122, 0)` → `Pass`. Under `abs_sub_returns_abs_dd711db_1` the fix returns `1` whereas the mutation returns `123`.
- `witness_abs_sub_difference_case_neg123_neg124` — `property_abs_sub_difference(-123, 0, -124, 0)` → `Pass`. Fix returns `1`; mutation returns `|-123| = 123`.
- `witness_is_integer_matches_string_case_scale_10_nonzero_tail` — `property_is_integer_matches_string(4_000_000_000, 10)` → `Pass`. Under `is_integer_scale_decrement_951512d_1` the buggy `scale -= 10` exits after one `div 10^9` iteration and returns `true` for the non-integer `0.4000000000`.
- `witness_is_integer_matches_string_case_scale_19_nonzero_high` — `property_is_integer_matches_string(4_000_000_000_000_000_000, 19)` → `Pass`. Same bug at scale 19: buggy path returns `true`; fix catches the leading `4` in the final `scale=1` step.
- `witness_from_i128_no_panic_case_i128_min` — `property_from_i128_extremes(0)` → `Pass`. Under `from_i128_negation_overflow_6f7d295_1` `-n as u128` panics in debug builds when `n == i128::MIN`; the fix uses `n.unsigned_abs()`.
- `witness_round_dp_preserves_case_zero_scale_28_dp_32` — `property_round_dp_preserves_when_dp_exceeds_scale(0, 28, 4)` → `Pass`. Effective scale=0, dp=4. Under `round_dp_early_return_reorder_c205456_1` the zero short-circuit fires before the dp check and rebuilds the value with `scale=dp`.
- `witness_round_dp_preserves_case_zero_scale_5_dp_15` — `property_round_dp_preserves_when_dp_exceeds_scale(0, 5, 10)` → `Pass`. Effective scale=5, dp=15. Same reorder bug, different scale.
- `witness_checked_ln_no_panic_case_zero` — `property_checked_ln_no_panic(0, 0)` → `Pass`. Under `checked_ln_zero_panic_092fdf8_1` the `value.is_zero()` guard is missing from `ln_wide`; `ZERO.checked_ln()` enters an unbounded scale-overflowing loop and panics.
- `witness_scientific_fmt_roundtrip_case_zero` — `property_scientific_fmt_roundtrip(0, 0)` → `Pass`. Under `scientific_fmt_zero_d0f2a64_1` `format!("{:e}", Decimal::ZERO)` yields `"e0"` (no mantissa) and fails to re-parse through `from_scientific_exact`.
- `witness_scientific_fmt_roundtrip_case_zero_scale_5` — `property_scientific_fmt_roundtrip(0, 5)` → `Pass`. Same mutation with scale=5; the mantissa loop still emits nothing without the zero short-circuit.
- `witness_from_scientific_no_panic_case_neg_u32_max` — `property_from_scientific_no_panic(0, 1, 1)` → `Pass`. Parses `"1.1e-<u32::MAX>"`. Under `scientific_scale_overflow_722af9f_1` the `current_scale + exp` addition overflows `u32` and panics; the fix rejects exponents above `MAX_SCALE` with an error.
- `witness_from_scientific_no_panic_case_neg_u32_max_digit7` — `property_from_scientific_no_panic(0, 7, 1)` → `Pass`. Same overflow path with base `"7.7"` to confirm the bug isn't literal-specific.
- `witness_checked_div_no_panic_case_issue_392` — `property_checked_div_no_panic(0, 0, 0, 0, 0)` → `Pass`. Exact issue-392 reproducer (`-79228157791897.854723898738431 / 184512476.73336922111`). Under `div_remainder_overflow_a231fbf_1` the unchecked `remainder += 1` in `Buf16`'s 128/64 carry branch panics when the remainder rolls through `u64::MAX`.
