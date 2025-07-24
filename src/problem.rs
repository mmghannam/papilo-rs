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
    pub fn add_col(&mut self, lb: f64, ub: f64, integer: bool, cost: f64, name: &str) -> usize {
        let c_name = CString::new(name).expect("Failed to create CString");
        unsafe {
            ffi::papilo_problem_add_col(
                self.raw,
                lb,
                ub,
                integer.into(),
                cost,
                c_name.as_ptr(),
            ).try_into()
                .expect("Failed to add column")
        }
    }


    /// Adds a row to the problem.
    pub fn add_row(&mut self, name: &str, coefficients: &[(usize, f64)], lhs: f64, rhs: f64) -> usize {
        let c_name = CString::new(name).expect("Failed to create CString");
        let row_id = unsafe {
            ffi::papilo_problem_add_generic_row(
                self.raw,
                lhs,
                rhs,
                c_name.as_ptr(),
            )
        };

        for &(col_id, coeff) in coefficients {
            unsafe {
                ffi::papilo_problem_add_nonzero(self.raw, row_id, col_id as i32, coeff);
            }
        }


        row_id as usize
    }
}

impl Drop for Problem {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::papilo_problem_free(self.raw) };
        }
    }
}
