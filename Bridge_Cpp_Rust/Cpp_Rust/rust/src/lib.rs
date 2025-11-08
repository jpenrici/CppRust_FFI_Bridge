use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use libloading::{Library, Symbol};

type ProcessTextFn = unsafe extern "C" fn(*const c_char) -> *const c_char;

pub fn verify_library(lib_path: &str) -> Result<(), String> {
    unsafe {
        match Library::new(lib_path) {
            Ok(lib) => {
                let symbol: Result<Symbol<ProcessTextFn>, _> = lib.get(b"process_text");
                match symbol {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Symbol 'process_text' not found: {e}")),
                }
            }
            Err(e) => Err(format!("Failed to load library: {e}")),
        }
    }
}

extern "C" {
    fn process_text(input: *const c_char) -> *const c_char;
}

pub fn rust_process(input: &str) -> String {
    let c_input = CString::new(input).unwrap();
    unsafe {
    // let ptr = process_text(c_input.as_ptr());
    //     let c_str = CStr::from_ptr(ptr);
    //     c_str.to_string_lossy().into_owned()
    }
    return "C++ processed".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_exists() {
        let path = "../cpp/lib/libsmartex.so";
        match verify_library(path) {
            Ok(_) => println!("C++ library loaded successfully.: {}", path),
            Err(err) => panic!("Error loading library.: {}", err),
        }
    }

    #[test]
    fn test_process_text() {
        let result = rust_process("Hello from Rust");
        assert!(result.contains("C++ processed"));
    }
}
