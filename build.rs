fn main() {
    // Add the search path for the .rlib file
    println!("cargo:rustc-link-search=compiled_lib");
}
