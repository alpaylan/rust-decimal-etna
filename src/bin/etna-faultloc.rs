use crabcheck::profiling::quickcheck;
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

// All rust-decimal properties take primitive tuples. Crabcheck's Mutate
// only covers i32/usize/bool/tuples up to 3-ary, so we use usize/i32 and
// cast to u8/u32/i64 inside; 4/5-arg properties are wrapped as nested
// 2-of-3 tuples to stay within the Mutate surface area.

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        return;
    }
    let result = match (args[1].as_str(), args[2].as_str()) {
        ("crabcheck", "AbsSubDifference") => {
            quickcheck(|(an, (asc, bn, bsc)): (i32, (usize, i32, usize))| {
                {
                    to_opt(property_abs_sub_difference(an as i64, asc as u32, bn as i64, bsc as u32))
                }
            })
        }
        ("crabcheck", "IsIntegerMatchesString") => {
            quickcheck(|(n, s): (i32, usize)| {
                to_opt(property_is_integer_matches_string(n as i64, s as u32))
            })
        }
        ("crabcheck", "FromI128Extremes") => quickcheck(|choice: usize| {
            to_opt(property_from_i128_extremes(choice as u8))
        }),
        ("crabcheck", "RoundDpPreservesWhenDpExceedsScale") => {
            quickcheck(|(n, s, extra): (i32, usize, usize)| {
                {
                    to_opt(property_round_dp_preserves_when_dp_exceeds_scale(
                        n as i64,
                        s as u8,
                        extra as u8,
                    ))
                }
            })
        }
        ("crabcheck", "CheckedLnNoPanic") => {
            quickcheck(|(n, s): (i32, usize)| {
                to_opt(property_checked_ln_no_panic(n as i64, s as u8))
            })
        }
        ("crabcheck", "ScientificFmtRoundtrip") => {
            quickcheck(|(n, s): (i32, usize)| {
                to_opt(property_scientific_fmt_roundtrip(n as i64, s as u8))
            })
        }
        ("crabcheck", "FromScientificNoPanic") => {
            quickcheck(|(eb, bd, ne): (usize, usize, usize)| {
                {
                    to_opt(property_from_scientific_no_panic(eb as u8, bd as u8, ne as u8))
                }
            })
        }
        ("crabcheck", "CheckedDivNoPanic") => {
            quickcheck(
                |((an, asc, bn), (bsc, mix)): ((i32, usize, i32), (usize, usize))| {
                    {
                        to_opt(property_checked_div_no_panic(
                            an as i64,
                            asc as u8,
                            bn as i64,
                            bsc as u8,
                            mix as u8,
                        ))
                    }
                },
            )
        }
        (a, b) => panic!("Unknown: {a} {b}"),
    };
    println!("Result: {:?}", result);
}
