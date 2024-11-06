fn main() {
    println!("cargo:rustc-link-search=native=/workspaces/dnd-character-simulator/external_libs");
    println!("cargo:rustc-link-lib=dylib=dnd_character_api");
}
