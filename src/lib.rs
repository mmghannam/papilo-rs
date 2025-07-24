#![deny(missing_docs)]
//! Rust bindings for the PaPILO presolving library.


/// Contains wrappers for the Papilo_Solver struct and its methods.
pub mod solver;
/// Contains wrappers for the Papilo_Problem struct and its methods.
pub mod problem;

pub use papilo_sys as ffi;
