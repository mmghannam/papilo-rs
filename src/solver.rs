use crate::problem::Problem;
use crate::ffi;


/// A struct representing a solver instance in the PaPILO library.
pub struct Solver {
    raw: *mut ffi::Papilo_Solver,
}

impl Solver {
    /// Returns a raw pointer to the underlying `Papilo_Solver`.
    pub fn raw(&self) -> *mut ffi::Papilo_Solver {
        self.raw
    }

    /// Creates a new instance of the `Solver`.
    pub fn new() -> Self {
        let raw = unsafe { ffi::papilo_solver_create() };
        if raw.is_null() {
            panic!("Failed to create a new Solver instance");
        }
        Self { raw }
    }

    /// Loads a problem into the solver.
    pub fn load_problem(&mut self, problem: Problem) {
        unsafe {
            ffi::papilo_solver_load_problem(self.raw, problem.raw());
        }
    }

    /// Starts the solver with the loaded problem.
    pub fn start(&mut self) {
        unsafe {
            ffi::papilo_solver_start(self.raw);
        }
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


    #[test]
    fn empty_problem_load() {
        let problem = Problem::new();
        let mut solver = Solver::new();
        solver.load_problem(problem);
        assert!(!solver.raw.is_null(), "Solver instance should not be null after loading problem");
        solver.start();
        // The drop methods will be called automatically at the end of the test
    }
}