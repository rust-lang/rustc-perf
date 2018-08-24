# `libucd`

[![Build Status](https://travis-ci.org/sourtin/libucd.svg?branch=master)](https://travis-ci.org/sourtin/libucd)
[![crates.io](http://meritbadge.herokuapp.com/ucd)](https://crates.io/crates/ucd)

This library extends the inbuilt `char` type with the `Codepoint` trait, which implements 100 properties of the UCD (Unicode Character Database). It aims to be fast and compact, and to have minimal dependencies (it does not require the rust standard library so only needs rust's `core` crate).

```rust
extern crate ucd;
use ucd::Codepoint;

fn main() {
    let salawat: char = 'ﷺ';
    let decomp: String = salawat.decomposition_map().collect();
    println!("{} -> {}", salawat, decomp);
    // ﷺ -> صلى الله عليه وسلم
}
```

Though the library is fairly extensive, it is not complete. The properties it lacks are:

* Character [names](https://github.com/huonw/unicode_names) and aliases
* Unihan properties
* Tangut source data
* Named sequences
* Standardised variants
* CJK Radicals
* Emoji source data

Data was compressed as arrays of ranges of codepoints having each value for each property, excluding the most common value. Lookup is then implemented by a binary search which should provide O(1) access. A very unscientific test suggests each lookup takes around 100ns on a core i5 processor.

## Disclaimer

Please note that this data has been derived from the flat XML version of the UCD. Though this is published by Unicode, it is not the official version of the database. As such, it cannot be guaranteed that this library is entirely consistent with the official database. The unit tests do however suggest consistency with the XML database at least.

## Reference

_Note, in most cases enum values are simply the full name of the property value converted to camelcase. Perhaps I will eventually create a rustdoc version that will be more helpful._

In the tables below, the first column is the name of the property, and the second column is the method name. The methods are implemented via the `ucd::Codepoint` trait on `char`.

| General | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Age | `age` | `Option<(u8,u8)>` | Return `None` if the character is unassigned, else a tuple of major and minor age |
| Block | `block` | `Option<UnicodeBlock>` |  |
| General_Category | `category` | `UnicodeCategory` | Return `Unassigned` if unassigned |
| ISO_Comment (deprecated, stabilized) | `iso_comment` | `&str` | All characters are null, i.e. `""` is returned |

| Function and Appearance | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Alphabetic | `Codepoint::is_alphabetic` or `is_alpha` | `bool` | Rust's `char` already defines a method `is_alphabetic` which shadows this implementation and provides slightly different results (possibly by using an outdated version of the UCD). Use the namespace, or the `is_alpha` alias instead. |
| ASCII_Hex_Digit | `is_hex_digit_ascii` | `bool` | |
| Dash | `is_dash` | `bool` | |
| Default_Ignorable_Code_Point | `is_default_ignorable` | `bool` | |
| Deprecated | `is_deprecated` | `bool` | |
| Diacritic | `is_diacritic` | `bool` | |
| Extender | `is_extender` | `bool` | |
| Hex_Digit | `is_hex_digit` | `bool` | |
| Hyphen _(deprecated, stabilized)_ | `is_hyphen` | `bool` | |
| Logical_Order_Exception | `is_logical_order_exception` | `bool` | |
| Math | `is_math` | `bool` | |
| Noncharacter_Code_Point | `is_noncharacter` | `bool` | |
| Other_Alphabetic | `is_alphabetic_other` | `bool` | |
| Other_Default_Ignorable_Code_Point | `is_default_ignorable_other` | `bool` | |
| Other_Math | `is_math_other` | `bool` | |
| Prepended_Concatenation_Mark | `is_prepended_concatentation_mark` | `bool` | |
| Quotation_Mark | `is_quotation_mark` | `bool` | |
| Sentence_Terminal | `is_sentence_terminal` | `bool` | |
| Soft_Dotted | `is_soft_dotted` | `bool` | |
| Terminal_Punctuation | `is_terminal_punctuation` | `bool` | |
| Variation_Selector | `is_variation_selector` | `bool` | |
| White_Space | `Codepoint::is_whitespace` or `is_white` | `bool` | Again, `char` shadows this with its own implementation so use the namespace or alias. |

| Numeric | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Numeric_Type | `numeric_type` | `Option<NumericType>` |  |
| Numeric_Value | `numeric_value` | `Option<Number>` | The `Number` type is an enum of `Integer(i64)` and `Rational(i32,u32)`, to cover values as large as 10^12 and as small as 1/160. |

| Identifiers and Syntax | Method | Return Type | Note |
| ---- | --- | --- | --- |
| ID_Continue | `is_id_continue` | `bool` | |
| ID_Start | `is_id_start` | `bool` | |
| Other_ID_Continue | `is_id_continue_other` | `bool` | |
| Other_ID_Start | `is_id_start_other` | `bool` | |
| Pattern_Syntax | `is_pattern_syntax` | `bool` | |
| Pattern_White_Space | `is_pattern_whitespace` | `bool` | |
| XID_Continue | `is_id_continue_nfkc` | `bool` | |
| XID_Start | `is_id_start_nfkc` | `bool` | |

| Scripts | Method | Return Type | Note |
| ---- | --- | --- | --- |
| East_Asian_Width | `east_asian_width` | `EastAsianWidth` | |
| Hangul_Syllable_Type | `hangul_syllable_type` | `Option<HangulSyllableType>` | |
| Ideographic | `is_ideograph` | `bool` | |
| IDS_Binary_Operator | `is_ideograph_description_sequence_binary_operator` | `bool` | |
| IDS_Trinary_Operator | `is_ideograph_description_sequence_trinary_operator` | `bool` | |
| Indic_Positional_Category | `indic_positional_category` | `Option<IndicPositionalCategory>` | |
| Indic_Syllabic_Category | `indic_syllabic_category` | `Option<IndicSyllabicCategory>` | |
| Jamo_Short_Name | `jamo_short_name` | `Option<&str>` | |
| Join_Control | `join_control` | `bool` | |
| Joining_Group | `joining_group` | `JoiningGroup` | |
| Joining_Type | `joining_type` | `JoiningType` | |
| Radical | `is_ideograph_description_sequence_radical` | `bool` | |
| Script | `script` | `Option<Script>` | |
| Script_Extensions | `script_extensions` | `Option<&[Script]>` | |
| Unified_Ideograph | `is_ideograph_unified` | `bool` | |

| Bidirectionality | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Bidi_Class | `bidi_class` | `BidiClass` | |
| Bidi_Control | `bidi_is_control` | `bool` | |
| Bidi_Mirrored | `bidi_is_mirrored` | `bool` | |
| Bidi_Mirroring_Glyph | `bidi_mirror` | `Option<char>` | |
| Bidi_Paired_Bracket | `bidi_paired_braclet` | `char` | |
| Bidi_Paired_Bracket_Type | `bidi_paired_bracket_type` | `Option<BidiPairedBracketType>` | |

| Case | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Case_Folding | `casefold` | `CharIter` | `CharIter` is an iterator over a series of `char`s, and is used because the library makes no use of `std` and thus cannot dynamically allocate memory. |
| Case_Ignorable | `is_case_ignorable` | `bool` | |
| Cased | `is_cased` | `bool` | |
| Changes_When_Casefolded | `changes_when_casefolded` | `bool` | |
| Changes_When_Casemapped | `changes_when_casemapped` | `bool` | |
| Changes_When_Lowercased | `changes_when_lowercased` | `bool` | |
| Changes_When_NFKC_Casefolded | `changes_when_casefolded_nfkc` | `bool` | |
| Changes_When_Titlecased | `changes_when_titlecased` | `bool` | |
| Changes_When_Uppercased | `changes_when_uppercased` | `bool` | |
| FC_NFKC_Closure (deprecated) | `casefold_nfkc_closure` | `CharIter` | |
| Lowercase | `is_lowercase` | `bool` | Again, `char` shadows this with its own implementation so use the namespace or alias. |
| Lowercase_Mapping | `lowercase` | `CharIter` | |
| NFKC_Casefold | `casefold_nfkc` | `CharIter` | |
| Other_Lowercase | `is_lowercase_other` | `bool` |
| Other_Uppercase | `is_uppercase_other` | `bool` |
| Simple_Case_Folding | `casefold_simple` | `char` | |
| Simple_Lowercase_Mapping | `lowercase_simple` | `char` | |
| Simple_Titlecase_Mapping | `titlecase_simple` | `char` | |
| Simple_Uppercase_Mapping | `uppercase_simple` | `char` | |
| Titlecase_Mapping | `titlecase` | `CharIter` | |
| Uppercase | `is_uppercase` | `bool` | Again, `char` shadows this with its own implementation so use the namespace or alias. |
| Uppercase_Mapping | `uppercase` | `CharIter` | |

| Normalisation | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Canonical_Combining_Class | `canonical_combining_class` | `u8` | |
| Composition_Exclusion | `excluded_from_composition` | `bool` | |
| Decomposition_Mapping | `decomposition_map` | `CharIter` | |
| Decomposition_Type | `decomposition_type` | `Option<DecompositionType>` | |
| Expands_On_NFC _(deprecated)_ | `expands_on_nfc` | `bool` | |
| Expands_On_NFD _(deprecated)_ | `expands_on_nfd` | `bool` | |
| Expands_On_NFKC _(deprecated)_ | `expands_on_nfkc` | `bool` | |
| Expands_On_NFKD _(deprecated)_ | `expands_on_nfkd` | `bool` |
| Full_Composition_Exclusion | `excluded_from_composition_fully` | `bool` | |
| NFC_Quick_Check | `quick_check_nfc` | `Trilean` | Returns one of `Trilean::True`, `Trilean::Maybe` or `Trilean::False` |
| NFD_Quick_Check | `quick_check_nfd` | `bool` | |
| NFKC_Quick_Check | `quick_check_nfkc` | `Trilean` | |
| NFKD_Quick_Check | `quick_check_nfkd` | `bool` |

| Segmentation | Method | Return Type | Note |
| ---- | --- | --- | --- |
| Grapheme_Base | `is_grapheme_base` | `bool` | |
| Grapheme_Cluster_Break | `grapheme_cluster_break` | `GraphemeClusterBreak` | |
| Grapheme_Extend | `is_grapheme_extend` | `bool` | |
| Grapheme_Link _(deprecated)_ | `is_grapheme_link` | `bool` | |
| Line_Break | `linebreak_class` | `Option<LinebreakClass>` | |
| Other_Grapheme_Extend | `is_grapheme_extend_other` | `bool` | |
| Sentence_Break | `sentence_break` | `SentenceBreak` | |
| Word_Break | `word_break` | `WordBreak` |
