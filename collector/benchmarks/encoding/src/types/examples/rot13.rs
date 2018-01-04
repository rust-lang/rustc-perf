// This is a part of rust-encoding.
// Copyright (c) 2015, Simonas Kazlauskas
// See README.md and LICENSE.txt for details.

/**
 * Rotation by 13 places encoding.
 *
 * This example demonstrates how to use encoding-types package to implement your own encodings.
 */

extern crate encoding_types;

use encoding_types::*;
use std::str::from_utf8;

#[derive(Clone, Copy)]
pub struct ROT13Encoding;


impl Encoding for ROT13Encoding {
    fn name(&self) -> &'static str { "rot13" }
    fn whatwg_name(&self) -> Option<&'static str> { None }
    fn raw_encoder(&self) -> Box<RawEncoder> { ROT13Encoder::new() }
    fn raw_decoder(&self) -> Box<RawDecoder> { ROT13Decoder::new() }
}

#[derive(Clone, Copy)]
pub struct ROT13Encoder;

impl ROT13Encoder {
    pub fn new() -> Box<RawEncoder> {
        Box::new(ROT13Encoder)
    }
}

impl RawEncoder for ROT13Encoder {
    fn from_self(&self) -> Box<RawEncoder> {
        ROT13Encoder::new()
    }
    fn is_ascii_compatible(&self) -> bool { true }

    fn raw_feed(&mut self, input: &str, output: &mut ByteWriter) -> (usize, Option<CodecError>) {
        output.writer_hint(input.len());
        for byte in input.bytes() {
            output.write_byte(rotate_byte(byte))
        }
        (input.len(), None)
    }

    fn raw_finish(&mut self, _output: &mut ByteWriter) -> Option<CodecError> {
        None
    }
}


#[derive(Clone, Copy)]
pub struct ROT13Decoder;

impl ROT13Decoder {
    pub fn new() -> Box<RawDecoder> {
        Box::new(ROT13Decoder)
    }
}

impl RawDecoder for ROT13Decoder {
    fn from_self(&self) -> Box<RawDecoder> {
        ROT13Decoder::new()
    }
    fn is_ascii_compatible(&self) -> bool { true }

    fn raw_feed(&mut self, input: &[u8], output: &mut StringWriter) -> (usize, Option<CodecError>) {
        output.writer_hint(input.len());
        let string = match from_utf8(input) {
            Ok(s) => s,
            Err(_e) => return (0, Some(CodecError {
                upto: 0, // Utf8Error::valid_up_to is unstable at the time of writing,
                         // therefore we can’t quite do partial parsing yet
                cause: "input is not valid utf-8".into()
            }))
        };
        for ch in string.chars() {
            match ch {
                'a'...'z' | 'A'...'Z' => { output.write_char(rotate_byte(ch as u8) as char) },
                _ => { output.write_char(ch) }
            }
        }
        (input.len(), None)
    }

    fn raw_finish(&mut self, _output: &mut StringWriter) -> Option<CodecError> {
        None
    }
}

fn rotate_byte(byte: u8) -> u8 {
    match byte {
        b'a'...b'm' | b'A'...b'M' => { byte + 13 }
        b'n'...b'z' | b'N'...b'Z' => { byte - 13 }
        _ => { byte }
    }
}

fn main() {
    use std::io;
    use std::io::Read;

    let mut ret = Vec::new();
    io::stdin().read_to_end(&mut ret).ok().expect("cannot read from the input");
    match ROT13Encoding.decode(&ret, DecoderTrap::Strict) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("decoder error: {}", e),
    };
}
