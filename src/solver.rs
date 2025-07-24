use crate::ffi;
use crate::problem::Problem;
use papilo_sys::PAPILO_SOLVE_RESULT;

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
    pub fn start(&mut self) -> SolveInfo {
        SolveInfo {
            raw: unsafe { *ffi::papilo_solver_start(self.raw) },
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


/// A struct representing the solving information returned by the solver.
#[derive(Debug)]
pub struct SolveInfo {
    raw: ffi::PAPILO_SOLVING_INFO,
}

impl SolveInfo {
    /// Returns the dual bound of the solution.
    pub fn dualbound(&self) -> f64 {
        self.raw.dualbound
    }

    /// Returns the solving time.
    pub fn solving_time(&self) -> f64 {
        self.raw.solvingtime
    }

    /// Returns the presolve time.
    pub fn presolve_time(&self) -> f64 {
        self.raw.presolvetime
    }

    /// Returns the best solution objective value.
    pub fn best_solution_objective(&self) -> f64 {
        self.raw.bestsol_obj
    }

    /// Returns the best solution integer violation.
    pub fn best_solution_integer_violation(&self) -> f64 {
        self.raw.bestsol_intviol
    }

    /// Returns the best solution bound violation.
    pub fn best_solution_bound_violation(&self) -> f64 {
        self.raw.bestsol_boundviol
    }

    /// Returns the best solution constraint violation.
    pub fn best_solution_constraint_violation(&self) -> f64 {
        self.raw.bestsol_consviol
    }

    /// Returns the solve result.
    pub fn solve_result(&self) -> SolveResult {
        self.raw.solve_result.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An enum representing the possible results of a solve operation.
pub enum SolveResult {
    /// The problem was solved to optimality.
    Optimal,
    /// The solving stopped early with a feasible solution due to limits or interrupts.
    Feasible,
    /// The solving stopped early and without a solution due to limits or interrupts.
    Stopped,
    /// The problem was detected to be unbounded or infeasible.
    UnboundedOrInfeasible,
    /// The problem was detected to be unbounded.
    Unbounded,
    /// The problem was detected to be infeasible.
    Infeasible,
}

impl From<PAPILO_SOLVE_RESULT> for SolveResult {
    fn from(result: PAPILO_SOLVE_RESULT) -> Self {
        match result {
            ffi::Papilo_SolveResult_PAPILO_SOLVE_RESULT_OPTIMAL => SolveResult::Optimal,
            ffi::Papilo_SolveResult_PAPILO_SOLVE_RESULT_FEASIBLE => SolveResult::Feasible,
            ffi::Papilo_SolveResult_PAPILO_SOLVE_RESULT_STOPPED => SolveResult::Stopped,
            ffi::Papilo_SolveResult_PAPILO_SOLVE_RESULT_UNBND_OR_INFEAS => {
                SolveResult::UnboundedOrInfeasible
            }
            ffi::Papilo_SolveResult_PAPILO_SOLVE_RESULT_UNBOUNDED => SolveResult::Unbounded,
            ffi::Papilo_SolveResult_PAPILO_SOLVE_RESULT_INFEASIBLE => SolveResult::Infeasible,
            _ => panic!("Unknown solve result: {}", result),
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
        assert_eq!(res.dualbound(), 15.0);
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
        assert_eq!(res.dualbound(), 20.0);
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
        assert_eq!(res.solve_result(), SolveResult::Optimal);
        assert_eq!(res.best_solution_objective(), 30.0);
        assert_eq!(res.dualbound(), 30.0);
    }
}
