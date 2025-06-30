const BLOB_BINARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/blob-binary"));
const BLOB_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/blob-string"));

fn main() {
    println!("Binary blob: {BLOB_BINARY:?}");
    println!("String blob: {BLOB_STRING:?}");
}
