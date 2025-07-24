# papilo-rs

Rust bindings for the [PaPILO](https://github.com/scipopt/papilo) (Parallel Presolve for Integer and Linear Optimization) library.

## Usage

Add this crate to your project by running:

```bash
cargo add papilo-rs
```

### Example

Then, you can use it in your Rust code as follows:

```rust


```rust
use papilo_rs::{Solver, Problem};

fn main() {
    let mut solver = Solver::new();
    let problem = Problem::new();
    solver.load_problem(problem);
    // add columns and rows to the problem
    let res = solver.start();
    // Retrieve results, etc.
}
```

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

