use std::path::PathBuf;

fn main() {
    // Path to the DLL stored in external_libs directory
    let dll_dir = PathBuf::from("../external_libs");
    println!("cargo:rustc-link-search=native={}", dll_dir.display());
    println!("cargo:rustc-link-lib=dylib=dnd_character_api");
}
