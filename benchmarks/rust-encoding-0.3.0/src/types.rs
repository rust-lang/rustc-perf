/*!
 * Reexports of types from encoding-types crate.
 */

pub use encoding_types::{CodecError, DecoderTrap, EncoderTrap, ByteWriter, Encoding, RawDecoder,
                         RawEncoder, StringWriter, EncodingRef, EncoderTrapFunc, DecoderTrapFunc};
pub use super::decode;

#[cfg(test)]
mod tests {
    use super::*;
    use super::EncoderTrap::NcrEscape;
    use util::StrCharIndex;
    use std::convert::Into;
    use std::sync::mpsc::channel;

    // a contrived encoding example: same as ASCII, but inserts `prepend` between each character
    // within two "e"s (so that `widespread` becomes `wide*s*p*r*ead` and `eeeeasel` becomes
    // `e*ee*ease*l` where `*` is substituted by `prepend`) and prohibits `prohibit` character.
    struct MyEncoder { flag: bool, prohibit: char, prepend: &'static str, toggle: bool }
    impl RawEncoder for MyEncoder {
        fn from_self(&self) -> Box<RawEncoder> {
            Box::new(MyEncoder { flag: self.flag,
                                 prohibit: self.prohibit,
                                 prepend: self.prepend,
                                 toggle: false })
        }
        fn is_ascii_compatible(&self) -> bool { self.flag }
        fn raw_feed(&mut self, input: &str,
                    output: &mut ByteWriter) -> (usize, Option<CodecError>) {
            for ((i,j), ch) in input.index_iter() {
                if ch <= '\u{7f}' && ch != self.prohibit {
                    if self.toggle && !self.prepend.is_empty() {
                        output.write_bytes(self.prepend.as_bytes());
                    }
                    output.write_byte(ch as u8);
                    if ch == 'e' {
                        self.toggle = !self.toggle;
                    }
                } else {
                    return (i, Some(CodecError { upto: j as isize,
                                                 cause: "!!!".into() }));
                }
            }
            (input.len(), None)
        }
        fn raw_finish(&mut self, _output: &mut ByteWriter) -> Option<CodecError> { None }
    }

    struct MyEncoding { flag: bool, prohibit: char, prepend: &'static str }
    impl Encoding for MyEncoding {
        fn name(&self) -> &'static str { "my encoding" }
        fn raw_encoder(&self) -> Box<RawEncoder> {
            Box::new(MyEncoder { flag: self.flag,
                                 prohibit: self.prohibit,
                                 prepend: self.prepend,
                                 toggle: false })
        }
        fn raw_decoder(&self) -> Box<RawDecoder> { panic!("not supported") }
    }

    #[test]
    fn test_encoding_debug_format() {
        let enc = MyEncoding {
            flag: true,
            prohibit: '\u{80}',
            prepend: "",
        };

        assert_eq!(format!("{:?}", &enc as &Encoding), "Encoding(my encoding)");
    }

    #[test]
    fn test_reencoding_trap_with_ascii_compatible_encoding() {
        static COMPAT: &'static MyEncoding =
            &MyEncoding { flag: true, prohibit: '\u{80}', prepend: "" };
        static INCOMPAT: &'static MyEncoding =
            &MyEncoding { flag: false, prohibit: '\u{80}', prepend: "" };

        assert_eq!(COMPAT.encode("Hello\u{203d} I'm fine.", NcrEscape),
                   Ok(b"Hello&#8253; I'm fine.".to_vec()));
        assert_eq!(INCOMPAT.encode("Hello\u{203d} I'm fine.", NcrEscape),
                   Ok(b"Hello&#8253; I'm fine.".to_vec()));
    }

    #[test]
    fn test_reencoding_trap_with_ascii_incompatible_encoding() {
        static COMPAT: &'static MyEncoding =
            &MyEncoding { flag: true, prohibit: '\u{80}', prepend: "*" };
        static INCOMPAT: &'static MyEncoding =
            &MyEncoding { flag: false, prohibit: '\u{80}', prepend: "*" };

        // this should behave incorrectly as the encoding broke the assumption.
        assert_eq!(COMPAT.encode("Hello\u{203d} I'm fine.", NcrEscape),
                   Ok(b"He*l*l*o&#8253;* *I*'*m* *f*i*n*e.".to_vec()));
        assert_eq!(INCOMPAT.encode("Hello\u{203d} I'm fine.", NcrEscape),
                   Ok(b"He*l*l*o*&*#*8*2*5*3*;* *I*'*m* *f*i*n*e.".to_vec()));
    }

    #[test]
    fn test_encoding_sendable() {
        static COMPAT: &'static MyEncoding =
            &MyEncoding { flag: true, prohibit: '\u{80}', prepend: "*" };
        let encoder = COMPAT.raw_encoder();
        let (tx, _rx) = channel();
        let _ = tx.send(encoder);
    }

    #[test]
    #[should_panic]
    fn test_reencoding_trap_can_fail() {
        static FAIL: &'static MyEncoding = &MyEncoding { flag: false, prohibit: '&', prepend: "" };

        // this should fail as this contrived encoding does not support `&` at all
        let _ = FAIL.encode("Hello\u{203d} I'm fine.", NcrEscape);
    }
}
