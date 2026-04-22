# rust-decimal — Injected Bugs

Total mutations: 8

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `abs_sub_returns_abs_dd711db_1` | `abs_sub_returns_abs` | `src/decimal.rs` | `patch` | `dd711dbee5081a243e8eae5b314ecd8b0505f3ef` |
| 2 | `checked_ln_zero_panic_092fdf8_1` | `checked_ln_zero_panic` | `src/ops/wide.rs` | `patch` | `092fdf8c8def5e2eb4ca5624ebfae731c3c40407` |
| 3 | `div_remainder_overflow_a231fbf_1` | `div_remainder_overflow` | `src/ops/div.rs` | `patch` | `a231fbf12c543534c6b300648e8c3a8b467968cf` |
| 4 | `from_i128_negation_overflow_6f7d295_1` | `from_i128_negation_overflow` | `src/decimal.rs` | `patch` | `6f7d295cd82571429064132265f907131841c60f` |
| 5 | `is_integer_scale_decrement_951512d_1` | `is_integer_scale_decrement` | `src/decimal.rs` | `patch` | `951512d003a4b65724d6074a15630534994b40e6` |
| 6 | `round_dp_early_return_reorder_c205456_1` | `round_dp_early_return_reorder` | `src/decimal.rs` | `patch` | `c205456643a5f831396c2e98caa7fc91f96363bc` |
| 7 | `scientific_fmt_zero_d0f2a64_1` | `scientific_fmt_zero` | `src/str.rs` | `patch` | `d0f2a64eb22391188b8984f973b3d4abf5720fd5` |
| 8 | `scientific_scale_overflow_722af9f_1` | `scientific_scale_overflow` | `src/decimal.rs` | `patch` | `722af9f2c2829273f9c1b09e2fd989378f795cbc` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `abs_sub_returns_abs_dd711db_1` | `AbsSubDifference` | `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124` |
| `checked_ln_zero_panic_092fdf8_1` | `CheckedLnNoPanic` | `witness_checked_ln_no_panic_case_zero` |
| `div_remainder_overflow_a231fbf_1` | `CheckedDivNoPanic` | `witness_checked_div_no_panic_case_issue_392` |
| `from_i128_negation_overflow_6f7d295_1` | `FromI128Extremes` | `witness_from_i128_no_panic_case_i128_min` |
| `is_integer_scale_decrement_951512d_1` | `IsIntegerMatchesString` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high` |
| `round_dp_early_return_reorder_c205456_1` | `RoundDpPreservesWhenDpExceedsScale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15` |
| `scientific_fmt_zero_d0f2a64_1` | `ScientificFmtRoundtrip` | `witness_scientific_fmt_roundtrip_case_zero`, `witness_scientific_fmt_roundtrip_case_zero_scale_5` |
| `scientific_scale_overflow_722af9f_1` | `FromScientificNoPanic` | `witness_from_scientific_no_panic_case_neg_u32_max`, `witness_from_scientific_no_panic_case_neg_u32_max_digit7` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `AbsSubDifference` | ✓ | ✓ | ✓ | ✓ |
| `CheckedLnNoPanic` | ✓ | ✓ | ✓ | ✓ |
| `CheckedDivNoPanic` | ✓ | ✓ | ✓ | ✓ |
| `FromI128Extremes` | ✓ | ✓ | ✓ | ✓ |
| `IsIntegerMatchesString` | ✓ | ✓ | ✓ | ✓ |
| `RoundDpPreservesWhenDpExceedsScale` | ✓ | ✓ | ✓ | ✓ |
| `ScientificFmtRoundtrip` | ✓ | ✓ | ✓ | ✓ |
| `FromScientificNoPanic` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. abs_sub_returns_abs

- **Variant**: `abs_sub_returns_abs_dd711db_1`
- **Location**: `src/decimal.rs`
- **Property**: `AbsSubDifference`
- **Witness(es)**:
  - `witness_abs_sub_difference_case_123_122`
  - `witness_abs_sub_difference_case_neg123_neg124`
- **Source**: Re-implement abs_sub function to match num-trait 0.1 behavior
  > `<Decimal as num_traits::Signed>::abs_sub` returned `self.abs()` on the `self > other` branch instead of the arithmetic difference `self - other`, so `abs_sub(123, 122)` came out as `123` rather than `1`. The fix rewrites the impl to `max(a - b, 0)`, matching `num-traits 0.1` semantics.
- **Fix commit**: `dd711dbee5081a243e8eae5b314ecd8b0505f3ef` — Re-implement abs_sub function to match num-trait 0.1 behavior
- **Invariant violated**: `num_traits::Signed::abs_sub(a, b) == max(a - b, 0)`. When `a > b` the result is the arithmetic difference, not `|a|`.
- **How the mutation triggers**: `<Decimal as Signed>::abs_sub` returns `self.abs()` on the `self > other` branch. For `abs_sub(123, 122)` the correct answer is `1`, but the mutation returns `123`.

### 2. checked_ln_zero_panic

- **Variant**: `checked_ln_zero_panic_092fdf8_1`
- **Location**: `src/ops/wide.rs`
- **Property**: `CheckedLnNoPanic`
- **Witness(es)**:
  - `witness_checked_ln_no_panic_case_zero`
- **Source**: Avoid panic in checked_ln for zero input
  > `ops::wide::ln_wide` guarded against negatives but not against `Decimal::ZERO`; on zero, the range-reduction loop multiplied by `E_INVERSE` without bound, overflowing the scale and panicking. The fix extends the guard to `|| value.is_zero()` so `checked_ln(0)` returns `None` instead.
- **Fix commit**: `092fdf8c8def5e2eb4ca5624ebfae731c3c40407` — Avoid panic in checked_ln for zero input
- **Invariant violated**: `Decimal::checked_ln()` must not panic for any input — zero/negative inputs are expected to return `None`.
- **How the mutation triggers**: `ops::wide::ln_wide` drops the `|| value.is_zero()` clause from its early-return guard. On `Decimal::ZERO` the range-reduction loop multiplies by `E_INVERSE` indefinitely, overflowing the scale and panicking.

### 3. div_remainder_overflow

- **Variant**: `div_remainder_overflow_a231fbf_1`
- **Location**: `src/ops/div.rs`
- **Property**: `CheckedDivNoPanic`
- **Witness(es)**:
  - `witness_checked_div_no_panic_case_issue_392`
- **Source**: fix: wrap remainder increment in Buf16 long division
  > `Buf16`'s 128/64 long-division carry branch used `remainder += 1`, which could overflow `u64` for adversarial dividend/divisor pairs (issue #392) and panicked under overflow-checks. The fix swaps to `remainder = remainder.wrapping_add(1)` — the wrap is provably safe for the division algorithm but the compiler can't see it.
- **Fix commit**: `a231fbf12c543534c6b300648e8c3a8b467968cf` — fix: wrap remainder increment in Buf16 long division
- **Invariant violated**: `Decimal::checked_div(a, b)` must never panic; it returns `None` when the division would overflow.
- **How the mutation triggers**: `Buf16`'s 128/64 long-division carry branch reverts to plain `remainder += 1`. For the issue-392 reproducer (`-79228157791897.854723898738431 / 184512476.73336922111`), the remainder saturates `u64` and the unchecked add panics under overflow-checks.

### 4. from_i128_negation_overflow

- **Variant**: `from_i128_negation_overflow_6f7d295_1`
- **Location**: `src/decimal.rs`
- **Property**: `FromI128Extremes`
- **Witness(es)**:
  - `witness_from_i128_no_panic_case_i128_min`
- **Source**: Fix overflow negating i128::MIN in FromPrimitive
  > `<Decimal as FromPrimitive>::from_i128` negated the input via `-n as u128`, which panics on `i128::MIN` under overflow-checks (no non-negative representation exists for the minimum). The fix uses `n.unsigned_abs()`, which returns `2^127` for `i128::MIN` without going through a signed negation.
- **Fix commit**: `6f7d295cd82571429064132265f907131841c60f` — Fix overflow negating i128::MIN in FromPrimitive
- **Invariant violated**: `<Decimal as FromPrimitive>::from_i128(n)` never panics; it returns `None` when `n` is too large to represent.
- **How the mutation triggers**: The negative branch uses `-n as u128` instead of `n.unsigned_abs()`. For `n == i128::MIN` the literal negation overflows, panicking in debug builds.

### 5. is_integer_scale_decrement

- **Variant**: `is_integer_scale_decrement_951512d_1`
- **Location**: `src/decimal.rs`
- **Property**: `IsIntegerMatchesString`
- **Witness(es)**:
  - `witness_is_integer_matches_string_case_scale_10_nonzero_tail`
  - `witness_is_integer_matches_string_case_scale_19_nonzero_high`
- **Source**: Fix scale decrement in Decimal::is_integer
  > `Decimal::is_integer` decremented `scale -= 10` per `div_by_u32(bits, 10^9)` iteration even though each iteration consumed only 9 digits of scale. For values whose low 9 digits were zero (e.g. `0.4000000000`) the loop exited early and reported "integer" when it wasn't. The fix uses `scale -= 9` so the accounting matches.
- **Fix commit**: `951512d003a4b65724d6074a15630534994b40e6` — Fix scale decrement in Decimal::is_integer
- **Invariant violated**: `Decimal::is_integer()` agrees with the string form: true iff there are no non-zero digits after the decimal point.
- **How the mutation triggers**: `Decimal::is_integer` decrements `scale -= 10` per `div_by_u32(bits, 10^9)` iteration even though only 9 digits of scale were actually consumed. For values whose low 9 digits are zero (e.g. `0.4000000000` with scale 10, or `0.4000000000000000000` with scale 19), the loop exits early reporting "integer".

### 6. round_dp_early_return_reorder

- **Variant**: `round_dp_early_return_reorder_c205456_1`
- **Location**: `src/decimal.rs`
- **Property**: `RoundDpPreservesWhenDpExceedsScale`
- **Witness(es)**:
  - `witness_round_dp_preserves_case_zero_scale_28_dp_32`
  - `witness_round_dp_preserves_case_zero_scale_5_dp_15`
- **Source**: Reorder round_dp early returns to preserve scale when dp is larger
  > `round_dp_with_strategy` checked the `value.is_zero()` short-circuit before the `old_scale <= dp` guard, so rounding zero to a larger scale replaced the original scale with `dp` (e.g. `Decimal::new(0, 5).round_dp(15)` became scale 15). The fix reorders the early-return blocks so the scale-preserving branch runs first.
- **Fix commit**: `c205456643a5f831396c2e98caa7fc91f96363bc` — Reorder round_dp early returns to preserve scale when dp is larger
- **Invariant violated**: When `dp >= old_scale`, `round_dp_with_strategy` must return `*self` unchanged (preserving the original scale).
- **How the mutation triggers**: The zero short-circuit runs before the `old_scale <= dp` check, so zero values are rebuilt with `scale = dp` instead of retaining the original scale. E.g. `Decimal::new(0, 5).round_dp(15)` ends up with scale 15 instead of 5.

### 7. scientific_fmt_zero

- **Variant**: `scientific_fmt_zero_d0f2a64_1`
- **Location**: `src/str.rs`
- **Property**: `ScientificFmtRoundtrip`
- **Witness(es)**:
  - `witness_scientific_fmt_roundtrip_case_zero`
  - `witness_scientific_fmt_roundtrip_case_zero_scale_5`
- **Source**: fix: scientific formatting of 0
  > `fmt_scientific_notation` iterated over the mantissa digits, which is empty when the value is zero, so `format!("{:e}", Decimal::ZERO)` produced `"e0"` — rejected when re-parsed through `from_scientific`. The fix adds an `if value.is_zero() { return f.write_str("0e0"); }` short-circuit.
- **Fix commit**: `d0f2a64eb22391188b8984f973b3d4abf5720fd5` — fix: scientific formatting of 0
- **Invariant violated**: `format!("{:e}", d)` must roundtrip through `Decimal::from_scientific_exact` for any Decimal.
- **How the mutation triggers**: `fmt_scientific_notation` drops the `if value.is_zero() { return f.write_str("0e0"); }` short-circuit, so zero's mantissa digits iterate zero times and the buffer is just `"e0"`, which fails to re-parse.

### 8. scientific_scale_overflow

- **Variant**: `scientific_scale_overflow_722af9f_1`
- **Location**: `src/decimal.rs`
- **Property**: `FromScientificNoPanic`
- **Witness(es)**:
  - `witness_from_scientific_no_panic_case_neg_u32_max`
  - `witness_from_scientific_no_panic_case_neg_u32_max_digit7`
- **Source**: fix: bounds check in Decimal::from_scientific on exponent
  > `Decimal::from_scientific_exact` added the parsed exponent onto the current scale without bounds-checking, so inputs like `"1e-4294967295"` overflowed `u32` and panicked under overflow-checks. The fix adds `if exp > Self::MAX_SCALE { return Err(...) }` guards on both sign branches before the add.
- **Fix commit**: `722af9f2c2829273f9c1b09e2fd989378f795cbc` — fix: bounds check in Decimal::from_scientific on exponent
- **Invariant violated**: `Decimal::from_scientific_exact(s)` must never panic; it returns `Err(ScaleExceedsMaximumPrecision)` for out-of-range exponents.
- **How the mutation triggers**: Both `if exp > Self::MAX_SCALE { return Err(...); }` guards are removed. For `"d.d e-<u32::MAX>"` the unchecked `current_scale + exp` overflows `u32` under overflow-checks, panicking.
