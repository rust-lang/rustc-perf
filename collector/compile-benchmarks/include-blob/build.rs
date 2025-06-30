use std::path::PathBuf;

fn main() {
    let byte_count = 30usize * 1024 * 1024;
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is missing"));

    // Generate large (30 MiB) blobs of data to include into the resulting program
    let binary_data: Vec<u8> = (0..byte_count).map(|v| (v % 256) as u8).collect();
    std::fs::write(out_dir.join("blob-binary"), binary_data).expect("cannot write binary blob");

    let string_data: String = ('a'..'z').cycle().take(byte_count).collect();
    std::fs::write(out_dir.join("blob-string"), string_data).expect("cannot write string blob");
}
