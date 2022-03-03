const MY_PRELOAD_LIB: &[u8] = include_bytes!(env!("CARGO_CDYLIB_FILE_WASM_MODULE"));

fn main() {
    println!("Hello, wasm! ({} bytes)", MY_PRELOAD_LIB.len());
}
