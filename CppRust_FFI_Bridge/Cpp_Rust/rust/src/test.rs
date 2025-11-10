use smart_rust::{rust_process, verify_library};

#[test]
fn test_library_load() {
    assert!(verify_library().is_ok(), "Failed to load C++ library.");
}

#[test]
fn test_process_function() {
    let result = rust_process("Test Rust with C++ library").unwrap();
    assert!(result.contains("C++"), "C++ did not process correctly.");
}
