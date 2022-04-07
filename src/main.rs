use gloo_file::Blob;

fn main() {
    let bytes: &[u8] = &[1, 2, 3, 4];
    let _ = Blob::new(bytes);
}
