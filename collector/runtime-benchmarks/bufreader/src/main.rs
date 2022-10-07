use benchlib::benchmark::{black_box, BenchmarkSuite};
use snap::{read::FrameDecoder, write::FrameEncoder};
use std::io::{BufRead, BufReader, Write};

const BYTES: usize = 64 * 1024 * 1024;

fn main() {
    let mut suite = BenchmarkSuite::new();

    // Inspired by https://github.com/rust-lang/rust/issues/102727
    // The pattern we want is a BufReader which wraps a Read impl where one Read::read call will
    // never fill the whole BufReader buffer.
    suite.register("bufreader-snappy", || {
        let data = vec![0u8; BYTES];
        move || {
            let mut compressed = Vec::new();
            FrameEncoder::new(&mut compressed)
                .write_all(&data[..])
                .unwrap();
            let mut reader = BufReader::with_capacity(BYTES, FrameDecoder::new(&compressed[..]));

            while let Ok(buf) = reader.fill_buf() {
                if buf.is_empty() {
                    break;
                }
                black_box(buf);
                let len = buf.len();
                reader.consume(len);
            }
        }
    });

    suite.run().unwrap();
}
