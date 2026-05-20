//! Fault-localization integration tests for rust-decimal.

use rust_decimal::etna::{
    property_abs_sub_difference, property_checked_div_no_panic, property_checked_ln_no_panic,
    property_from_i128_extremes, property_from_scientific_no_panic,
    property_is_integer_matches_string, property_round_dp_preserves_when_dp_exceeds_scale,
    property_scientific_fmt_roundtrip, PropertyResult,
};

fn to_opt(r: PropertyResult) -> Option<bool> {
    match r {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn property_abs_sub_difference_test(input: (i32, (usize, i32, usize))) -> Option<bool> {
    let (an, (asc, bn, bsc)) = input;
    to_opt(property_abs_sub_difference(an as i64, asc as u32, bn as i64, bsc as u32))
}

fn property_is_integer_matches_string_test(input: (i32, usize)) -> Option<bool> {
    let (n, s) = input;
    to_opt(property_is_integer_matches_string(n as i64, s as u32))
}

fn property_from_i128_extremes_test(choice: usize) -> Option<bool> {
    to_opt(property_from_i128_extremes(choice as u8))
}

fn property_round_dp_preserves_when_dp_exceeds_scale_test(input: (i32, usize, usize)) -> Option<bool> {
    let (n, s, extra) = input;
    to_opt(property_round_dp_preserves_when_dp_exceeds_scale(n as i64, s as u8, extra as u8))
}

fn property_checked_ln_no_panic_test(input: (i32, usize)) -> Option<bool> {
    let (n, s) = input;
    to_opt(property_checked_ln_no_panic(n as i64, s as u8))
}

fn property_scientific_fmt_roundtrip_test(input: (i32, usize)) -> Option<bool> {
    let (n, s) = input;
    to_opt(property_scientific_fmt_roundtrip(n as i64, s as u8))
}

fn property_from_scientific_no_panic_test(input: (usize, usize, usize)) -> Option<bool> {
    let (eb, bd, ne) = input;
    to_opt(property_from_scientific_no_panic(eb as u8, bd as u8, ne as u8))
}

fn property_checked_div_no_panic_test(input: ((i32, usize, i32), (usize, usize))) -> Option<bool> {
    let ((an, asc, bn), (bsc, mix)) = input;
    to_opt(property_checked_div_no_panic(an as i64, asc as u8, bn as i64, bsc as u8, mix as u8))
}

fn emit_locate_json(r: &crabcheck::profiling::LocateResult) {
    use crabcheck::quickcheck::ResultStatus;
    let status = match &r.run.status {
        ResultStatus::Failed { .. } => "Failed",
        ResultStatus::Finished => "Finished",
        ResultStatus::GaveUp => "GaveUp",
        ResultStatus::TimedOut => "TimedOut",
        ResultStatus::Aborted { .. } => "Aborted",
    };
    let top = if let Some(s) = r.top() {
        serde_json::json!({
            "rank": s.rank, "file": s.region.file, "function": s.region.function,
            "start_line": s.region.start_line, "end_line": s.region.end_line,
            "ochiai": s.region.suspiciousness.ochiai, "delta": s.region.delta,
            "panic_overlap": s.panic_overlap,
            "confidence": format!("{}", s.confidence),
            "confidence_rule": s.confidence_rule,
        })
    } else { serde_json::Value::Null };
    let top_5: Vec<_> = r.suspects.iter().take(5).map(|s| serde_json::json!({
        "rank": s.rank, "file": s.region.file, "function": s.region.function,
        "start_line": s.region.start_line, "end_line": s.region.end_line,
        "confidence": format!("{}", s.confidence),
        "confidence_rule": s.confidence_rule,
        "panic_overlap": s.panic_overlap,
    })).collect();
    let diags: Vec<_> = r.diagnostics.iter().map(|d| d.tag()).collect();
    let out = serde_json::json!({
        "status": status, "passed": r.run.passed, "discarded": r.run.discarded,
        "n_panics": r.n_panics, "n_suspects": r.suspects.len(),
        "top": top, "top_5": top_5, "diagnostics": diags,
    });
    println!("@@LOCATE@@ {}", out);
}

#[test]
fn locate_abs_sub_difference() {
    let report = crabcheck::quickcheck_with_locate!(property_abs_sub_difference_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_is_integer_matches_string() {
    let report = crabcheck::quickcheck_with_locate!(property_is_integer_matches_string_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_from_i128_extremes() {
    let report = crabcheck::quickcheck_with_locate!(property_from_i128_extremes_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_round_dp_preserves_when_dp_exceeds_scale() {
    let report = crabcheck::quickcheck_with_locate!(property_round_dp_preserves_when_dp_exceeds_scale_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_checked_ln_no_panic() {
    let report = crabcheck::quickcheck_with_locate!(property_checked_ln_no_panic_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_scientific_fmt_roundtrip() {
    let report = crabcheck::quickcheck_with_locate!(property_scientific_fmt_roundtrip_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_from_scientific_no_panic() {
    let report = crabcheck::quickcheck_with_locate!(property_from_scientific_no_panic_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}

#[test]
fn locate_checked_div_no_panic() {
    let report = crabcheck::quickcheck_with_locate!(property_checked_div_no_panic_test, "rust_decimal");
    eprintln!("{report}");
    emit_locate_json(&report);
}
