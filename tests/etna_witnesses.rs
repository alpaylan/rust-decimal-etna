// Deterministic witness tests for ETNA variants.
//
// Each `witness_<name>_case_<tag>` passes on the base commit and fails under
// the corresponding `etna/<variant>` branch. Witnesses call `property_<name>`
// directly with frozen inputs; they do not touch framework machinery.

use rust_decimal::etna::{
    property_abs_sub_difference, property_checked_ln_no_panic, property_from_i128_extremes,
    property_from_i128_no_panic, property_is_integer_matches_string,
    property_round_dp_preserves_when_dp_exceeds_scale, PropertyResult,
};

fn expect_pass(r: PropertyResult, what: &str) {
    match r {
        PropertyResult::Pass => (),
        PropertyResult::Fail(m) => panic!("{what}: property failed: {m}"),
        PropertyResult::Discard => panic!("{what}: unexpected discard"),
    }
}

// ---------------------------------------------------------------------------
// Variant: abs_sub_returns_abs_dd711db_1
//
// Mutation reverts the `abs_sub` fix so the branch `self > other` returns
// `self.abs()` instead of `self - other`. For `a=123, b=122` the correct
// answer is `1`, not `123`.
#[test]
fn witness_abs_sub_difference_case_123_122() {
    expect_pass(
        property_abs_sub_difference(123, 0, 122, 0),
        "abs_sub(123, 122) must equal 1",
    );
}

#[test]
fn witness_abs_sub_difference_case_neg123_neg124() {
    // -123 - (-124) = 1, but |-123| = 123.
    expect_pass(
        property_abs_sub_difference(-123, 0, -124, 0),
        "abs_sub(-123, -124) must equal 1",
    );
}

// ---------------------------------------------------------------------------
// Variant: is_integer_scale_decrement_951512d_1
//
// Mutation reverts `scale -= 9` back to `scale -= 10` in `is_integer`,
// causing the division-by-10^9 loop to skip a digit. A number like
// `0.400000000` (scale 9) is correctly flagged non-integer, but a number with
// a trailing non-zero far past the 9-digit chunk boundary (scale > 9) misses
// the remainder check and returns `true`.
#[test]
fn witness_is_integer_matches_string_case_scale_10_nonzero_tail() {
    // 0.4000000000 -> scale = 10, trailing `4` lands in the first chunk
    // under the buggy `scale -= 10`. Correct behaviour returns false.
    expect_pass(
        property_is_integer_matches_string(4_000_000_000, 10),
        "is_integer(0.4000000000) must be false",
    );
}

#[test]
fn witness_is_integer_matches_string_case_scale_19_nonzero_tail() {
    // 0.4000000000000000001, scale 19.
    expect_pass(
        property_is_integer_matches_string(4_000_000_000_000_000_001, 19),
        "is_integer(0.4000000000000000001) must be false",
    );
}

// ---------------------------------------------------------------------------
// Variant: from_i128_negation_overflow_6f7d295_1
//
// Mutation reverts `n.unsigned_abs()` to `-n as u128`. On `i128::MIN`, the
// negation overflows and panics in debug builds (UB in release). The property
// asserts that `from_i128` never panics on any i128 value.
#[test]
fn witness_from_i128_no_panic_case_i128_min() {
    expect_pass(
        property_from_i128_extremes(0),
        "from_i128(i128::MIN) must not panic",
    );
}

#[test]
fn witness_from_i128_no_panic_case_i128_min_plus_1() {
    expect_pass(
        property_from_i128_extremes(1),
        "from_i128(i128::MIN + 1) must not panic",
    );
}

// Small i64 value round-trips; kept as a sanity check for the positive case.
#[test]
fn witness_from_i128_no_panic_case_small_positive() {
    expect_pass(
        property_from_i128_no_panic(42),
        "from_i128(42) must not panic",
    );
}

// ---------------------------------------------------------------------------
// Variant: round_dp_early_return_reorder_c205456_1
//
// Mutation reverts the ordering of the two early-return blocks inside
// `round_dp_with_strategy`: the zero short-circuit happens before the
// "already shorter than dp" check, so rounding `0` to a larger dp than its
// current scale returns `Decimal { scale: dp }` instead of preserving the
// input. A decimal with scale=28 rounded to dp=32 should retain scale 28.
#[test]
fn witness_round_dp_preserves_case_zero_scale_28_dp_32() {
    // scale=28, dp = 28 + 4 = 32. The original bug caused a panic because
    // dp > MAX_SCALE (28); the reordered fix short-circuits first.
    expect_pass(
        property_round_dp_preserves_when_dp_exceeds_scale(0, 28, 4),
        "round_dp(0e-28, 32) must preserve the input",
    );
}

#[test]
fn witness_round_dp_preserves_case_small_scale_4_dp_10() {
    expect_pass(
        property_round_dp_preserves_when_dp_exceeds_scale(1234, 4, 6),
        "round_dp(0.1234, 10) must return 0.1234 with scale 4",
    );
}

// ---------------------------------------------------------------------------
// Variant: checked_ln_zero_panic_092fdf8_1 (maths feature only)
//
// Mutation reverts the `!self.is_zero()` guard inside `checked_ln`, causing
// `Decimal::ZERO.checked_ln()` to fall through into the fast path and recurse
// until the stack overflows. The property asserts that `checked_ln` never
// panics.
#[test]
#[cfg(feature = "maths")]
fn witness_checked_ln_no_panic_case_zero() {
    expect_pass(
        property_checked_ln_no_panic(0, 0),
        "Decimal::ZERO.checked_ln() must not panic",
    );
}

#[test]
#[cfg(feature = "maths")]
fn witness_checked_ln_no_panic_case_one() {
    expect_pass(
        property_checked_ln_no_panic(1, 0),
        "Decimal::ONE.checked_ln() must not panic",
    );
}
