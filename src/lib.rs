#![deny(missing_docs)]
//! Rust bindings for the PaPILO presolving library.
//! # Example
//! ```rust
//! use papilo_rs::solver::Solver;
//! use papilo_rs::problem::Problem;
//!
//!  let mut problem = Problem::new();
//!  let x = problem.add_col(1.0, 10.0, true, 10.0, "x1");
//!  problem.add_row("r1", &[(x, 1.0)], 2.5, f64::INFINITY);
//!
//!  let mut solver = Solver::new();
//!  solver.load_problem(problem);
//!  let res = solver.start();
//!  assert_eq!(res.dualbound, 30.0);
//! ```


/// Contains wrappers for the Papilo_Problem struct and its methods.
pub mod problem;
/// Contains wrappers for the Papilo_Solver struct and its methods.
pub mod solver;

/// Re-export the FFI bindings to allow direct access to the underlying C functions.
pub use papilo_sys as ffi;
