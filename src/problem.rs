use crate::ffi;
use std::ffi::{CStr, CString};

/// A struct representing a problem in the Papilo library.
pub struct Problem {
    raw: *mut ffi::Papilo_Problem,
}

impl Problem {
    /// Returns a raw pointer to the underlying `Papilo_Problem`.
    pub fn raw(&self) -> *mut ffi::Papilo_Problem {
        self.raw
    }

    /// Creates a new `Problem` instance with default parameters.
    pub fn new() -> Self {
        let problem_name = CStr::from_bytes_until_nul(b"papilo-rs\0").unwrap();
        let raw = unsafe {
            ffi::papilo_problem_create(f64::INFINITY, problem_name.as_ptr(), 1000, 10, 10)
        };
        assert!(!raw.is_null());
        Problem { raw }
    }

    /// Adds a column to the problem.
    pub fn add_col(&mut self, lb: f64, ub: f64, integer: bool, cost: f64, name: &str) {
        let c_name = CString::new(name).expect("Failed to create CString");
        unsafe {
            ffi::papilo_problem_add_col(
                self.raw,
                lb,
                ub,
                integer.into(),
                cost,
                c_name.as_ptr(),
            );
        }
    }
}

impl Drop for Problem {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::papilo_problem_free(self.raw) };
        }
    }
}
