use crate::ffi;
use crate::problem::Problem;
use ffi::PAPILO_SOLVING_INFO as SolvingInfo;

/// A struct representing a solver instance in the PaPILO library.
pub struct Solver {
    raw: *mut ffi::Papilo_Solver,
    problem: Option<Problem>,
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
        Self { raw, problem: None }
    }

    /// Loads a problem into the solver.
    pub fn load_problem(&mut self, problem: Problem) {
        unsafe {
            ffi::papilo_solver_load_problem(self.raw, problem.raw());
        }
        self.problem = Some(problem);
    }

    /// Starts the solver with the loaded problem.
    pub fn start(&mut self) -> SolvingInfo {
        unsafe { *ffi::papilo_solver_start(self.raw) }
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
        assert!(
            !solver.raw.is_null(),
            "Solver instance should not be null after loading problem"
        );
        let res = solver.start();
        println!("Solving info: {:?}", res);
    }

    #[test]
    fn one_cont_column() {
        let mut problem = Problem::new();
        problem.add_col(1.5, 20.0, false, 10.0, "x2");

        let mut solver = Solver::new();
        solver.load_problem(problem);
        assert!(
            !solver.raw.is_null(),
            "Solver instance should not be null after loading problem with columns"
        );
        let res = solver.start();
        assert_eq!(res.dualbound, 15.0);
    }


    #[test]
    fn one_int_column() {
        let mut problem = Problem::new();
        problem.add_col(1.5, 10.0, true, 10.0, "x1");

        let mut solver = Solver::new();
        solver.load_problem(problem);
        assert!(
            !solver.raw.is_null(),
            "Solver instance should not be null after loading problem with integer columns"
        );
        let res = solver.start();
        assert_eq!(res.dualbound, 20.0);
    }


    #[test]
    fn one_col_one_row() {
        let mut problem = Problem::new();
        let x = problem.add_col(1.0, 10.0, true, 10.0, "x1");
        problem.add_row("r1", &[(x, 1.0)], 2.5, f64::INFINITY);

        let mut solver = Solver::new();
        solver.load_problem(problem);
        assert!(
            !solver.raw.is_null(),
            "Solver instance should not be null after loading problem with integer columns"
        );
        let res = solver.start();
        assert_eq!(res.dualbound, 30.0);
    }
}
