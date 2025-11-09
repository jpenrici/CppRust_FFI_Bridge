// Safe Rust wrapper for C++ library (dynamic FFI)

#![allow(dead_code)]

use libloading::{Library, Symbol};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Default path to the C++ library
const LIB_PATH: &str = "../cpp/lib/libsmartex.so";

// Defines the expected signature of the C++ function
type ProcessTextFn = unsafe extern "C" fn(*const c_char) -> *const c_char;
type FreeTextFn = unsafe extern "C" fn(*const c_char);

/// Checks if the C++ library is available.
pub fn verify_library() -> Result<(), String> {
    unsafe {
        let lib =
            Library::new(LIB_PATH).map_err(|e| format!("Library not found in {LIB_PATH}: {e}"))?;

        if lib.get::<Symbol<ProcessTextFn>>(b"process_text").is_err() {
            return Err("The process_text() function is missing from the library.".into());
        }

        if lib.get::<Symbol<FreeTextFn>>(b"free_text").is_err() {
            return Err("The free_text() function is missing from the library.".into());
        }

        Ok(())
    }
}

// Safe call from Rust to C++
pub fn rust_process(input: &str) -> Result<String, String> {
    unsafe {
        let lib = Library::new(LIB_PATH).map_err(|e| format!("Error loading lib: {e}"))?;

        let process_text: Symbol<ProcessTextFn> = match lib.get(b"process_text") {
            Ok(sym) => sym,
            Err(e) => return Err(format!("The process_text() function was not found.: {e}")),
        };

        let free_text: Symbol<FreeTextFn> = match lib.get(b"free_text") {
            Ok(sym) => sym,
            Err(e) => return Err(format!("Error locating free_text: {e}")),
        };

        let c_input = CString::new(input).map_err(|_| "Error converting input.".to_string())?;

        let ptr = process_text(c_input.as_ptr());
        if ptr.is_null() {
            return Err("[C++] returned null pointer".to_string());
        }

        let output = CStr::from_ptr(ptr).to_string_lossy().into_owned();

        free_text(ptr);

        Ok(output)
    }
}
