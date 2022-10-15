use std::io::{BufRead, BufReader, Cursor, Write};

use snap::{read::FrameDecoder, write::FrameEncoder};

use benchlib::{black_box, define_benchmark, run_benchmark_group};

// Inspired by https://github.com/rust-lang/rust/issues/102727
// The pattern we want is a BufReader which wraps a Read impl where one Read::read call will
// never fill the whole BufReader buffer.
fn read_buf(
    mut reader: BufReader<FrameDecoder<Cursor<Vec<u8>>>>,
) -> BufReader<FrameDecoder<Cursor<Vec<u8>>>> {
    while let Ok(buf) = reader.fill_buf() {
        if buf.is_empty() {
            break;
        }
        black_box(buf);
        let len = buf.len();
        reader.consume(len);
    }
    reader
}

fn main() {
    run_benchmark_group(|group| {
        define_benchmark!(group, "bufreader_snappy", read_buf, {
            const BYTES: usize = 64 * 1024 * 1024;

            let data = vec![0u8; BYTES];
            let mut compressed = Vec::new();
            FrameEncoder::new(&mut compressed)
                .write_all(&data[..])
                .unwrap();
            BufReader::with_capacity(BYTES, FrameDecoder::new(Cursor::new(compressed)))
        });
    });
}
