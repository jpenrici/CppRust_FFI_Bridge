// Executes the process_text function from the C++ library.

use smart_rust::{rust_process, verify_library};

fn main() {
    println!("Checking C++ library ...");
    match verify_library() {
        Ok(_) => println!("Library loaded and functions verified."),
        Err(err) => panic!("Error loading library: {}", err),
    }

    let input = "Rust using C++ library.";
    match rust_process(input) {
        Ok(output) => println!("C++ result: {output}"),
        Err(err) => eprintln!("Failed to execute function: {err}"),
    }
}
