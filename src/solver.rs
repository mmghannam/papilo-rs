use crate::ffi;


/// A struct representing a solver instance in the PaPILO library.
pub struct Solver {
    raw: *mut ffi::Papilo_Solver,
}

impl Solver {
    /// Creates a new instance of the `Solver`.
    pub fn new() -> Self {
        let raw = unsafe { ffi::papilo_solver_create() };
        if raw.is_null() {
            panic!("Failed to create a new Solver instance");
        }
        Self { raw }
    }
}


impl Drop for Solver {
    fn drop(&mut self) {
        unsafe {
            ffi::papilo_solver_free(self.raw);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_creation() {
        let solver = Solver::new();
        assert!(!solver.raw.is_null(), "Solver instance should not be null");
        // The drop method will be called automatically at the end of the test
    }
}