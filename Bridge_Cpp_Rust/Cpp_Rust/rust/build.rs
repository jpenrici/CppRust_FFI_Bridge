// Generates the C++ binding and compiles the integrated library
fn main() {
    println!("cargo:rustc-link-search=native=../cpp/lib/");
    println!("cargo:rustc-link-lib=smartex");
}
