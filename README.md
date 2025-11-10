# CppRust-FFI-Bridge

A minimal interoperability example between **C++** and **Rust**, using **FFI** with dynamic library loading and runtime safety verification.

## Objective
Demonstrate how Rust can safely call C++ functions through an external shared library (`.so`), using `libloading` and `extern "C"` wrappers to ensure memory safety and runtime checks.

## Recommended Documentation

### Rust
- [Rust FFI - The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)
- [The Rust Book - FFI Section](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [libloading crate](https://docs.rs/libloading/latest/libloading/)

### C++
- [C++ Standard Library](https://en.cppreference.com/w/cpp/standard_library.html)
- [cppreference.com - Language linkage](https://en.cppreference.com/w/cpp/language/language_linkage.html)

---
