# papilo-rs

[![][img_crates]][crates] [![][img_doc]][doc]

[img_crates]: https://img.shields.io/crates/v/papilo-rs.svg

[crates]: https://crates.io/crates/papilo-rs

[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

[doc]: https://docs.rs/papilo-rs/

[img_coverage]: https://img.shields.io/codecov/c/github/mmghannam/papilo-rs.svg

Rust bindings for the [PaPILO](https://github.com/scipopt/papilo) (Parallel Presolve for Integer and Linear Optimization) library.

## Usage

Add this crate to your project by running:

```bash
cargo add papilo-rs
```

### Example

Then, you can use it in your Rust code as follows:

```rust
use papilo_rs::{Solver, Problem};

fn main() {
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
```

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

