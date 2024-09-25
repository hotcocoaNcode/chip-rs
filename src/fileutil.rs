pub fn load_bytes(fname: String) -> Vec<u8> {
    let res = std::fs::read(fname.clone());
    if res.is_err() {
        panic!("Failed to open {}", fname);
    }
    res.unwrap()
}