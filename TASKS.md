# rust-decimal — ETNA Tasks

Total tasks: 32

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `abs_sub_returns_abs_dd711db_1` | proptest | `AbsSubDifference` | `witness_abs_sub_difference_case_123_122` |
| 002 | `abs_sub_returns_abs_dd711db_1` | quickcheck | `AbsSubDifference` | `witness_abs_sub_difference_case_123_122` |
| 003 | `abs_sub_returns_abs_dd711db_1` | crabcheck | `AbsSubDifference` | `witness_abs_sub_difference_case_123_122` |
| 004 | `abs_sub_returns_abs_dd711db_1` | hegel | `AbsSubDifference` | `witness_abs_sub_difference_case_123_122` |
| 005 | `checked_ln_zero_panic_092fdf8_1` | proptest | `CheckedLnNoPanic` | `witness_checked_ln_no_panic_case_zero` |
| 006 | `checked_ln_zero_panic_092fdf8_1` | quickcheck | `CheckedLnNoPanic` | `witness_checked_ln_no_panic_case_zero` |
| 007 | `checked_ln_zero_panic_092fdf8_1` | crabcheck | `CheckedLnNoPanic` | `witness_checked_ln_no_panic_case_zero` |
| 008 | `checked_ln_zero_panic_092fdf8_1` | hegel | `CheckedLnNoPanic` | `witness_checked_ln_no_panic_case_zero` |
| 009 | `div_remainder_overflow_a231fbf_1` | proptest | `CheckedDivNoPanic` | `witness_checked_div_no_panic_case_issue_392` |
| 010 | `div_remainder_overflow_a231fbf_1` | quickcheck | `CheckedDivNoPanic` | `witness_checked_div_no_panic_case_issue_392` |
| 011 | `div_remainder_overflow_a231fbf_1` | crabcheck | `CheckedDivNoPanic` | `witness_checked_div_no_panic_case_issue_392` |
| 012 | `div_remainder_overflow_a231fbf_1` | hegel | `CheckedDivNoPanic` | `witness_checked_div_no_panic_case_issue_392` |
| 013 | `from_i128_negation_overflow_6f7d295_1` | proptest | `FromI128Extremes` | `witness_from_i128_no_panic_case_i128_min` |
| 014 | `from_i128_negation_overflow_6f7d295_1` | quickcheck | `FromI128Extremes` | `witness_from_i128_no_panic_case_i128_min` |
| 015 | `from_i128_negation_overflow_6f7d295_1` | crabcheck | `FromI128Extremes` | `witness_from_i128_no_panic_case_i128_min` |
| 016 | `from_i128_negation_overflow_6f7d295_1` | hegel | `FromI128Extremes` | `witness_from_i128_no_panic_case_i128_min` |
| 017 | `is_integer_scale_decrement_951512d_1` | proptest | `IsIntegerMatchesString` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail` |
| 018 | `is_integer_scale_decrement_951512d_1` | quickcheck | `IsIntegerMatchesString` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail` |
| 019 | `is_integer_scale_decrement_951512d_1` | crabcheck | `IsIntegerMatchesString` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail` |
| 020 | `is_integer_scale_decrement_951512d_1` | hegel | `IsIntegerMatchesString` | `witness_is_integer_matches_string_case_scale_10_nonzero_tail` |
| 021 | `round_dp_early_return_reorder_c205456_1` | proptest | `RoundDpPreservesWhenDpExceedsScale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32` |
| 022 | `round_dp_early_return_reorder_c205456_1` | quickcheck | `RoundDpPreservesWhenDpExceedsScale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32` |
| 023 | `round_dp_early_return_reorder_c205456_1` | crabcheck | `RoundDpPreservesWhenDpExceedsScale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32` |
| 024 | `round_dp_early_return_reorder_c205456_1` | hegel | `RoundDpPreservesWhenDpExceedsScale` | `witness_round_dp_preserves_case_zero_scale_28_dp_32` |
| 025 | `scientific_fmt_zero_d0f2a64_1` | proptest | `ScientificFmtRoundtrip` | `witness_scientific_fmt_roundtrip_case_zero` |
| 026 | `scientific_fmt_zero_d0f2a64_1` | quickcheck | `ScientificFmtRoundtrip` | `witness_scientific_fmt_roundtrip_case_zero` |
| 027 | `scientific_fmt_zero_d0f2a64_1` | crabcheck | `ScientificFmtRoundtrip` | `witness_scientific_fmt_roundtrip_case_zero` |
| 028 | `scientific_fmt_zero_d0f2a64_1` | hegel | `ScientificFmtRoundtrip` | `witness_scientific_fmt_roundtrip_case_zero` |
| 029 | `scientific_scale_overflow_722af9f_1` | proptest | `FromScientificNoPanic` | `witness_from_scientific_no_panic_case_neg_u32_max` |
| 030 | `scientific_scale_overflow_722af9f_1` | quickcheck | `FromScientificNoPanic` | `witness_from_scientific_no_panic_case_neg_u32_max` |
| 031 | `scientific_scale_overflow_722af9f_1` | crabcheck | `FromScientificNoPanic` | `witness_from_scientific_no_panic_case_neg_u32_max` |
| 032 | `scientific_scale_overflow_722af9f_1` | hegel | `FromScientificNoPanic` | `witness_from_scientific_no_panic_case_neg_u32_max` |

## Witness Catalog

- `witness_abs_sub_difference_case_123_122` — base passes, variant fails
- `witness_abs_sub_difference_case_neg123_neg124` — base passes, variant fails
- `witness_checked_ln_no_panic_case_zero` — base passes, variant fails
- `witness_checked_div_no_panic_case_issue_392` — base passes, variant fails
- `witness_from_i128_no_panic_case_i128_min` — base passes, variant fails
- `witness_is_integer_matches_string_case_scale_10_nonzero_tail` — base passes, variant fails
- `witness_is_integer_matches_string_case_scale_19_nonzero_high` — base passes, variant fails
- `witness_round_dp_preserves_case_zero_scale_28_dp_32` — base passes, variant fails
- `witness_round_dp_preserves_case_zero_scale_5_dp_15` — base passes, variant fails
- `witness_scientific_fmt_roundtrip_case_zero` — base passes, variant fails
- `witness_scientific_fmt_roundtrip_case_zero_scale_5` — base passes, variant fails
- `witness_from_scientific_no_panic_case_neg_u32_max` — base passes, variant fails
- `witness_from_scientific_no_panic_case_neg_u32_max_digit7` — base passes, variant fails
