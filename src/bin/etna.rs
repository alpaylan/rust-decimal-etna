// ETNA workload runner for rust-decimal.
//
// Usage: cargo run --release --bin etna -- <tool> <property>
//   tool:     etna | proptest | quickcheck | crabcheck | hegel
//   property: AbsSubDifference | IsIntegerMatchesString | FromI128NoPanic
//             | FromI128Extremes | RoundDpPreserves | CheckedLnNoPanic
//             | ScientificFmtRoundtrip | FromScientificNoPanic
//             | CheckedDivNoPanic | All
//
// Emits exactly one JSON line per invocation. Always exits 0.

use crabcheck::quickcheck as crabcheck_qc;
use hegel::{generators as hgen, Hegel, Settings as HegelSettings};
use proptest_etna as proptest;
use proptest_etna::prelude::*;
use proptest_etna::test_runner::{Config as ProptestConfig, TestCaseError, TestError};
use quickcheck::{QuickCheck, ResultStatus, TestResult};
use rust_decimal::etna::{
    property_abs_sub_difference, property_checked_div_no_panic, property_checked_ln_no_panic,
    property_from_i128_extremes, property_from_i128_no_panic, property_from_scientific_no_panic,
    property_is_integer_matches_string, property_round_dp_preserves_when_dp_exceeds_scale,
    property_scientific_fmt_roundtrip, PropertyResult,
};
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

impl Metrics {
    fn combine(self, other: Metrics) -> Metrics {
        Metrics {
            inputs: self.inputs + other.inputs,
            elapsed_us: self.elapsed_us + other.elapsed_us,
        }
    }
}

type Outcome = (Result<(), String>, Metrics);

fn to_err(r: PropertyResult) -> Result<(), String> {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

const ALL_PROPERTIES: &[&str] = &[
    "AbsSubDifference",
    "IsIntegerMatchesString",
    "FromI128NoPanic",
    "FromI128Extremes",
    "RoundDpPreserves",
    "CheckedLnNoPanic",
    "ScientificFmtRoundtrip",
    "FromScientificNoPanic",
    "CheckedDivNoPanic",
];

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    let mut first_err: Result<(), String> = Ok(());
    for p in ALL_PROPERTIES {
        let (r, m) = f(p);
        total = total.combine(m);
        if r.is_err() && first_err.is_ok() {
            first_err = r;
        }
    }
    (first_err, total)
}

// ============================================================================
// etna (witness replay) runner — calls property functions with frozen inputs.
// ============================================================================

fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let result = match property {
        "AbsSubDifference" => to_err(property_abs_sub_difference(123, 0, 122, 0)),
        "IsIntegerMatchesString" => {
            to_err(property_is_integer_matches_string(4_000_000_000, 10))
        }
        "FromI128NoPanic" => to_err(property_from_i128_no_panic(42)),
        "FromI128Extremes" => to_err(property_from_i128_extremes(0)),
        "RoundDpPreserves" => to_err(property_round_dp_preserves_when_dp_exceeds_scale(0, 28, 4)),
        "CheckedLnNoPanic" => to_err(property_checked_ln_no_panic(0, 0)),
        "ScientificFmtRoundtrip" => to_err(property_scientific_fmt_roundtrip(0, 0)),
        "FromScientificNoPanic" => to_err(property_from_scientific_no_panic(0, 1, 1)),
        "CheckedDivNoPanic" => to_err(property_checked_div_no_panic(-79228157791897, 15, 1, 0, 0)),
        _ => return (Err(format!("Unknown property: {property}")), Metrics::default()),
    };
    let elapsed_us = t0.elapsed().as_micros();
    (result, Metrics { inputs: 1, elapsed_us })
}

// ============================================================================
// Proptest runner
// ============================================================================

fn pt_i64() -> BoxedStrategy<i64> {
    prop_oneof![
        Just(0_i64),
        Just(1_i64),
        Just(-1_i64),
        Just(i64::MIN),
        Just(i64::MAX),
        Just(4_000_000_000_i64),
        Just(4_000_000_000_000_000_001_i64),
        Just(123_i64),
        Just(-123_i64),
        any::<i64>(),
    ]
    .boxed()
}

fn pt_scale_small() -> BoxedStrategy<u32> {
    (0u32..=10).boxed()
}

fn pt_scale_large() -> BoxedStrategy<u32> {
    (0u32..=20).boxed()
}

fn map_err_testerror<T>(r: Result<T, TestError<impl std::fmt::Debug>>) -> Result<T, String> {
    r.map_err(|e| match e {
        TestError::Fail(reason, _) => reason.to_string(),
        other => format!("{other:?}"),
    })
}

fn run_proptest_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let mut runner = proptest::test_runner::TestRunner::new(ProptestConfig::default());
    let c = counter.clone();
    let result: Result<(), String> = match property {
        "AbsSubDifference" => {
            let strat = (pt_i64(), pt_scale_small(), pt_i64(), pt_scale_small());
            map_err_testerror(runner.run(&strat, move |(an, asc, bn, bsc)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_abs_sub_difference(an, asc, bn, bsc)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(format!(
                        "({an} {asc} {bn} {bsc})"
                    ))),
                }
            }))
        }
        "IsIntegerMatchesString" => {
            let strat = (pt_i64(), pt_scale_large());
            map_err_testerror(runner.run(&strat, move |(n, s)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_is_integer_matches_string(n, s)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({n} {s})")))
                    }
                }
            }))
        }
        "FromI128NoPanic" => {
            let strat = pt_i64();
            map_err_testerror(runner.run(&strat, move |n| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_from_i128_no_panic(n)));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({n})")))
                    }
                }
            }))
        }
        "FromI128Extremes" => {
            let strat = any::<u8>();
            map_err_testerror(runner.run(&strat, move |ch| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_from_i128_extremes(ch)));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({ch})")))
                    }
                }
            }))
        }
        "RoundDpPreserves" => {
            let strat = (pt_i64(), any::<u8>(), any::<u8>());
            map_err_testerror(runner.run(&strat, move |(n, s, extra)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_round_dp_preserves_when_dp_exceeds_scale(n, s, extra)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({n} {s} {extra})")))
                    }
                }
            }))
        }
        "CheckedLnNoPanic" => {
            let strat = (pt_i64(), any::<u8>());
            map_err_testerror(runner.run(&strat, move |(n, s)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_checked_ln_no_panic(n, s)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({n} {s})")))
                    }
                }
            }))
        }
        "ScientificFmtRoundtrip" => {
            let strat = (pt_i64(), any::<u8>());
            map_err_testerror(runner.run(&strat, move |(n, s)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_scientific_fmt_roundtrip(n, s)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({n} {s})")))
                    }
                }
            }))
        }
        "FromScientificNoPanic" => {
            let strat = (any::<u8>(), any::<u8>(), any::<u8>());
            map_err_testerror(runner.run(&strat, move |(eb, bd, ne)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_from_scientific_no_panic(eb, bd, ne)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({eb} {bd} {ne})")))
                    }
                }
            }))
        }
        "CheckedDivNoPanic" => {
            let strat = (pt_i64(), any::<u8>(), pt_i64(), any::<u8>(), any::<u8>());
            map_err_testerror(runner.run(&strat, move |(an, asc, bn, bsc, mix)| {
                c.fetch_add(1, Ordering::Relaxed);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_checked_div_no_panic(an, asc, bn, bsc, mix)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                    Ok(PropertyResult::Fail(_)) | Err(_) => {
                        Err(TestCaseError::fail(format!("({an} {asc} {bn} {bsc} {mix})")))
                    }
                }
            }))
        }
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = counter.load(Ordering::Relaxed);
    (result, Metrics { inputs, elapsed_us })
}

// ============================================================================
// QuickCheck runner (fork with `etna` feature, fn-pointer API).
// ============================================================================

static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn qc_abs_sub_difference(an: i64, asc: u32, bn: i64, bsc: u32) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_abs_sub_difference(an, asc, bn, bsc) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_is_integer_matches_string(n: i64, s: u32) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_is_integer_matches_string(n, s) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_from_i128_no_panic(n: i64) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_from_i128_no_panic(n) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_from_i128_extremes(ch: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_from_i128_extremes(ch) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_round_dp_preserves(n: i64, s: u8, extra: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_round_dp_preserves_when_dp_exceeds_scale(n, s, extra) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_checked_ln_no_panic(n: i64, s: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_checked_ln_no_panic(n, s) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_scientific_fmt_roundtrip(n: i64, s: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_scientific_fmt_roundtrip(n, s) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_from_scientific_no_panic(eb: u8, bd: u8, ne: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_from_scientific_no_panic(eb, bd, ne) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_checked_div_no_panic(an: i64, asc: u8, bn: i64, bsc: u8, mix: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_checked_div_no_panic(an, asc, bn, bsc, mix) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let qc = || {
        QuickCheck::new()
            .tests(200)
            .max_tests(2000)
            .max_time(Duration::from_secs(86_400))
    };
    let result = match property {
        "AbsSubDifference" => {
            qc().quicktest(qc_abs_sub_difference as fn(i64, u32, i64, u32) -> TestResult)
        }
        "IsIntegerMatchesString" => {
            qc().quicktest(qc_is_integer_matches_string as fn(i64, u32) -> TestResult)
        }
        "FromI128NoPanic" => qc().quicktest(qc_from_i128_no_panic as fn(i64) -> TestResult),
        "FromI128Extremes" => qc().quicktest(qc_from_i128_extremes as fn(u8) -> TestResult),
        "RoundDpPreserves" => {
            qc().quicktest(qc_round_dp_preserves as fn(i64, u8, u8) -> TestResult)
        }
        "CheckedLnNoPanic" => {
            qc().quicktest(qc_checked_ln_no_panic as fn(i64, u8) -> TestResult)
        }
        "ScientificFmtRoundtrip" => {
            qc().quicktest(qc_scientific_fmt_roundtrip as fn(i64, u8) -> TestResult)
        }
        "FromScientificNoPanic" => {
            qc().quicktest(qc_from_scientific_no_panic as fn(u8, u8, u8) -> TestResult)
        }
        "CheckedDivNoPanic" => {
            qc().quicktest(qc_checked_div_no_panic as fn(i64, u8, i64, u8, u8) -> TestResult)
        }
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = QC_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!("({})", arguments.join(" "))),
        ResultStatus::Aborted { err } => Err(format!("aborted: {err:?}")),
        ResultStatus::TimedOut => Err("timed out".to_string()),
        ResultStatus::GaveUp => Err(format!(
            "gave up: passed={}, discarded={}",
            result.n_tests_passed, result.n_tests_discarded
        )),
    };
    (status, metrics)
}

// ============================================================================
// Crabcheck runner (fn-pointer API).
// ============================================================================

static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn cc_abs_sub_difference((an, asc, bn, bsc): (i64, u32, i64, u32)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_abs_sub_difference(an, asc % 10, bn, bsc % 10) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_is_integer_matches_string((n, s): (i64, u32)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_is_integer_matches_string(n, s % 20) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_from_i128_no_panic(n: i64) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_from_i128_no_panic(n) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_from_i128_extremes(ch: u8) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_from_i128_extremes(ch) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_round_dp_preserves((n, s, extra): (i64, u8, u8)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_round_dp_preserves_when_dp_exceeds_scale(n, s, extra) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_checked_ln_no_panic((n, s): (i64, u8)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_checked_ln_no_panic(n, s) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_scientific_fmt_roundtrip((n, s): (i64, u8)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_scientific_fmt_roundtrip(n, s) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_from_scientific_no_panic((eb, bd, ne): (u8, u8, u8)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_from_scientific_no_panic(eb, bd, ne) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_checked_div_no_panic((an, asc, bn, bsc, mix): (i64, u8, i64, u8, u8)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_checked_div_no_panic(an, asc, bn, bsc, mix) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let cfg = crabcheck::quickcheck::Config { tests: 200 };
    let result = match property {
        "AbsSubDifference" => crabcheck_qc::quickcheck_with_config(cfg, cc_abs_sub_difference),
        "IsIntegerMatchesString" => {
            crabcheck_qc::quickcheck_with_config(cfg, cc_is_integer_matches_string)
        }
        "FromI128NoPanic" => crabcheck_qc::quickcheck_with_config(cfg, cc_from_i128_no_panic),
        "FromI128Extremes" => crabcheck_qc::quickcheck_with_config(cfg, cc_from_i128_extremes),
        "RoundDpPreserves" => crabcheck_qc::quickcheck_with_config(cfg, cc_round_dp_preserves),
        "CheckedLnNoPanic" => crabcheck_qc::quickcheck_with_config(cfg, cc_checked_ln_no_panic),
        "ScientificFmtRoundtrip" => {
            crabcheck_qc::quickcheck_with_config(cfg, cc_scientific_fmt_roundtrip)
        }
        "FromScientificNoPanic" => {
            crabcheck_qc::quickcheck_with_config(cfg, cc_from_scientific_no_panic)
        }
        "CheckedDivNoPanic" => {
            crabcheck_qc::quickcheck_with_config(cfg, cc_checked_div_no_panic)
        }
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = CC_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match result.status {
        crabcheck_qc::ResultStatus::Finished => Ok(()),
        crabcheck_qc::ResultStatus::Failed { arguments } => {
            Err(format!("({})", arguments.join(" ")))
        }
        crabcheck_qc::ResultStatus::TimedOut => Err("timed out".to_string()),
        crabcheck_qc::ResultStatus::GaveUp => Err(format!(
            "gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        crabcheck_qc::ResultStatus::Aborted { error } => Err(format!("aborted: {error}")),
    };
    (status, metrics)
}

// ============================================================================
// Hegel runner (hegeltest 0.3.7 from crates.io).
// ============================================================================

static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> HegelSettings {
    use hegel::HealthCheck;
    HegelSettings::new()
        .test_cases(200)
        .suppress_health_check(HealthCheck::all())
}

fn run_hegel_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(AssertUnwindSafe(|| match property {
        "AbsSubDifference" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let an = tc.draw(hgen::integers::<i64>());
                let asc = (tc.draw(hgen::integers::<u8>()) as u32) % 10;
                let bn = tc.draw(hgen::integers::<i64>());
                let bsc = (tc.draw(hgen::integers::<u8>()) as u32) % 10;
                let cex = format!("({an} {asc} {bn} {bsc})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_abs_sub_difference(an, asc, bn, bsc)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "IsIntegerMatchesString" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let n = tc.draw(hgen::integers::<i64>());
                let s = (tc.draw(hgen::integers::<u8>()) as u32) % 20;
                let cex = format!("({n} {s})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_is_integer_matches_string(n, s)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "FromI128NoPanic" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let n = tc.draw(hgen::integers::<i64>());
                let cex = format!("({n})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_from_i128_no_panic(n)));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "FromI128Extremes" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let ch = tc.draw(hgen::integers::<u8>());
                let cex = format!("({ch})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_from_i128_extremes(ch)));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "RoundDpPreserves" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let n = tc.draw(hgen::integers::<i64>());
                let s = tc.draw(hgen::integers::<u8>());
                let extra = tc.draw(hgen::integers::<u8>());
                let cex = format!("({n} {s} {extra})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_round_dp_preserves_when_dp_exceeds_scale(n, s, extra)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "CheckedLnNoPanic" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let n = tc.draw(hgen::integers::<i64>());
                let s = tc.draw(hgen::integers::<u8>());
                let cex = format!("({n} {s})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_checked_ln_no_panic(n, s)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "ScientificFmtRoundtrip" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let n = tc.draw(hgen::integers::<i64>());
                let s = tc.draw(hgen::integers::<u8>());
                let cex = format!("({n} {s})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_scientific_fmt_roundtrip(n, s)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "FromScientificNoPanic" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let eb = tc.draw(hgen::integers::<u8>());
                let bd = tc.draw(hgen::integers::<u8>());
                let ne = tc.draw(hgen::integers::<u8>());
                let cex = format!("({eb} {bd} {ne})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_from_scientific_no_panic(eb, bd, ne)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "CheckedDivNoPanic" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let an = tc.draw(hgen::integers::<i64>());
                let asc = tc.draw(hgen::integers::<u8>());
                let bn = tc.draw(hgen::integers::<i64>());
                let bsc = tc.draw(hgen::integers::<u8>());
                let mix = tc.draw(hgen::integers::<u8>());
                let cex = format!("({an} {asc} {bn} {bsc} {mix})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_checked_div_no_panic(an, asc, bn, bsc, mix)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{property}"),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(msg
                .strip_prefix("Property test failed: ")
                .unwrap_or(&msg)
                .to_string())
        }
    };
    (status, metrics)
}

// ============================================================================
// Dispatch + main
// ============================================================================

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (Err(format!("Unknown tool: {tool}")), Metrics::default()),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    metrics: Metrics,
    counterexample: Option<&str>,
    error: Option<&str>,
) {
    let cex = counterexample.map_or("null".to_string(), json_str);
    let err = error.map_or("null".to_string(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        metrics.inputs,
        json_str(&format!("{}us", metrics.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!(
            "Properties: AbsSubDifference | IsIntegerMatchesString | FromI128NoPanic | FromI128Extremes | RoundDpPreserves | CheckedLnNoPanic | ScientificFmtRoundtrip | FromScientificNoPanic | CheckedDivNoPanic | All"
        );
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(previous_hook);

    let (result, metrics) = match caught {
        Ok(outcome) => outcome,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "panic with non-string payload".to_string()
            };
            emit_json(
                tool,
                property,
                "aborted",
                Metrics::default(),
                None,
                Some(&format!("adapter panic: {msg}")),
            );
            return;
        }
    };

    match result {
        Ok(()) => emit_json(tool, property, "passed", metrics, None, None),
        Err(msg) => emit_json(tool, property, "failed", metrics, Some(&msg), None),
    }
}
