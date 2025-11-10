// Check and configure the link with the C++ library.

fn main() {
    const LIB_PATH: &str = "../cpp/lib/libsmartex.so";

    if !std::path::Path::new(LIB_PATH).exists() {
        panic!("C++ library not found in {}", LIB_PATH);
    }

    // Tells the Rust linker where to look and which library to use.
    println!("cargo:rustc-link-search=native=../cpp/lib"); // search for .so
    println!("cargo:rustc-link-lib=smartex"); // name of the library

    // Includes RPATH in the final binary (allows execution without setting LD_LIBRARY_PATH)
    println!("cargo:rustc-link-arg=-Wl,-rpath,../cpp/lib"); // ensures runtime loading
}
