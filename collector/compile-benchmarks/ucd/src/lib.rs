#![no_std]
use core::char;
use core::slice;

pub mod tables;
use tables::Search;
pub use tables::{
    BidiClass,
    BidiPairedBracketType,
    DecompositionType,
    EastAsianWidth,
    GraphemeClusterBreak,
    HangulSyllableType,
    IndicPositionalCategory,
    IndicSyllabicCategory,
    JoiningGroup,
    JoiningType,
    LinebreakClass,
    NumericType,
    Script,
    SentenceBreak,
    Trilean,
    UnicodeBlock,
    UnicodeCategory,
    WordBreak
};

// for use with numeric_value
#[derive(Clone,Copy,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub enum Number {
    Integer(i64),
    Rational(i32,u32)
}

fn cp_decode((c1,c2,c3): (u8,u8,u8)) -> char {
    let c = (c1 as u32)*65536 + (c2 as u32)*256 + (c3 as u32);
    unsafe { char::from_u32_unchecked(c) }
}

enum CharIterInternal {
    Iterator(slice::Iter<'static, (u8,u8,u8)>),
    Double(char, char),
    Single(char),
    Exhausted
}

pub struct CharIter(CharIterInternal);

impl CharIter {
    pub fn new(osl: Option<&'static [(u8,u8,u8)]>, cp: char) -> CharIter {
        CharIter(match osl {
            Some(sl) => CharIterInternal::Iterator(sl.iter()),
            None => CharIterInternal::Single(cp)
        })
    }

    pub fn hangul(a: char, b: char) -> CharIter {
        CharIter(CharIterInternal::Double(a, b))
    }
}

impl Iterator for CharIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.0 {
            CharIterInternal::Iterator(ref mut it) => it.next().map(|c| cp_decode(*c)),
            CharIterInternal::Double(a, b) => {
                self.0 = CharIterInternal::Single(b);
                Some(a)
            },
            CharIterInternal::Single(c) => {
                self.0 = CharIterInternal::Exhausted;
                Some(c)
            },
            CharIterInternal::Exhausted => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            CharIterInternal::Iterator(ref it) => it.size_hint(),
            CharIterInternal::Double(_, _) => (2, Some(2)),
            CharIterInternal::Single(_) => (1, Some(1)),
            CharIterInternal::Exhausted => (0, Some(0))
        }
    }
}

pub trait Codepoint where Self: core::marker::Sized {
    // general
    fn age(self) -> Option<(u8,u8)>;
    fn block(self) -> Option<UnicodeBlock>;
    fn category(self) -> UnicodeCategory;
    fn codepoint(self) -> char;
    fn iso_comment(self) -> &'static str;

    // function and appearance
    fn is_alphabetic(self) -> bool;
    fn is_alphabetic_other(self) -> bool;
    fn is_dash(self) -> bool;
    fn is_default_ignorable(self) -> bool;
    fn is_default_ignorable_other(self) -> bool;
    fn is_deprecated(self) -> bool;
    fn is_diacritic(self) -> bool;
    fn is_extender(self) -> bool;
    fn is_hex_digit(self) -> bool;
    fn is_hex_digit_ascii(self) -> bool;
    fn is_hyphen(self) -> bool;
    fn is_logical_order_exception(self) -> bool;
    fn is_math(self) -> bool;
    fn is_math_other(self) -> bool;
    fn is_noncharacter(self) -> bool;
    fn is_preprended_concatenation_mark(self) -> bool;
    fn is_quotation_mark(self) -> bool;
    fn is_sentence_terminal(self) -> bool;
    fn is_soft_dotted(self) -> bool;
    fn is_terminal_punctuation(self) -> bool;
    fn is_variation_selector(self) -> bool;
    fn is_whitespace(self) -> bool;

    // numeric
    fn numeric_type(self) -> Option<NumericType>;
    fn numeric_value(self) -> Option<Number>;

    // identifiers and syntax
    fn is_id_continue(self) -> bool;
    fn is_id_continue_nfkc(self) -> bool;
    fn is_id_continue_other(self) -> bool;
    fn is_id_start(self) -> bool;
    fn is_id_start_nfkc(self) -> bool;
    fn is_id_start_other(self) -> bool;
    fn is_pattern_syntax(self) -> bool;
    fn is_pattern_whitespace(self) -> bool;

    // scripts
    fn east_asian_width(self) -> EastAsianWidth;
    fn hangul_syllable_type(self) -> Option<HangulSyllableType>;
    fn jamo_short_name(self) -> Option<&'static str>;
    fn indic_positional_category(self) -> Option<IndicPositionalCategory>;
    fn indic_syllabic_category(self) -> IndicSyllabicCategory;
    fn is_ideograph(self) -> bool;
    fn is_ideograph_description_sequence_binary_operator(self) -> bool;
    fn is_ideograph_description_sequence_radical(self) -> bool;
    fn is_ideograph_description_sequence_trinary_operator(self) -> bool;
    fn is_ideograph_unified(self) -> bool;
    fn join_control(self) -> bool;
    fn joining_group(self) -> JoiningGroup;
    fn joining_type(self) -> JoiningType;
    fn script(self) -> Option<Script>;
    fn script_extensions(self) -> Option<&'static [Script]>;

    // bidirectionality
    fn bidi_class(self) -> BidiClass;
    fn bidi_is_control(self) -> bool;
    fn bidi_is_mirrored(self) -> bool;
    fn bidi_mirror(self) -> Option<char>;
    fn bidi_paired_bracket(self) -> char;
    fn bidi_paired_bracket_type(self) -> Option<BidiPairedBracketType>;

    // case
    fn casefold(self) -> CharIter;
    fn casefold_nfkc(self) -> CharIter;
    fn casefold_nfkc_closure(self) -> CharIter;
    fn casefold_simple(self) -> char;
    fn changes_when_casefolded(self) -> bool;
    fn changes_when_casefolded_nfkc(self) -> bool;
    fn changes_when_casemapped(self) -> bool;
    fn changes_when_lowercased(self) -> bool;
    fn changes_when_titlecased(self) -> bool;
    fn changes_when_uppercased(self) -> bool;
    fn is_case_ignorable(self) -> bool;
    fn is_cased(self) -> bool;
    fn is_lowercase(self) -> bool;
    fn is_lowercase_other(self) -> bool;
    fn is_uppercase(self) -> bool;
    fn is_uppercase_other (self) -> bool;
    fn lowercase(self) -> CharIter;
    fn lowercase_simple(self) -> char;
    fn titlecase(self) -> CharIter;
    fn titlecase_simple(self) -> char;
    fn uppercase(self) -> CharIter;
    fn uppercase_simple(self) -> char;

    // normalisation
    fn canonical_combining_class(self) -> u8;
    fn decomposition_map(self) -> CharIter;
    fn decomposition_type(self) -> Option<DecompositionType>;
    fn excluded_from_composition(self) -> bool;
    fn excluded_from_composition_fully(self) -> bool;
    fn expands_on_nfc(self) -> bool;
    fn expands_on_nfd(self) -> bool;
    fn expands_on_nfkc(self) -> bool;
    fn expands_on_nfkd(self) -> bool;
    fn quick_check_nfc(self) -> Trilean;
    fn quick_check_nfd(self) -> bool;
    fn quick_check_nfkc(self) -> Trilean;
    fn quick_check_nfkd(self) -> bool;

    // segmentation
    fn grapheme_cluster_break(self) -> GraphemeClusterBreak;
    fn is_grapheme_base(self) -> bool;
    fn is_grapheme_extend(self) -> bool;
    fn is_grapheme_extend_other(self) -> bool;
    fn is_grapheme_link(self) -> bool;
    fn linebreak_class(self) -> Option<LinebreakClass>;
    fn sentence_break(self) -> SentenceBreak;
    fn word_break(self) -> WordBreak;

    // account for inbuilt char methods, which seem to be for unicode 8.0
    fn is_alpha(self) -> bool { Codepoint::is_alphabetic(self) }
    fn is_lower(self) -> bool { Codepoint::is_lowercase(self) }
    fn is_upper(self) -> bool { Codepoint::is_uppercase(self) }
    fn is_white(self) -> bool { Codepoint::is_whitespace(self) }
}

impl Codepoint for char {
    // general
    fn age(self) -> Option<(u8,u8)> {
        tables::UCD_AGE.search(self)
    }

    fn block(self) -> Option<UnicodeBlock> {
        tables::UCD_BLOCK.search(self)
    }

    fn category(self) -> UnicodeCategory {
        tables::UCD_CAT.search(self)
            .unwrap_or(UnicodeCategory::Unassigned)
    }

    fn codepoint(self) -> char {
        self
    }

    fn iso_comment(self) -> &'static str {
        ""
    }




    // function and appearance
    fn is_alphabetic(self) -> bool {
        tables::UCD_ALPHA.includes(self)
    }

    fn is_alphabetic_other(self) -> bool {
        tables::UCD_ALPHA_OTHER.includes(self)
    }

    fn is_dash(self) -> bool {
        tables::UCD_DASH.includes(self)
    }

    fn is_default_ignorable(self) -> bool {
        tables::UCD_DEFAULT_IGNORABLE.includes(self)
    }

    fn is_default_ignorable_other(self) -> bool {
        tables::UCD_DEFAULT_IGNORABLE_OTHER.includes(self)
    }

    fn is_deprecated(self) -> bool {
        match self as u32 {
            329 | 1651 | 3959 | 3961 | 6051 | 6052 |
                  8298...8303 | 9001 | 9002 | 917505 => true,
            _ => false
        }
    }
    fn is_diacritic(self) -> bool {
        tables::UCD_DIACRITIC.includes(self)
    }

    fn is_extender(self) -> bool {
        tables::UCD_EXTENDER.includes(self)
    }

    fn is_hex_digit(self) -> bool {
        tables::UCD_HEX_DIGIT.includes(self)
    }

    fn is_hex_digit_ascii(self) -> bool {
        tables::UCD_HEX_DIGIT_ASCII.includes(self)
    }

    fn is_hyphen(self) -> bool {
        tables::UCD_HYPHEN.includes(self)
    }

    fn is_logical_order_exception(self) -> bool {
        tables::UCD_LOGICAL_ORDER_EXCEPTION.includes(self)
    }

    fn is_math(self) -> bool {
        tables::UCD_MATH.includes(self)
    }

    fn is_math_other(self) -> bool {
        tables::UCD_MATH_OTHER.includes(self)
    }

    fn is_noncharacter(self) -> bool {
        let cp = self as u32;
        (cp >= 0xfdd0 && cp <= 0xfdef)
            || ((cp & 0xffff) >= 0xfffe)
    }

    fn is_preprended_concatenation_mark(self) -> bool {
        tables::UCD_PREPENDED_CONCATENATION_MARK.includes(self)
    }

    fn is_quotation_mark(self) -> bool {
        tables::UCD_QUOT.includes(self)
    }

    fn is_sentence_terminal(self) -> bool {
        tables::UCD_TERM_SENTENCE.includes(self)
    }

    fn is_soft_dotted(self) -> bool {
        tables::UCD_SOFT_DOTTED.includes(self)
    }

    fn is_terminal_punctuation(self) -> bool {
        tables::UCD_TERM_PUNC.includes(self)
    }

    fn is_variation_selector(self) -> bool {
        let cp = self as u32;
        (cp >= 917760 && cp <= 917999)
            || (cp >= 65024 && cp <= 65039)
            || (cp >= 6155 && cp <= 6157)
    }

    fn is_whitespace(self) -> bool {
        tables::UCD_WHITE.includes(self)
    }




    // numeric
    fn numeric_type(self) -> Option<NumericType> {
        tables::UCD_NUMTYPE.search(self)
    }

    fn numeric_value(self) -> Option<Number> {
        tables::UCD_NUMVAL.search(self).map(|i| {
            match tables::UCD_NUMS[i as usize] {
                (num, 1) => Number::Integer(num),
                (num, den) => Number::Rational(num as i32, den as u32)
            }
        })
    }



    // identifiers and syntax
    fn is_id_continue(self) -> bool {
        tables::UCD_ID_CONT.includes(self)
    }

    fn is_id_continue_nfkc(self) -> bool {
        tables::UCD_ID_CONT_NFKC.includes(self)
    }

    fn is_id_continue_other(self) -> bool {
         match self as u32 {
            183 | 903 | 4969...4977 | 6618 => true,
            _ => false
        }
    }

    fn is_id_start(self) -> bool {
        tables::UCD_ID_START.includes(self)
    }

    fn is_id_start_nfkc(self) -> bool {
        tables::UCD_ID_START_NFKC.includes(self)
    }

    fn is_id_start_other(self) -> bool {
        match self as u32 {
            6277 | 6278 | 8472 | 8494| 12443 | 12444 => true,
            _ => false
        }
    }

    fn is_pattern_syntax(self) -> bool {
        tables::UCD_PATT_SYNTAX.includes(self)
    }

    fn is_pattern_whitespace(self) -> bool {
        match self as u32 {
            9...13 | 32 | 133 | 8206
                   | 8207 | 8232 | 8233 => true,
            _ => false
        }
    }




    // scripts
    fn east_asian_width(self) -> EastAsianWidth {
        tables::UCD_EAWIDTH.search(self)
            .unwrap_or(EastAsianWidth::Neutral)
    }

    fn hangul_syllable_type(self) -> Option<HangulSyllableType> {
        let cp = self as u32;
        match cp {
            4352...4447 | 43360...43388 => Some(HangulSyllableType::LeadingJamo),
            4448...4519 | 55216...55238 => Some(HangulSyllableType::VowelJamo),
            4520...4607 | 55243...55291 => Some(HangulSyllableType::TrailingJamo),
            44032...55203 => Some({
                if cp % 28 == 16 { HangulSyllableType::LVSyllable }
                else { HangulSyllableType::LVTSyllable }
            }),
            _ => None
        }
    }

    fn jamo_short_name(self) -> Option<&'static str> {
        tables::UCD_JSN.search(self)
    }

    fn indic_positional_category(self) -> Option<IndicPositionalCategory> {
        tables::UCD_INPC.search(self)
    }

    fn indic_syllabic_category(self) -> IndicSyllabicCategory {
        tables::UCD_INSC.search(self)
            .unwrap_or(IndicSyllabicCategory::Other)
    }

    fn is_ideograph(self) -> bool {
        tables::UCD_IDEO.includes(self)
    }

    fn is_ideograph_description_sequence_binary_operator(self) -> bool {
        match self as u32 {
            12272 | 12273 | 12276...12283 => true,
            _ => false
        }
    }

    fn is_ideograph_description_sequence_radical(self) -> bool {
        match self as u32 {
            11904...11929 | 11931...12019 | 12032...12245 => true,
            _ => false
        }
    }

    fn is_ideograph_description_sequence_trinary_operator(self) -> bool {
        let cp = self as u32;
        cp == 12274 || cp == 12275
    }

    fn is_ideograph_unified(self) -> bool {
        tables::UCD_IDEO_UNIFIED.includes(self)
    }

    fn join_control(self) -> bool {
        let cp = self as u32;
        cp == 8204 || cp == 8205
    }

    fn joining_group(self) -> JoiningGroup {
        tables::UCD_JOINGRP.search(self)
            .unwrap_or(JoiningGroup::NoJoiningGroup)
    }

    fn joining_type(self) -> JoiningType {
        tables::UCD_JOINTYPE.search(self)
            .unwrap_or(JoiningType::NonJoining)
    }

    fn script(self) -> Option<Script> {
        tables::UCD_SCRIPT.search(self)
    }

    fn script_extensions(self) -> Option<&'static [Script]> {
        match tables::UCD_SCRIPTEXT.search(self) {
            None => self.script().map(
                |s| tables::UCD_SCRIPT_MAP[s as usize]),
            x => x
        }
    }




    // bidirectionality
    fn bidi_class(self) -> BidiClass {
        tables::UCD_BIDI_CLASS.search(self)
            .unwrap_or(BidiClass::LeftToRight)
    }

    fn bidi_is_control(self) -> bool {
        match self as u32 {
            1564 | 8206 | 8207 | 8234...8238 | 8294...8297 => true,
            _ => false
        }
    }

    fn bidi_is_mirrored(self) -> bool {
        tables::UCD_BIDI_MIRRORED.includes(self)
    }

    fn bidi_mirror(self) -> Option<char> {
        tables::UCD_BIDI_MIRROR.search(self)
    }

    fn bidi_paired_bracket(self) -> char {
        tables::UCD_BIDI_PAIRED.search(self)
            .unwrap_or(self)
    }

    fn bidi_paired_bracket_type(self) -> Option<BidiPairedBracketType> {
        tables::UCD_BIDI_BRATYPE.search(self)
    }




    // case
    fn casefold(self) -> CharIter {
        CharIter::new(tables::UCD_CASE_FD.search(self), self)
    }

    fn casefold_nfkc(self) -> CharIter {
        CharIter::new(tables::UCD_CASE_FD_NFKC.search(self), self)
    }

    fn casefold_nfkc_closure(self) -> CharIter {
        CharIter::new(tables::UCD_CASE_FD_CLOS.search(self), self)
    }

    fn casefold_simple(self) -> char {
        tables::UCD_CASE_SIMP_FD.search(self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }

    fn changes_when_casefolded(self) -> bool {
        tables::UCD_CASE_CHANGES_CASEFOLD.includes(self)
    }

    fn changes_when_casefolded_nfkc(self) -> bool {
        tables::UCD_CASE_CHANGES_CASEFOLD_NFKC.includes(self)
    }

    fn changes_when_casemapped(self) -> bool {
        tables::UCD_CASE_CHANGES_CASEMAP.includes(self)
    }

    fn changes_when_lowercased(self) -> bool {
        tables::UCD_CASE_CHANGES_LOWER.includes(self)
    }

    fn changes_when_titlecased(self) -> bool {
        tables::UCD_CASE_CHANGES_TITLE.includes(self)
    }

    fn changes_when_uppercased(self) -> bool {
        tables::UCD_CASE_CHANGES_UPPER.includes(self)
    }

    fn is_case_ignorable(self) -> bool {
        tables::UCD_CASE_IGNORABLE.includes(self)
    }

    fn is_cased(self) -> bool {
        tables::UCD_CASED.includes(self)
    }

    fn is_lowercase(self) -> bool {
        tables::UCD_CASE_IS_LOWER.includes(self)
    }

    fn is_lowercase_other(self) -> bool {
        tables::UCD_CASE_IS_LOWER_OTHER.includes(self)
    }

    fn is_uppercase(self) -> bool {
        tables::UCD_CASE_IS_UPPER.includes(self)
    }

    fn is_uppercase_other (self) -> bool {
        match self as u32 {
            8544...8559 | 9398...9423 | 127280...127305
                        | 127312...127337 | 127344...127369 => true,
            _ => false
        }
    }

    fn lowercase(self) -> CharIter {
        CharIter::new(tables::UCD_CASE_LW.search(self), self)
    }

    fn lowercase_simple(self) -> char {
        tables::UCD_CASE_SIMP_LW.search(self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }

    fn titlecase(self) -> CharIter {
        CharIter::new(tables::UCD_CASE_TI.search(self), self)
    }

    fn titlecase_simple(self) -> char {
        tables::UCD_CASE_SIMP_TI.search(self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }

    fn uppercase(self) -> CharIter {
        CharIter::new(tables::UCD_CASE_UP.search(self), self)
    }

    fn uppercase_simple(self) -> char {
        tables::UCD_CASE_SIMP_UP.search(self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }




    // normalisation
    fn canonical_combining_class(self) -> u8 {
        tables::UCD_COMBCLS.search(self)
            .unwrap_or(0)
    }

    fn decomposition_map(self) -> CharIter {
        // manually handle arithmetic decomposition mapping
        // for hangul syllables, cutting out 11172 entries
        // from the data table; implementation is directly
        // from the unicode standard, chapter 3.12
        const SBASE: u32 = 0xac00;
        const LBASE: u32 = 0x1100;
        const VBASE: u32 = 0x1161;
        const TBASE: u32 = 0x11a7;
        const VCOUNT: u32 = 21;
        const TCOUNT: u32 = 28;
        const NCOUNT: u32 = VCOUNT * TCOUNT;

        match self.hangul_syllable_type() {
            Some(HangulSyllableType::LVSyllable) => unsafe {
                let sindex = (self as u32) - SBASE;
                let lindex = sindex / NCOUNT;
                let vindex = (sindex % NCOUNT) / TCOUNT;
                CharIter::hangul(
                    char::from_u32_unchecked(LBASE + lindex),
                    char::from_u32_unchecked(VBASE + vindex))
            },
            Some(HangulSyllableType::LVTSyllable) => unsafe {
                let sindex = (self as u32) - SBASE;
                let tindex = sindex % TCOUNT;
                let lvindex = sindex - tindex;
                CharIter::hangul(
                    char::from_u32_unchecked(SBASE + lvindex),
                    char::from_u32_unchecked(TBASE + tindex))
            },
            _ =>
                CharIter::new(tables::UCD_DECOMP_MAP.search(self), self)
        }
    }

    fn decomposition_type(self) -> Option<DecompositionType> {
        tables::UCD_DECOMP_TYPE.search(self)
    }

    fn excluded_from_composition(self) -> bool {
        tables::UCD_COMP_EXCL.includes(self)
    }

    fn excluded_from_composition_fully(self) -> bool {
        tables::UCD_COMP_EXCL_FULL.includes(self)
    }

    fn expands_on_nfc(self) -> bool {
        tables::UCD_EXPANDING_NFC.includes(self)
    }

    fn expands_on_nfd(self) -> bool {
        tables::UCD_EXPANDING_NFD.includes(self)
    }

    fn expands_on_nfkc(self) -> bool {
        tables::UCD_EXPANDING_NFKC.includes(self)
    }

    fn expands_on_nfkd(self) -> bool {
        tables::UCD_EXPANDING_NFKD.includes(self)
    }

    fn quick_check_nfc(self) -> Trilean {
        tables::UCD_QNFC.search(self)
            .unwrap_or(Trilean::True)
    }

    fn quick_check_nfd(self) -> bool {
        !tables::UCD_QUICK_NFD.includes(self)
    }

    fn quick_check_nfkc(self) -> Trilean {
        match tables::UCD_QNFKC.includes(self) {
            true => Trilean::False,
            false => self.quick_check_nfc()
        }
    }

    fn quick_check_nfkd(self) -> bool {
        !tables::UCD_QUICK_NFKD.includes(self)
    }




    // segmentation
    fn grapheme_cluster_break(self) -> GraphemeClusterBreak {
        let cx = self.clone();
        match self.hangul_syllable_type() {
            Some(HangulSyllableType::LeadingJamo)  => GraphemeClusterBreak::LeadingJamo,
            Some(HangulSyllableType::VowelJamo)    => GraphemeClusterBreak::VowelJamo,
            Some(HangulSyllableType::TrailingJamo) => GraphemeClusterBreak::TrailingJamo,
            Some(HangulSyllableType::LVSyllable)   => GraphemeClusterBreak::LVHangulSyllable,
            Some(HangulSyllableType::LVTSyllable)  => GraphemeClusterBreak::LVTHangulSyllable,
            None => tables::UCD_GCB.search(cx)
                        .unwrap_or(GraphemeClusterBreak::Other)
        }
    }

    fn is_grapheme_base(self) -> bool {
        tables::UCD_GRAPH_BASE.includes(self)
    }

    fn is_grapheme_extend(self) -> bool {
        tables::UCD_GRAPH_EXT.includes(self)
    }

    fn is_grapheme_extend_other(self) -> bool {
        tables::UCD_GRAPH_EXT_OTHER.includes(self)
    }

    fn is_grapheme_link(self) -> bool {
        tables::UCD_GRAPH_LINK.includes(self)
    }

    fn linebreak_class(self) -> Option<LinebreakClass> {
        tables::UCD_LB.search(self)
    }

    fn sentence_break(self) -> SentenceBreak {
        tables::UCD_SBRK.search(self)
            .unwrap_or(SentenceBreak::Other)
    }

    fn word_break(self) -> WordBreak {
        tables::UCD_WBRK.search(self)
            .unwrap_or(WordBreak::Other)
    }
}