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
            ffi::papilo_problem_add_row(
                self.raw,
                lhs,
                rhs,
                c_name.as_ptr(),
            )
        } as usize;

        for &(col_id, coeff) in coefficients {
            self.set_row_coef(row_id, col_id, coeff);
        }


        row_id
    }


    /// Sets a coefficient for a specific row and column.
    pub fn set_row_coef(&mut self, row_id: usize, col_id: usize, value: f64) {
        unsafe {
            ffi::papilo_problem_add_nonzero(self.raw, row_id as i32, col_id as i32, value);
        }
    }

    /// Returns the number of columns (variables) currently in the problem.
    pub fn num_cols(&self) -> usize {
        unsafe { ffi::papilo_problem_get_num_cols(self.raw) as usize }
    }

    /// Returns the number of rows (constraints) currently in the problem.
    pub fn num_rows(&self) -> usize {
        unsafe { ffi::papilo_problem_get_num_rows(self.raw) as usize }
    }

    /// Changes the lower bound of a column.
    pub fn change_col_lb(&mut self, col_id: usize, lb: f64) {
        unsafe { ffi::papilo_problem_change_col_lb(self.raw, col_id as i32, lb) };
    }

    /// Changes the upper bound of a column.
    pub fn change_col_ub(&mut self, col_id: usize, ub: f64) {
        unsafe { ffi::papilo_problem_change_col_ub(self.raw, col_id as i32, ub) };
    }

    /// Changes the objective coefficient of a column.
    pub fn change_col_obj(&mut self, col_id: usize, obj: f64) {
        unsafe { ffi::papilo_problem_change_col_obj(self.raw, col_id as i32, obj) };
    }

    /// Changes whether a column is integer-constrained.
    pub fn change_col_integral(&mut self, col_id: usize, integral: bool) {
        unsafe {
            ffi::papilo_problem_change_col_integral(self.raw, col_id as i32, integral.into())
        };
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Problem {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::papilo_problem_free(self.raw) };
        }
    }
}
