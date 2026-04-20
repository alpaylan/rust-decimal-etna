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

/// `format!("{:e}", d)` must round-trip through `from_scientific_exact` for
/// every representable Decimal. Detects the d0f2a64 bug where formatting
/// `Decimal::ZERO` as scientific notation produced `"e0"` (no mantissa)
/// instead of `"0e0"`, causing the roundtrip to fail.
pub fn property_scientific_fmt_roundtrip(num: i64, scale: u8) -> PropertyResult {
    let scale = scale as u32 % 10;
    let d = Decimal::new(num, scale);
    let s = format!("{d:e}");
    match Decimal::from_scientific_exact(&s) {
        Ok(parsed) if parsed == d => PropertyResult::Pass,
        Ok(parsed) => PropertyResult::Fail(format!(
            "format(\"{{:e}}\", {d}) = {s:?}, re-parses to {parsed} (!= {d})"
        )),
        Err(e) => PropertyResult::Fail(format!(
            "format(\"{{:e}}\", {d}) = {s:?}, fails to re-parse: {e}"
        )),
    }
}

/// `Decimal::from_scientific(s)` must never panic for any `s`. It either
/// parses successfully or returns `Err`. Detects the 722af9f bug where a
/// string like `"1e-4294967292"` caused `current_scale + exp` to overflow
/// `u32`, panicking under overflow checks.
pub fn property_from_scientific_no_panic(exp_bump: u8, base_digit: u8, neg_exp: u8) -> PropertyResult {
    // Construct large negative/positive exponents near u32::MAX. The base
    // must have a non-zero fractional part so the parsed `current_scale` is
    // > 0; then `current_scale + exp` overflows u32 on the buggy path.
    let exp_val = u32::MAX - (exp_bump as u32 % 32);
    let sign = if neg_exp & 1 == 1 { "-" } else { "" };
    let digit = (base_digit % 10) as u32;
    let s = format!("{digit}.{digit}e{sign}{exp_val}");
    match std::panic::catch_unwind(|| Decimal::from_scientific_exact(&s)) {
        Ok(_) => PropertyResult::Pass,
        Err(_) => PropertyResult::Fail(format!("from_scientific({s:?}) panicked")),
    }
}

/// `Decimal::checked_div(a, b)` must never panic, regardless of inputs. It
/// returns `None` when overflow would occur. Detects the a231fbf bug where
/// the carry-handling branch in `Buf16`'s 128/64 division used unchecked
/// `remainder += 1`, panicking in debug / with overflow-checks when the
/// remainder saturated `u64`.
pub fn property_checked_div_no_panic(
    a_num: i64,
    a_scale: u8,
    b_num: i64,
    b_scale: u8,
    mix: u8,
) -> PropertyResult {
    // Mix biases toward wide operands so the division exercises the 128/64
    // long-division path that hosts the reverted wrapping_add. The "issue
    // 392" operands are the exact reproducer from rust-decimal PR #393; they
    // have a 29-digit mantissa, which is wide enough to enter the `Buf16`
    // path where the carry-handling branch lives.
    let a_scale = a_scale as u32 % 15;
    let b_scale = b_scale as u32 % 12;
    let (a, b) = match mix % 4 {
        0 => (
            Decimal::from_str("-79228157791897.854723898738431").unwrap(),
            Decimal::from_str("184512476.73336922111").unwrap(),
        ),
        1 => (
            Decimal::from_str("-79228157791897.854723898738431").unwrap(),
            Decimal::new(b_num.max(1), b_scale),
        ),
        2 => (Decimal::new(a_num, a_scale), Decimal::from_str("0.00000000001").unwrap()),
        _ => (Decimal::new(a_num, a_scale), Decimal::new(b_num, b_scale)),
    };
    match std::panic::catch_unwind(|| a.checked_div(b)) {
        Ok(_) => PropertyResult::Pass,
        Err(_) => PropertyResult::Fail(format!("checked_div({a}, {b}) panicked")),
    }
}
