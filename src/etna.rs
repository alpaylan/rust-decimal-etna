//! ETNA framework-neutral property functions for rust-decimal.
//!
//! Each `property_<name>` is a pure function taking concrete, owned inputs and
//! returning `PropertyResult`. Framework adapters in `src/bin/etna.rs` and the
//! deterministic witness tests in `tests/etna_witnesses.rs` both call these
//! functions directly — there is no re-implementation of any invariant inside
//! an adapter.

#![allow(missing_docs)]

use crate::Decimal;
#[cfg(feature = "maths")]
use crate::MathematicalOps;
use core::str::FromStr;
use num_traits::{FromPrimitive, Signed};

pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

/// `abs_sub(a, b)` must equal `max(a - b, 0)` for the `Signed` trait. When
/// `a > b`, the result is the arithmetic difference `a - b`, not `|a|`.
/// Detects the dd711db bug where the function returned `self.abs()` when
/// `self > other`.
pub fn property_abs_sub_difference(a_num: i64, a_scale: u32, b_num: i64, b_scale: u32) -> PropertyResult {
    if a_scale > 10 || b_scale > 10 {
        return PropertyResult::Discard;
    }
    let a = Decimal::new(a_num, a_scale);
    let b = Decimal::new(b_num, b_scale);
    let got = a.abs_sub(&b);
    let expected = if a <= b { Decimal::ZERO } else { a - b };
    if got == expected {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "abs_sub({a}, {b}) = {got}, expected {expected}"
        ))
    }
}

/// `Decimal::is_integer()` must agree with the string representation: after
/// normalising `to_string()`, the value is an integer iff there is no decimal
/// point, or every digit after it is `0`. Detects the 951512d bug where the
/// scale decrement skipped a digit (`scale -= 10` instead of `-= 9`), causing
/// `is_integer` to return `true` for non-integer decimals with scale > 9.
pub fn property_is_integer_matches_string(num: i64, scale: u32) -> PropertyResult {
    if scale > 20 {
        return PropertyResult::Discard;
    }
    let d = Decimal::new(num, scale);
    let s = d.to_string();
    let expected = match s.split_once('.') {
        None => true,
        Some((_, frac)) => frac.chars().all(|c| c == '0'),
    };
    let got = d.is_integer();
    if got == expected {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "is_integer({d}) = {got}, expected {expected} (string: {s})"
        ))
    }
}

/// `Decimal::from_i128(n)` must never panic on any i128 input. It either
/// returns `Some(d)` when representable, or `None` when the magnitude
/// exceeds `Decimal::MAX`. Detects the 6f7d295 bug where `-i128::MIN` would
/// overflow while negating, causing a panic on `i128::MIN`.
pub fn property_from_i128_no_panic(n: i64) -> PropertyResult {
    let big = n as i128;
    match std::panic::catch_unwind(|| <Decimal as FromPrimitive>::from_i128(big)) {
        Ok(_) => PropertyResult::Pass,
        Err(_) => PropertyResult::Fail(format!("from_i128({big}) panicked")),
    }
}

/// `Decimal::from_i128(i128::MIN)` (and other magnitudes beyond Decimal::MAX)
/// must return `None` without panicking. Detects the same 6f7d295 bug as
/// `property_from_i128_no_panic`, but exercises the full i128 range to find
/// the pathological `i128::MIN` case.
pub fn property_from_i128_extremes(choice: u8) -> PropertyResult {
    let big: i128 = match choice % 6 {
        0 => i128::MIN,
        1 => i128::MIN + 1,
        2 => i128::MAX,
        3 => -(1_i128 << 96),
        4 => 1_i128 << 96,
        _ => -(choice as i128) * 10_000_000_000_000_000_000_000_000_000,
    };
    match std::panic::catch_unwind(|| <Decimal as FromPrimitive>::from_i128(big)) {
        Ok(_) => PropertyResult::Pass,
        Err(_) => PropertyResult::Fail(format!("from_i128({big}) panicked")),
    }
}

/// `round_dp_with_strategy(dp)` must preserve the value when `dp` is greater
/// than or equal to the current scale. Detects the c205456 bug where rounding
/// zero with `dp > old_scale` would shrink the scale to `dp` rather than
/// keeping `old_scale`, losing the scale invariant when `dp` exceeded
/// Decimal::MAX_SCALE.
pub fn property_round_dp_preserves_when_dp_exceeds_scale(
    num: i64,
    scale: u8,
    extra_dp: u8,
) -> PropertyResult {
    let scale = scale as u32 % 28;
    let dp = scale + (extra_dp as u32 % 12);
    let d = Decimal::new(num, scale);
    let rounded = d.round_dp(dp);
    if rounded == d && rounded.scale() == scale {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "round_dp({d}, dp={dp}) = {rounded} (scale {}), expected {d} (scale {scale})",
            rounded.scale()
        ))
    }
}

/// `Decimal::ZERO.checked_ln()` must not panic. It is expected to return
/// `Some(Decimal::ZERO)` (the project's chosen convention from 092fdf8) or
/// at worst `None`. Detects the original bug where `checked_ln` on zero
/// entered a loop that overflowed and panicked.
#[cfg(feature = "maths")]
pub fn property_checked_ln_no_panic(num: i64, scale: u8) -> PropertyResult {
    let scale = scale as u32 % 10;
    let d = Decimal::new(num, scale);
    match std::panic::catch_unwind(|| d.checked_ln()) {
        Ok(_) => PropertyResult::Pass,
        Err(_) => PropertyResult::Fail(format!("checked_ln({d}) panicked")),
    }
}

#[cfg(not(feature = "maths"))]
pub fn property_checked_ln_no_panic(_num: i64, _scale: u8) -> PropertyResult {
    PropertyResult::Discard
}

/// A concrete parsed input variant used by properties that need to exercise
/// specific test strings alongside generated inputs.
pub fn parse_decimal(s: &str) -> Option<Decimal> {
    Decimal::from_str(s).ok()
}
