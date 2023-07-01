// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Streams of tendrils.

use tendril::{Tendril, Atomicity, NonAtomic};
use fmt;

use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::path::Path;

use encoding::{EncodingRef, RawDecoder};
use utf8;

/// Trait for types that can process a tendril.
///
/// This is a "push" interface, unlike the "pull" interface of
/// `Iterator<Item=Tendril<F>>`. The push interface matches
/// [html5ever][] and other incremental parsers with a similar
/// architecture.
///
/// [html5ever]: https://github.com/servo/html5ever
pub trait TendrilSink<F, A=NonAtomic>
    where F: fmt::Format,
          A: Atomicity,
{
    /// Process this tendril.
    fn process(&mut self, t: Tendril<F, A>);

    /// Indicates that an error has occurred.
    fn error(&mut self, desc: Cow<'static, str>);

    /// What the overall result of processing is.
    type Output;

    /// Indicates the end of the stream.
    fn finish(self) -> Self::Output;

    /// Process one tendril and finish.
    fn one<T>(mut self, t: T) -> Self::Output where Self: Sized, T: Into<Tendril<F, A>> {
        self.process(t.into());
        self.finish()
    }

    /// Consume an iterator of tendrils, processing each item, then finish.
    fn from_iter<I>(mut self, i: I) -> Self::Output
    where Self: Sized, I: IntoIterator, I::Item: Into<Tendril<F, A>> {
        for t in i {
            self.process(t.into())
        }
        self.finish()
    }

    /// Read from the given stream of bytes until exhaustion and process incrementally,
    /// then finish. Return `Err` at the first I/O error.
    fn read_from<R>(mut self, r: &mut R) -> io::Result<Self::Output>
    where Self: Sized, R: io::Read, F: fmt::SliceFormat<Slice=[u8]> {
        const BUFFER_SIZE: u32 = 4 * 1024;
        loop {
            let mut tendril = Tendril::<F, A>::new();
            // FIXME: this exposes uninitialized bytes to a generic R type
            // this is fine for R=File which never reads these bytes,
            // but user-defined types might.
            // The standard library pushes zeros to `Vec<u8>` for that reason.
            unsafe {
                tendril.push_uninitialized(BUFFER_SIZE);
            }
            loop {
                match r.read(&mut tendril) {
                    Ok(0) => return Ok(self.finish()),
                    Ok(n) => {
                        tendril.pop_back(BUFFER_SIZE - n as u32);
                        self.process(tendril);
                        break
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e)
                }
            }
        }
    }


    /// Read from the file at the given path and process incrementally,
    /// then finish. Return `Err` at the first I/O error.
    fn from_file<P>(self, path: P) -> io::Result<Self::Output>
    where Self: Sized, P: AsRef<Path>, F: fmt::SliceFormat<Slice=[u8]> {
        self.read_from(&mut try!(File::open(path)))
    }
}

/// A `TendrilSink` adaptor that takes bytes, decodes them as UTF-8,
/// lossily replace ill-formed byte sequences with U+FFFD replacement characters,
/// and emits Unicode (`StrTendril`).
///
/// This does not allocate memory: the output is either subtendrils on the input,
/// on inline tendrils for a single code point.
pub struct Utf8LossyDecoder<Sink, A=NonAtomic>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity
{
    decoder: utf8::Decoder,
    pub inner_sink: Sink,
    marker: PhantomData<A>,
}

impl<Sink, A> Utf8LossyDecoder<Sink, A>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity,
{
    /// Create a new incremental UTF-8 decoder.
    #[inline]
    pub fn new(inner_sink: Sink) -> Self {
        Utf8LossyDecoder {
            decoder: utf8::Decoder::new(),
            inner_sink: inner_sink,
            marker: PhantomData,
        }
    }
}

impl<Sink, A> TendrilSink<fmt::Bytes, A> for Utf8LossyDecoder<Sink, A>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity,
{
    #[inline]
    fn process(&mut self, t: Tendril<fmt::Bytes, A>) {
        let mut input = &*t;
        loop {
            let (ch, s, result) = self.decoder.decode(input);
            if !ch.is_empty() {
                self.inner_sink.process(Tendril::from_slice(&*ch));
            }
            if !s.is_empty() {
                // rust-utf8 promises that `s` is a subslice of `&*t`
                // so this substraction won’t underflow and `subtendril()` won’t panic.
                let offset = s.as_ptr() as usize - t.as_ptr() as usize;
                let subtendril = t.subtendril(offset as u32, s.len() as u32);
                unsafe {
                    self.inner_sink.process(subtendril.reinterpret_without_validating());
                }
            }
            match result {
                utf8::Result::Ok | utf8::Result::Incomplete => break,
                utf8::Result::Error { remaining_input_after_error: remaining } => {
                    self.inner_sink.error("invalid byte sequence".into());
                    self.inner_sink.process(Tendril::from_slice(utf8::REPLACEMENT_CHARACTER));
                    input = remaining;
                }
            }
        }
    }

    #[inline]
    fn error(&mut self, desc: Cow<'static, str>) {
        self.inner_sink.error(desc);
    }

    type Output = Sink::Output;

    #[inline]
    fn finish(mut self) -> Sink::Output {
        if self.decoder.has_incomplete_sequence() {
            self.inner_sink.error("incomplete byte sequence at end of stream".into());
            self.inner_sink.process(Tendril::from_slice(utf8::REPLACEMENT_CHARACTER));
        }
        self.inner_sink.finish()
    }
}

/// A `TendrilSink` adaptor that takes bytes, decodes them as the given character encoding,
/// lossily replace ill-formed byte sequences with U+FFFD replacement characters,
/// and emits Unicode (`StrTendril`).
///
/// This allocates new tendrils for encodings other than UTF-8.
pub struct LossyDecoder<Sink, A=NonAtomic>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity {
    inner: LossyDecoderInner<Sink, A>,
}

enum LossyDecoderInner<Sink, A>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity {
    Utf8(Utf8LossyDecoder<Sink, A>),
    Other(Box<RawDecoder>, Sink)
}

impl<Sink, A> LossyDecoder<Sink, A>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity,
{
    /// Create a new incremental decoder.
    #[inline]
    pub fn new(encoding: EncodingRef, sink: Sink) -> LossyDecoder<Sink, A> {
        if encoding.name() == "utf-8" {
            LossyDecoder::utf8(sink)
        } else {
            LossyDecoder {
                inner:  LossyDecoderInner::Other(encoding.raw_decoder(), sink)
            }
        }
    }

    /// Create a new incremental decoder for the UTF-8 encoding.
    ///
    /// This is useful for content that is known at run-time to be UTF-8
    /// (whereas `Utf8LossyDecoder` requires knowning at compile-time.)
    #[inline]
    pub fn utf8(sink: Sink) -> LossyDecoder<Sink, A> {
        LossyDecoder {
            inner: LossyDecoderInner::Utf8(Utf8LossyDecoder::new(sink))
        }
    }

    /// Give a reference to the inner sink.
    pub fn inner_sink(&self) -> &Sink {
        match self.inner {
            LossyDecoderInner::Utf8(ref utf8) => &utf8.inner_sink,
            LossyDecoderInner::Other(_, ref inner_sink) => inner_sink,
        }
    }

    /// Give a mutable reference to the inner sink.
    pub fn inner_sink_mut(&mut self) -> &mut Sink {
        match self.inner {
            LossyDecoderInner::Utf8(ref mut utf8) => &mut utf8.inner_sink,
            LossyDecoderInner::Other(_, ref mut inner_sink) => inner_sink,
        }
    }
}

impl<Sink, A> TendrilSink<fmt::Bytes, A> for LossyDecoder<Sink, A>
    where Sink: TendrilSink<fmt::UTF8, A>,
          A: Atomicity,
{
    #[inline]
    fn process(&mut self, mut t: Tendril<fmt::Bytes, A>) {
        let (decoder, sink) = match self.inner {
            LossyDecoderInner::Utf8(ref mut utf8) => return utf8.process(t),
            LossyDecoderInner::Other(ref mut decoder, ref mut sink) => (decoder, sink),
        };

        let mut out = Tendril::new();
        loop {
            match decoder.raw_feed(&*t, &mut out) {
                (_, Some(err)) => {
                    out.push_char('\u{fffd}');
                    sink.error(err.cause);
                    debug_assert!(err.upto >= 0);
                    t.pop_front(err.upto as u32);
                    // continue loop and process remainder of t
                }
                (_, None) => break,
            }
        }
        if out.len() > 0 {
            sink.process(out);
        }
    }

    #[inline]
    fn error(&mut self, desc: Cow<'static, str>) {
        match self.inner {
            LossyDecoderInner::Utf8(ref mut utf8) => utf8.error(desc),
            LossyDecoderInner::Other(_, ref mut sink) => sink.error(desc),
        }
    }

    type Output = Sink::Output;

    #[inline]
    fn finish(self) -> Sink::Output {
        let (mut decoder, mut sink) = match self.inner {
            LossyDecoderInner::Utf8(utf8) => return utf8.finish(),
            LossyDecoderInner::Other(decoder, sink) => (decoder, sink),
        };

        let mut out = Tendril::new();
        if let Some(err) = decoder.raw_finish(&mut out) {
            out.push_char('\u{fffd}');
            sink.error(err.cause);
        }
        if out.len() > 0 {
            sink.process(out);
        }
        sink.finish()
    }
}

#[cfg(test)]
mod test {
    use super::{TendrilSink, LossyDecoder, Utf8LossyDecoder};
    use tendril::{Tendril, Atomicity, SliceExt, NonAtomic};
    use fmt;
    use std::borrow::Cow;
    use encoding::EncodingRef;
    use encoding::all as enc;

    struct Accumulate<A>
        where A: Atomicity,
    {
        tendrils: Vec<Tendril<fmt::UTF8, A>>,
        errors: Vec<String>,
    }

    impl<A> Accumulate<A>
        where A: Atomicity,
    {
        fn new() -> Accumulate<A> {
            Accumulate {
                tendrils: vec![],
                errors: vec![],
            }
        }
    }

    impl<A> TendrilSink<fmt::UTF8, A> for Accumulate<A>
        where A: Atomicity,
    {
        fn process(&mut self, t: Tendril<fmt::UTF8, A>) {
            self.tendrils.push(t);
        }

        fn error(&mut self, desc: Cow<'static, str>) {
            self.errors.push(desc.into_owned());
        }

        type Output = (Vec<Tendril<fmt::UTF8, A>>, Vec<String>);

        fn finish(self) -> Self::Output {
            (self.tendrils, self.errors)
        }
    }

    fn check_utf8(input: &[&[u8]], expected: &[&str], errs: usize) {
        let decoder = Utf8LossyDecoder::new(Accumulate::<NonAtomic>::new());
        let (tendrils, errors) = decoder.from_iter(input.iter().cloned());
        assert_eq!(expected, &*tendrils.iter().map(|t| &**t).collect::<Vec<_>>());
        assert_eq!(errs, errors.len());
    }

    #[test]
    fn utf8() {
        check_utf8(&[], &[], 0);
        check_utf8(&[b""], &[], 0);
        check_utf8(&[b"xyz"], &["xyz"], 0);
        check_utf8(&[b"x", b"y", b"z"], &["x", "y", "z"], 0);

        check_utf8(&[b"xy\xEA\x99\xAEzw"], &["xy\u{a66e}zw"], 0);
        check_utf8(&[b"xy\xEA", b"\x99\xAEzw"], &["xy", "\u{a66e}", "zw"], 0);
        check_utf8(&[b"xy\xEA\x99", b"\xAEzw"], &["xy", "\u{a66e}", "zw"], 0);
        check_utf8(&[b"xy\xEA", b"\x99", b"\xAEzw"], &["xy", "\u{a66e}", "zw"], 0);
        check_utf8(&[b"\xEA", b"", b"\x99", b"", b"\xAE"], &["\u{a66e}"], 0);
        check_utf8(&[b"", b"\xEA", b"", b"\x99", b"", b"\xAE", b""], &["\u{a66e}"], 0);

        check_utf8(&[b"xy\xEA", b"\xFF", b"\x99\xAEz"],
            &["xy", "\u{fffd}", "\u{fffd}", "\u{fffd}", "\u{fffd}", "z"], 4);
        check_utf8(&[b"xy\xEA\x99", b"\xFFz"],
            &["xy", "\u{fffd}", "\u{fffd}", "z"], 2);

        check_utf8(&[b"\xC5\x91\xC5\x91\xC5\x91"], &["őőő"], 0);
        check_utf8(&[b"\xC5\x91", b"\xC5\x91", b"\xC5\x91"], &["ő", "ő", "ő"], 0);
        check_utf8(&[b"\xC5", b"\x91\xC5", b"\x91\xC5", b"\x91"],
            &["ő", "ő", "ő"], 0);
        check_utf8(&[b"\xC5", b"\x91\xff", b"\x91\xC5", b"\x91"],
            &["ő", "\u{fffd}", "\u{fffd}", "ő"], 2);

        // incomplete char at end of input
        check_utf8(&[b"\xC0"], &["\u{fffd}"], 1);
        check_utf8(&[b"\xEA\x99"], &["\u{fffd}"], 1);
    }

    fn check_decode(enc: EncodingRef, input: &[&[u8]], expected: &str, errs: usize) {
        let mut decoder = LossyDecoder::new(enc, Accumulate::new());
        for x in input {
            decoder.process(x.to_tendril());
        }
        let (tendrils, errors) = decoder.finish();
        let mut tendril: Tendril<fmt::UTF8> = Tendril::new();
        for t in tendrils {
            tendril.push_tendril(&t);
        }
        assert_eq!(expected, &*tendril);
        assert_eq!(errs, errors.len());
    }

    #[test]
    fn decode_ascii() {
        check_decode(enc::ASCII, &[], "", 0);
        check_decode(enc::ASCII, &[b""], "", 0);
        check_decode(enc::ASCII, &[b"xyz"], "xyz", 0);
        check_decode(enc::ASCII, &[b"xy", b"", b"", b"z"], "xyz", 0);
        check_decode(enc::ASCII, &[b"x", b"y", b"z"], "xyz", 0);

        check_decode(enc::ASCII, &[b"\xFF"], "\u{fffd}", 1);
        check_decode(enc::ASCII, &[b"x\xC0yz"], "x\u{fffd}yz", 1);
        check_decode(enc::ASCII, &[b"x", b"\xC0y", b"z"], "x\u{fffd}yz", 1);
        check_decode(enc::ASCII, &[b"x\xC0yz\xFF\xFFw"], "x\u{fffd}yz\u{fffd}\u{fffd}w", 3);
    }

    #[test]
    fn decode_utf8() {
        check_decode(enc::UTF_8, &[], "", 0);
        check_decode(enc::UTF_8, &[b""], "", 0);
        check_decode(enc::UTF_8, &[b"xyz"], "xyz", 0);
        check_decode(enc::UTF_8, &[b"x", b"y", b"z"], "xyz", 0);

        check_decode(enc::UTF_8, &[b"\xEA\x99\xAE"], "\u{a66e}", 0);
        check_decode(enc::UTF_8, &[b"\xEA", b"\x99\xAE"], "\u{a66e}", 0);
        check_decode(enc::UTF_8, &[b"\xEA\x99", b"\xAE"], "\u{a66e}", 0);
        check_decode(enc::UTF_8, &[b"\xEA", b"\x99", b"\xAE"], "\u{a66e}", 0);
        check_decode(enc::UTF_8, &[b"\xEA", b"", b"\x99", b"", b"\xAE"], "\u{a66e}", 0);
        check_decode(enc::UTF_8, &[b"", b"\xEA", b"", b"\x99", b"", b"\xAE", b""], "\u{a66e}", 0);

        check_decode(enc::UTF_8, &[b"xy\xEA", b"\x99\xAEz"], "xy\u{a66e}z", 0);
        check_decode(enc::UTF_8, &[b"xy\xEA", b"\xFF", b"\x99\xAEz"],
            "xy\u{fffd}\u{fffd}\u{fffd}\u{fffd}z", 4);
        check_decode(enc::UTF_8, &[b"xy\xEA\x99", b"\xFFz"],
            "xy\u{fffd}\u{fffd}z", 2);

        // incomplete char at end of input
        check_decode(enc::UTF_8, &[b"\xC0"], "\u{fffd}", 1);
        check_decode(enc::UTF_8, &[b"\xEA\x99"], "\u{fffd}", 1);
    }

    #[test]
    fn decode_koi8_u() {
        check_decode(enc::KOI8_U, &[b"\xfc\xce\xc5\xd2\xc7\xc9\xd1"], "Энергия", 0);
        check_decode(enc::KOI8_U, &[b"\xfc\xce", b"\xc5\xd2\xc7\xc9\xd1"], "Энергия", 0);
        check_decode(enc::KOI8_U, &[b"\xfc\xce", b"\xc5\xd2\xc7", b"\xc9\xd1"], "Энергия", 0);
        check_decode(enc::KOI8_U, &[b"\xfc\xce", b"", b"\xc5\xd2\xc7", b"\xc9\xd1", b""], "Энергия", 0);
    }

    #[test]
    fn decode_windows_949() {
        check_decode(enc::WINDOWS_949, &[], "", 0);
        check_decode(enc::WINDOWS_949, &[b""], "", 0);
        check_decode(enc::WINDOWS_949, &[b"\xbe\xc8\xb3\xe7"], "안녕", 0);
        check_decode(enc::WINDOWS_949, &[b"\xbe", b"\xc8\xb3\xe7"], "안녕", 0);
        check_decode(enc::WINDOWS_949, &[b"\xbe", b"", b"\xc8\xb3\xe7"], "안녕", 0);
        check_decode(enc::WINDOWS_949, &[b"\xbe\xc8\xb3\xe7\xc7\xcf\xbc\xbc\xbf\xe4"],
            "안녕하세요", 0);
        check_decode(enc::WINDOWS_949, &[b"\xbe\xc8\xb3\xe7\xc7"], "안녕\u{fffd}", 1);

        check_decode(enc::WINDOWS_949, &[b"\xbe", b"", b"\xc8\xb3"], "안\u{fffd}", 1);
        check_decode(enc::WINDOWS_949, &[b"\xbe\x28\xb3\xe7"], "\u{fffd}(녕", 1);
    }

    #[test]
    fn read_from() {
        let decoder = Utf8LossyDecoder::new(Accumulate::<NonAtomic>::new());
        let mut bytes: &[u8] = b"foo\xffbar";
        let (tendrils, errors) = decoder.read_from(&mut bytes).unwrap();
        assert_eq!(&*tendrils.iter().map(|t| &**t).collect::<Vec<_>>(),
                   &["foo", "\u{FFFD}", "bar"]);
        assert_eq!(errors, &["invalid byte sequence"]);
    }
}
