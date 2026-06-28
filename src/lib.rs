#![deny(missing_docs)]
//! Rust bindings for the [PaPILO](https://github.com/scipopt/papilo) presolving
//! library.
//!
//! This crate lets you build a MIP/LP model, run PaPILO's presolving on it,
//! inspect the resulting reduced problem, and map a solution of the reduced
//! problem back to the original variable space. It does **not** solve the
//! problem — solve the reduced problem with a solver of your choice, then
//! postsolve.
//!
//! # Example
//! ```rust
//! use papilo_rs::presolver::Presolver;
//! use papilo_rs::problem::Problem;
//!
//! // Build a model.
//! let mut problem = Problem::new();
//! let x = problem.add_col(1.0, 10.0, true, 10.0, "x1");
//! problem.add_row("r1", &[(x, 1.0)], 2.5, f64::INFINITY);
//!
//! // Presolve it.
//! let mut presolver = Presolver::new();
//! let status = presolver.presolve(problem);
//!
//! // Inspect the reduced problem.
//! let reduced = presolver.reduced_problem();
//!
//! // Solve the reduced problem yourself, then map the solution back. (Here the
//! // problem is fully solved during presolving, so the reduced solution is
//! // empty.)
//! let reduced_solution = vec![0.0; reduced.num_cols];
//! let original = presolver.postsolve(&reduced_solution).unwrap();
//! assert_eq!(original, vec![3.0]);
//! ```

/// Solver parameter handling.
pub mod param;
/// Types describing the result of presolving and postsolving.
pub mod presolve;
/// Contains the `Presolver` type and its methods.
pub mod presolver;
/// Contains wrappers for the Papilo_Problem struct and its methods.
pub mod problem;

/// Re-export the FFI bindings to allow direct access to the underlying C functions.
pub use papilo_sys as ffi;
