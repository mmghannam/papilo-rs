use crate::ffi;
use crate::param::{ParamResult, Parameter};
use crate::presolve::{PostsolveError, PresolveStatus, ReducedProblem, WrongLength};
use crate::problem::Problem;

/// Convert a value using PaPILO's infinity sentinel into a Rust `f64`,
/// mapping `>= infinity` to `f64::INFINITY` and `<= -infinity` to
/// `f64::NEG_INFINITY`.
fn from_papilo_inf(value: f64, infinity: f64) -> f64 {
    if value >= infinity {
        f64::INFINITY
    } else if value <= -infinity {
        f64::NEG_INFINITY
    } else {
        value
    }
}

/// A presolver instance.
///
/// A `Presolver` runs PaPILO's presolving on a [`Problem`], exposes the reduced
/// problem, and maps solutions of the reduced problem back to the original
/// variable space. It does not solve the problem itself — solve the reduced
/// problem with a solver of your choice, then call [`postsolve`](Self::postsolve).
pub struct Presolver {
    raw: *mut ffi::Papilo_Presolver,
}

impl Presolver {
    /// Returns a raw pointer to the underlying `Papilo_Presolver`.
    pub fn raw(&self) -> *mut ffi::Papilo_Presolver {
        self.raw
    }

    /// Creates a new presolver with the default presolvers enabled.
    pub fn new() -> Self {
        let raw = unsafe { ffi::papilo_presolver_create() };
        assert!(!raw.is_null(), "Failed to create a new Presolver instance");
        let mut presolver = Self { raw };
        // Quiet by default; ignore if the parameter is unavailable.
        let _ = presolver.set_param("message.verbosity", 0);
        presolver
    }

    /// Sets a parameter for the presolver.
    pub fn set_param<P: Parameter>(&mut self, key: &str, value: P) -> Result<(), ParamResult> {
        P::set(self, key, value)
    }

    /// Runs presolving on the given problem and returns the resulting status.
    ///
    /// The problem is consumed: afterwards, the reduced problem can be inspected
    /// with [`reduced_problem`](Self::reduced_problem) and solutions of the
    /// reduced problem can be mapped back with [`postsolve`](Self::postsolve).
    pub fn presolve(&mut self, problem: Problem) -> PresolveStatus {
        unsafe { ffi::papilo_presolver_load_problem(self.raw, problem.raw()) };
        // `problem` is dropped here, freeing the now-empty C problem.
        unsafe { ffi::papilo_presolver_presolve(self.raw) }.into()
    }

    /// The value of infinity the loaded problem was created with. Bounds and row
    /// sides with absolute value at least this large are treated as infinite.
    pub fn infinity(&self) -> f64 {
        unsafe { ffi::papilo_presolver_get_infinity(self.raw) }
    }

    /// Number of columns (variables) in the original (loaded) problem.
    pub fn num_cols(&self) -> usize {
        unsafe { ffi::papilo_presolver_get_num_cols_original(self.raw) as usize }
    }

    /// Number of rows (constraints) in the original (loaded) problem.
    pub fn num_rows(&self) -> usize {
        unsafe { ffi::papilo_presolver_get_num_rows_original(self.raw) as usize }
    }

    /// Extracts the reduced problem produced by [`presolve`](Self::presolve).
    ///
    /// This must be called after [`presolve`](Self::presolve); otherwise the
    /// returned data is unspecified.
    pub fn reduced_problem(&self) -> ReducedProblem {
        let inf = self.infinity();
        let num_cols = unsafe { ffi::papilo_presolver_get_reduced_num_cols(self.raw) } as usize;
        let num_rows = unsafe { ffi::papilo_presolver_get_reduced_num_rows(self.raw) } as usize;
        let nnz = unsafe { ffi::papilo_presolver_get_reduced_num_nonzeros(self.raw) } as usize;

        let mut col_lower = vec![0.0; num_cols];
        let mut col_upper = vec![0.0; num_cols];
        unsafe {
            ffi::papilo_presolver_get_reduced_col_bounds(
                self.raw,
                col_lower.as_mut_ptr(),
                col_upper.as_mut_ptr(),
            );
        }
        for v in col_lower.iter_mut() {
            *v = from_papilo_inf(*v, inf);
        }
        for v in col_upper.iter_mut() {
            *v = from_papilo_inf(*v, inf);
        }

        let mut objective = vec![0.0; num_cols];
        unsafe { ffi::papilo_presolver_get_reduced_obj(self.raw, objective.as_mut_ptr()) };
        let objective_offset =
            unsafe { ffi::papilo_presolver_get_reduced_obj_offset(self.raw) };

        let mut integral_raw = vec![0u8; num_cols];
        unsafe {
            ffi::papilo_presolver_get_reduced_col_integral(self.raw, integral_raw.as_mut_ptr())
        };
        let col_integral = integral_raw.into_iter().map(|b| b != 0).collect();

        let mut row_lhs = vec![0.0; num_rows];
        let mut row_rhs = vec![0.0; num_rows];
        unsafe {
            ffi::papilo_presolver_get_reduced_row_sides(
                self.raw,
                row_lhs.as_mut_ptr(),
                row_rhs.as_mut_ptr(),
            );
        }
        for v in row_lhs.iter_mut() {
            *v = from_papilo_inf(*v, inf);
        }
        for v in row_rhs.iter_mut() {
            *v = from_papilo_inf(*v, inf);
        }

        let mut row_start_raw = vec![0i32; num_rows + 1];
        let mut col_indices_raw = vec![0i32; nnz];
        let mut values = vec![0.0; nnz];
        unsafe {
            ffi::papilo_presolver_get_reduced_matrix_csr(
                self.raw,
                row_start_raw.as_mut_ptr(),
                col_indices_raw.as_mut_ptr(),
                values.as_mut_ptr(),
            );
        }
        let row_start = row_start_raw.into_iter().map(|i| i as usize).collect();
        let col_indices = col_indices_raw.into_iter().map(|i| i as usize).collect();

        let mut orig_col_raw = vec![0i32; num_cols];
        unsafe {
            ffi::papilo_presolver_get_orig_col_mapping(self.raw, orig_col_raw.as_mut_ptr())
        };
        let orig_col = orig_col_raw.into_iter().map(|i| i as usize).collect();

        let mut orig_row_raw = vec![0i32; num_rows];
        unsafe {
            ffi::papilo_presolver_get_orig_row_mapping(self.raw, orig_row_raw.as_mut_ptr())
        };
        let orig_row = orig_row_raw.into_iter().map(|i| i as usize).collect();

        ReducedProblem {
            num_cols,
            num_rows,
            col_lower,
            col_upper,
            col_integral,
            objective,
            objective_offset,
            row_lhs,
            row_rhs,
            row_start,
            col_indices,
            values,
            orig_col,
            orig_row,
        }
    }

    /// Maps a primal solution of the reduced problem back to the original
    /// variable space.
    ///
    /// `reduced_solution` must have one entry per reduced column (see
    /// [`ReducedProblem::num_cols`]). On success, returns a solution with one
    /// entry per original column (see [`num_cols`](Self::num_cols)).
    ///
    /// This must be called after [`presolve`](Self::presolve).
    pub fn postsolve(&self, reduced_solution: &[f64]) -> Result<Vec<f64>, PostsolveError> {
        let expected =
            unsafe { ffi::papilo_presolver_get_reduced_num_cols(self.raw) } as usize;
        if reduced_solution.len() != expected {
            return Err(PostsolveError::WrongLength {
                expected,
                got: reduced_solution.len(),
            });
        }

        let mut original = vec![0.0; self.num_cols()];
        let status = unsafe {
            ffi::papilo_presolver_postsolve(
                self.raw,
                reduced_solution.as_ptr(),
                original.as_mut_ptr(),
            )
        };

        if status == ffi::Papilo_PostsolveStatus_PAPILO_POSTSOLVE_STATUS_OK {
            Ok(original)
        } else {
            Err(PostsolveError::Failed)
        }
    }

    /// Maps a primal solution of the original problem **forward** to the reduced
    /// problem space (the inverse direction of [`postsolve`](Self::postsolve)).
    ///
    /// `original` must have one entry per original column (see
    /// [`num_cols`](Self::num_cols)). Returns a solution with one entry per
    /// reduced column (see [`ReducedProblem::num_cols`]).
    ///
    /// The forward map is linear and exact, including for aggregating
    /// reductions (parallel-column merges, where a reduced variable is a linear
    /// combination of original variables). For a feasible solution of the
    /// original problem this yields the corresponding reduced solution; in
    /// particular `transform_solution(postsolve(z)) == z`.
    ///
    /// This must be called after [`presolve`](Self::presolve).
    pub fn transform_solution(&self, original: &[f64]) -> Result<Vec<f64>, WrongLength> {
        let expected = self.num_cols();
        if original.len() != expected {
            return Err(WrongLength {
                expected,
                got: original.len(),
            });
        }

        let num_reduced =
            unsafe { ffi::papilo_presolver_get_reduced_num_cols(self.raw) } as usize;
        let mut reduced = vec![0.0; num_reduced];
        unsafe {
            ffi::papilo_presolver_transform_solution(
                self.raw,
                original.as_ptr(),
                reduced.as_mut_ptr(),
            );
        }
        Ok(reduced)
    }
}

impl Default for Presolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Presolver {
    fn drop(&mut self) {
        unsafe {
            ffi::papilo_presolver_free(self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presolver_creation() {
        let presolver = Presolver::new();
        assert!(!presolver.raw.is_null());
    }

    #[test]
    fn presolve_solves_trivial_problem() {
        // This problem is fully solved during presolving.
        let mut problem = Problem::new();
        let x = problem.add_col(1.0, 10.0, true, 10.0, "x1");
        problem.add_row("r1", &[(x, 1.0)], 2.5, f64::INFINITY);

        let mut presolver = Presolver::new();
        let status = presolver.presolve(problem);
        assert_eq!(status, PresolveStatus::Reduced);

        assert_eq!(presolver.num_cols(), 1);
        assert_eq!(presolver.num_rows(), 1);

        let reduced = presolver.reduced_problem();
        assert_eq!(reduced.num_cols, 0);
        assert_eq!(reduced.num_rows, 0);

        // Postsolving the (empty) reduced solution reconstructs x = 3.
        let original = presolver.postsolve(&[]).expect("postsolve should succeed");
        assert_eq!(original, vec![3.0]);
    }

    #[test]
    fn presolve_extracts_reduced_problem() {
        // min x + y ; x,y in [0,10] ; x + 2y >= 4 ; 3x + y >= 5.
        // Disabling dual reductions keeps both columns in the reduced problem.
        let mut problem = Problem::new();
        let x = problem.add_col(0.0, 10.0, false, 1.0, "x");
        let y = problem.add_col(0.0, 10.0, false, 1.0, "y");
        problem.add_row("r0", &[(x, 1.0), (y, 2.0)], 4.0, f64::INFINITY);
        problem.add_row("r1", &[(x, 3.0), (y, 1.0)], 5.0, f64::INFINITY);

        let mut presolver = Presolver::new();
        presolver.set_param("presolve.dualreds", 0).unwrap();
        let status = presolver.presolve(problem);
        assert!(matches!(
            status,
            PresolveStatus::Unchanged | PresolveStatus::Reduced
        ));

        let reduced = presolver.reduced_problem();
        assert_eq!(reduced.num_cols, 2);
        assert_eq!(reduced.num_rows, 2);
        assert_eq!(reduced.num_nonzeros(), 4);
        assert_eq!(reduced.col_lower, vec![0.0, 0.0]);
        assert_eq!(reduced.col_upper, vec![10.0, 10.0]);
        assert_eq!(reduced.objective, vec![1.0, 1.0]);
        assert_eq!(reduced.col_integral, vec![false, false]);
        assert_eq!(reduced.row_lhs, vec![4.0, 5.0]);
        assert_eq!(reduced.row_rhs, vec![f64::INFINITY, f64::INFINITY]);

        let row0: Vec<_> = reduced.row(0).collect();
        let row1: Vec<_> = reduced.row(1).collect();
        assert_eq!(row0, vec![(0, 1.0), (1, 2.0)]);
        assert_eq!(row1, vec![(0, 3.0), (1, 1.0)]);

        assert_eq!(reduced.orig_col, vec![0, 1]);
        assert_eq!(reduced.orig_row, vec![0, 1]);
    }

    #[test]
    fn postsolve_rejects_wrong_length() {
        let mut problem = Problem::new();
        let x = problem.add_col(0.0, 10.0, false, 1.0, "x");
        problem.add_row("r0", &[(x, 1.0)], 1.0, f64::INFINITY);

        let mut presolver = Presolver::new();
        presolver.set_param("presolve.dualreds", 0).unwrap();
        presolver.presolve(problem);

        let expected = presolver.reduced_problem().num_cols;
        let err = presolver.postsolve(&vec![0.0; expected + 1]).unwrap_err();
        assert_eq!(
            err,
            PostsolveError::WrongLength {
                expected,
                got: expected + 1
            }
        );
    }

    #[test]
    fn problem_dimension_getters() {
        let mut problem = Problem::new();
        problem.add_col(0.0, 1.0, false, 1.0, "a");
        problem.add_col(0.0, 1.0, false, 1.0, "b");
        problem.add_row("r", &[(0, 1.0), (1, 1.0)], 0.0, 1.0);
        assert_eq!(problem.num_cols(), 2);
        assert_eq!(problem.num_rows(), 1);
    }

    #[test]
    fn transform_solution_subset_case() {
        // No aggregation: the forward map is a plain gather via orig_col.
        let mut problem = Problem::new();
        let x = problem.add_col(0.0, 10.0, false, 1.0, "x");
        let y = problem.add_col(0.0, 10.0, false, 1.0, "y");
        problem.add_row("r0", &[(x, 1.0), (y, 2.0)], 4.0, f64::INFINITY);
        problem.add_row("r1", &[(x, 3.0), (y, 1.0)], 5.0, f64::INFINITY);

        let mut presolver = Presolver::new();
        presolver.set_param("presolve.dualreds", 0).unwrap();
        presolver.presolve(problem);

        let reduced = presolver.reduced_problem();
        let original = vec![2.0, 3.0];
        let z = presolver.transform_solution(&original).unwrap();
        assert_eq!(z.len(), reduced.num_cols);
        // Each reduced column maps to one original column.
        for (k, &oc) in reduced.orig_col.iter().enumerate() {
            assert_eq!(z[k], original[oc]);
        }
    }

    #[test]
    fn transform_solution_handles_parallel_column_merge() {
        // x and y are parallel (col_y = 2*col_x, obj_y = 2*obj_x); w keeps the
        // rows non-parallel and keeps both columns at >= 2 nonzeros so the
        // parallel-column merge actually fires.
        let mut problem = Problem::new();
        let x = problem.add_col(0.0, 10.0, false, 1.0, "x");
        let y = problem.add_col(0.0, 10.0, false, 2.0, "y");
        let w = problem.add_col(0.0, 10.0, false, 1.0, "w");
        problem.add_row("r0", &[(x, 1.0), (y, 2.0), (w, 1.0)], 5.0, f64::INFINITY);
        problem.add_row("r1", &[(x, 1.0), (y, 2.0), (w, 5.0)], f64::NEG_INFINITY, 40.0);

        let mut presolver = Presolver::new();
        presolver.presolve(problem);

        let reduced = presolver.reduced_problem();
        // x and y are merged into a single reduced column (plus w).
        assert_eq!(reduced.num_cols, 2, "parallel columns should be merged");

        // A feasible original point: x + 2y = 4, w = 1.
        let x_star = vec![2.0, 1.0, 1.0];
        let z_star = presolver.transform_solution(&x_star).unwrap();

        // postsolve picks a (possibly different) split of the merged variable...
        let x2 = presolver.postsolve(&z_star).unwrap();
        assert_ne!(
            x2, x_star,
            "postsolve is expected to choose a different split of the merged variable"
        );

        // ...yet the forward map recombines it back to exactly z_star, which a
        // naive per-column gather could not do.
        let z2 = presolver.transform_solution(&x2).unwrap();
        for (a, b) in z2.iter().zip(z_star.iter()) {
            assert!((a - b).abs() < 1e-9, "round-trip mismatch: {a} vs {b}");
        }
    }

    #[test]
    fn transform_solution_rejects_wrong_length() {
        let mut problem = Problem::new();
        let x = problem.add_col(0.0, 10.0, false, 1.0, "x");
        problem.add_row("r0", &[(x, 1.0)], 1.0, f64::INFINITY);

        let mut presolver = Presolver::new();
        presolver.presolve(problem);

        let err = presolver.transform_solution(&[1.0, 2.0]).unwrap_err();
        assert_eq!(err, WrongLength { expected: 1, got: 2 });
    }
}
