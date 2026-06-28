# papilo-rs

[![][img_crates]][crates] [![][img_doc]][doc]

[img_crates]: https://img.shields.io/crates/v/papilo-rs.svg

[crates]: https://crates.io/crates/papilo-rs

[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

[doc]: https://docs.rs/papilo-rs/

[img_coverage]: https://img.shields.io/codecov/c/github/mmghannam/papilo-rs.svg

Rust bindings for the [PaPILO](https://github.com/scipopt/papilo) (Parallel Presolve for Integer and Linear Optimization) library.

PaPILO is a *presolver*: it simplifies a MIP/LP before it is handed to a solver.
With these bindings you can:

- build a MIP/LP model programmatically,
- run PaPILO's presolving on it,
- inspect the resulting reduced problem (bounds, objective, constraint matrix, mappings),
- and postsolve a solution of the reduced problem back to the original variable space.

This crate depends only on the header-only PaPILO core, so **no external solver
(SCIP / SoPlex) is required**. Solving the reduced problem is left to a solver of
your choice.

To build against a local checkout of PaPILO that you are editing in place, set
the `PAPILO_SRC` environment variable to its directory.

## Usage

```bash
cargo add papilo-rs
```

```rust
use papilo_rs::presolver::Presolver;
use papilo_rs::problem::Problem;

// Build a model: minimize x + y subject to x + 2y >= 4, with x, y in [0, 10].
let mut problem = Problem::new();
let x = problem.add_col(0.0, 10.0, false, 1.0, "x");
let y = problem.add_col(0.0, 10.0, false, 1.0, "y");
problem.add_row("r0", &[(x, 1.0), (y, 2.0)], 4.0, f64::INFINITY);

// Presolve it.
let mut presolver = Presolver::new();
let status = presolver.presolve(problem);
println!("presolve status: {status:?}");

// Inspect the reduced problem.
let reduced = presolver.reduced_problem();
println!("reduced columns: {}, rows: {}", reduced.num_cols, reduced.num_rows);

// Solve the reduced problem with your own solver, then map the solution back
// to the original variable space.
let reduced_solution = vec![0.0; reduced.num_cols];
let original_solution = presolver.postsolve(&reduced_solution).unwrap();
println!("solution in original space: {original_solution:?}");
```

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

