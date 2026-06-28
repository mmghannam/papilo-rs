//! End-to-end demonstration: build a model, presolve it, inspect the reduced
//! problem, and postsolve a reduced solution back to the original space.
//!
//! Run with: `cargo run --example presolve_postsolve`

use papilo_rs::presolver::Presolver;
use papilo_rs::problem::Problem;

fn build_problem() -> Problem {
    // minimize 10 * x  subject to  x >= 2.5,  x in [1, 10], x integer
    let mut problem = Problem::new();
    let x = problem.add_col(1.0, 10.0, true, 10.0, "x1");
    problem.add_row("r1", &[(x, 1.0)], 2.5, f64::INFINITY);
    problem
}

fn main() {
    let mut presolver = Presolver::new();
    let status = presolver.presolve(build_problem());

    let reduced = presolver.reduced_problem();
    println!("presolve status      : {status:?}");
    println!(
        "original cols / rows : {} / {}",
        presolver.num_cols(),
        presolver.num_rows()
    );
    println!(
        "reduced  cols / rows : {} / {}",
        reduced.num_cols, reduced.num_rows
    );

    // Solve the reduced problem with a solver of your choice. Here it is empty
    // (fully solved in presolving), so the reduced solution is empty.
    let reduced_solution = vec![0.0; reduced.num_cols];
    let original = presolver.postsolve(&reduced_solution).unwrap();
    println!("postsolved solution  : {original:?}");
}
