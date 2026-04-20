# rust-decimal — Injected Bugs

Total mutations: 5

All variants are patch-based; apply the listed patch to a clean HEAD to reproduce the buggy build. Each `etna/<variant>` branch is a pre-applied snapshot.

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `abs_sub` returns `self.abs()` instead of `self - other` | `abs_sub_returns_abs_dd711db_1` | `patches/abs_sub_returns_abs_dd711db_1.patch` | patch | `dd711dbee5081a243e8eae5b314ecd8b0505f3ef` |
| 2 | `is_integer` off-by-one in scale chunk decrement | `is_integer_scale_decrement_951512d_1` | `patches/is_integer_scale_decrement_951512d_1.patch` | patch | `951512d003a4b65724d6074a15630534994b40e6` |
| 3 | `from_i128` negation overflow on `i128::MIN` | `from_i128_negation_overflow_6f7d295_1` | `patches/from_i128_negation_overflow_6f7d295_1.patch` | patch | `6f7d295cd82571429064132265f907131841c60f` |
| 4 | `round_dp` early-return block ordering | `round_dp_early_return_reorder_c205456_1` | `patches/round_dp_early_return_reorder_c205456_1.patch` | patch | `c205456643a5f831396c2e98caa7fc91f96363bc` |
| 5 | `checked_ln` panics on zero (maths feature) | `checked_ln_zero_panic_092fdf8_1` | `patches/checked_ln_zero_panic_092fdf8_1.patch` | patch | `092fdf8c8def5e2eb4ca5624ebfae731c3c40407` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `abs_sub_returns_abs_dd711db_1` | `property_abs_sub_difference` | `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124` |
| `is_integer_scale_decrement_951512d_1` | `property_is_integer_matches_string` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high` |
| `from_i128_negation_overflow_6f7d295_1` | `property_from_i128_extremes` | `witness_from_i128_no_panic_case_i128_min` |
| `round_dp_early_return_reorder_c205456_1` | `property_round_dp_preserves_when_dp_exceeds_scale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15` |
| `checked_ln_zero_panic_092fdf8_1` | `property_checked_ln_no_panic` | `witness_checked_ln_no_panic_case_zero` |

## Framework Coverage

| Property | etna | proptest | quickcheck | crabcheck | hegel |
|----------|:----:|:--------:|:----------:|:---------:|:-----:|
| `property_abs_sub_difference` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_is_integer_matches_string` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_from_i128_extremes` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_round_dp_preserves_when_dp_exceeds_scale` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_checked_ln_no_panic` | ✓ | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. `abs_sub` returns `self.abs()` instead of `self - other`

- **Variant**: `abs_sub_returns_abs_dd711db_1`
- **Location**: `patches/abs_sub_returns_abs_dd711db_1.patch` (applies to `src/decimal.rs`)
- **Property**: `property_abs_sub_difference`
- **Witness(es)**: `witness_abs_sub_difference_case_123_122`, `witness_abs_sub_difference_case_neg123_neg124`
- **Fix commit**: `dd711dbee5081a243e8eae5b314ecd8b0505f3ef` — `Re-implement abs_sub function to match num-trait 0.1 behavior`
- **Invariant violated**: `num_traits::Signed::abs_sub(a, b) == max(a - b, 0)`. When `a > b` the result is the arithmetic difference, not `|a|`.
- **How the mutation triggers**: `<Decimal as Signed>::abs_sub` returns `self.abs()` on the `self > other` branch. For `abs_sub(123, 122)` the correct answer is `1`, but the mutation returns `123`.

### 2. `is_integer` off-by-one in scale chunk decrement

- **Variant**: `is_integer_scale_decrement_951512d_1`
- **Location**: `patches/is_integer_scale_decrement_951512d_1.patch` (applies to `src/decimal.rs`)
- **Property**: `property_is_integer_matches_string`
- **Witness(es)**: `witness_is_integer_matches_string_case_scale_10_nonzero_tail`, `witness_is_integer_matches_string_case_scale_19_nonzero_high`
- **Fix commit**: `951512d003a4b65724d6074a15630534994b40e6` — `Fix scale decrement in Decimal::is_integer`
- **Invariant violated**: `Decimal::is_integer()` agrees with the string form: true iff there are no non-zero digits after the decimal point.
- **How the mutation triggers**: `Decimal::is_integer` decrements `scale -= 10` per `div_by_u32(bits, 10^9)` iteration even though only 9 digits of scale were actually consumed. For values whose low 9 digits are zero (e.g. `0.4000000000` with scale 10, or `0.4000000000000000000` with scale 19), the loop exits early reporting "integer".

### 3. `from_i128` negation overflow on `i128::MIN`

- **Variant**: `from_i128_negation_overflow_6f7d295_1`
- **Location**: `patches/from_i128_negation_overflow_6f7d295_1.patch` (applies to `src/decimal.rs`)
- **Property**: `property_from_i128_extremes`
- **Witness(es)**: `witness_from_i128_no_panic_case_i128_min`
- **Fix commit**: `6f7d295cd82571429064132265f907131841c60f` — `Fix overflow negating i128::MIN in FromPrimitive`
- **Invariant violated**: `<Decimal as FromPrimitive>::from_i128(n)` never panics; it returns `None` when `n` is too large to represent.
- **How the mutation triggers**: The negative branch uses `-n as u128` instead of `n.unsigned_abs()`. For `n == i128::MIN` the literal negation overflows, panicking in debug builds.

### 4. `round_dp` early-return block ordering

- **Variant**: `round_dp_early_return_reorder_c205456_1`
- **Location**: `patches/round_dp_early_return_reorder_c205456_1.patch` (applies to `src/decimal.rs`)
- **Property**: `property_round_dp_preserves_when_dp_exceeds_scale`
- **Witness(es)**: `witness_round_dp_preserves_case_zero_scale_28_dp_32`, `witness_round_dp_preserves_case_zero_scale_5_dp_15`
- **Fix commit**: `c205456643a5f831396c2e98caa7fc91f96363bc` — `Reorder round_dp early returns to preserve scale when dp is larger`
- **Invariant violated**: When `dp >= old_scale`, `round_dp_with_strategy` must return `*self` unchanged (preserving the original scale).
- **How the mutation triggers**: The zero short-circuit runs before the `old_scale <= dp` check, so zero values are rebuilt with `scale = dp` instead of retaining the original scale. E.g. `Decimal::new(0, 5).round_dp(15)` ends up with scale 15 instead of 5.

### 5. `checked_ln` panics on zero (maths feature)

- **Variant**: `checked_ln_zero_panic_092fdf8_1`
- **Location**: `patches/checked_ln_zero_panic_092fdf8_1.patch` (applies to `src/ops/wide.rs`)
- **Property**: `property_checked_ln_no_panic`
- **Witness(es)**: `witness_checked_ln_no_panic_case_zero`
- **Fix commit**: `092fdf8c8def5e2eb4ca5624ebfae731c3c40407` — `Avoid panic in checked_ln for zero input`
- **Invariant violated**: `Decimal::checked_ln()` must not panic for any input — zero/negative inputs are expected to return `None`.
- **How the mutation triggers**: `ops::wide::ln_wide` drops the `|| value.is_zero()` clause from its early-return guard. On `Decimal::ZERO` the range-reduction loop multiplies by `E_INVERSE` indefinitely, overflowing the scale and panicking.
