# CppRust-FFI-Bridge

A minimal interoperability example between **C++** and **Rust**, using **FFI** with dynamic library loading and runtime safety verification.

## Objective
Demonstrate how Rust can safely call C++ functions through an external shared library (`.so`), using `libloading` and `extern "C"` wrappers to ensure memory safety and runtime checks.

## Recommended Documentation

### Rust
- [Rust FFI - The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)
- [libloading crate](https://docs.rs/libloading/latest/libloading/)
- [The Rust Book - FFI Section](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

### C++
- [cppreference.com - extern "C"](https://en.cppreference.com/w/cpp/language/extern)
- [cppreference.com - Dynamic libraries](https://en.cppreference.com/w/cpp/utility/program)
- [ISO C++ Standard Library Overview](https://en.cppreference.com/w/cpp)

---
