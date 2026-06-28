use crate::ffi;

/// The outcome of a [`Presolver::presolve`](crate::presolver::Presolver::presolve) call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresolveStatus {
    /// The problem was not changed by presolving.
    Unchanged,
    /// The problem was reduced by presolving.
    Reduced,
    /// The problem was detected to be unbounded or infeasible.
    UnboundedOrInfeasible,
    /// The problem was detected to be unbounded.
    Unbounded,
    /// The problem was detected to be infeasible.
    Infeasible,
}

impl From<ffi::PAPILO_PRESOLVE_STATUS> for PresolveStatus {
    fn from(status: ffi::PAPILO_PRESOLVE_STATUS) -> Self {
        match status {
            ffi::Papilo_PresolveStatus_PAPILO_PRESOLVE_STATUS_UNCHANGED => {
                PresolveStatus::Unchanged
            }
            ffi::Papilo_PresolveStatus_PAPILO_PRESOLVE_STATUS_REDUCED => PresolveStatus::Reduced,
            ffi::Papilo_PresolveStatus_PAPILO_PRESOLVE_STATUS_UNBND_OR_INFEAS => {
                PresolveStatus::UnboundedOrInfeasible
            }
            ffi::Papilo_PresolveStatus_PAPILO_PRESOLVE_STATUS_UNBOUNDED => {
                PresolveStatus::Unbounded
            }
            ffi::Papilo_PresolveStatus_PAPILO_PRESOLVE_STATUS_INFEASIBLE => {
                PresolveStatus::Infeasible
            }
            _ => panic!("Unknown presolve status: {}", status),
        }
    }
}

/// A snapshot of the problem produced by PaPILO's presolving step.
///
/// All vectors are indexed by *reduced* problem indices. Use [`ReducedProblem::orig_col`]
/// and [`ReducedProblem::orig_row`] to translate a reduced index back to the
/// corresponding index in the original problem.
///
/// Infinite bounds and row sides are represented as `f64::INFINITY` /
/// `f64::NEG_INFINITY`.
#[derive(Debug, Clone, PartialEq)]
pub struct ReducedProblem {
    /// Number of columns (variables) in the reduced problem.
    pub num_cols: usize,
    /// Number of rows (constraints) in the reduced problem.
    pub num_rows: usize,
    /// Lower bound of each column (`f64::NEG_INFINITY` if unbounded below).
    pub col_lower: Vec<f64>,
    /// Upper bound of each column (`f64::INFINITY` if unbounded above).
    pub col_upper: Vec<f64>,
    /// Whether each column is integer-constrained.
    pub col_integral: Vec<bool>,
    /// Objective coefficient of each column.
    pub objective: Vec<f64>,
    /// Constant objective offset.
    pub objective_offset: f64,
    /// Left-hand side of each row (`f64::NEG_INFINITY` if none).
    pub row_lhs: Vec<f64>,
    /// Right-hand side of each row (`f64::INFINITY` if none).
    pub row_rhs: Vec<f64>,
    /// CSR row pointers; `row_start[r]..row_start[r + 1]` indexes into
    /// [`col_indices`](Self::col_indices) / [`values`](Self::values) for row `r`.
    /// Has length `num_rows + 1`.
    pub row_start: Vec<usize>,
    /// Column index of each nonzero, in CSR order.
    pub col_indices: Vec<usize>,
    /// Value of each nonzero, in CSR order.
    pub values: Vec<f64>,
    /// For each reduced column, the index of the corresponding original column.
    pub orig_col: Vec<usize>,
    /// For each reduced row, the index of the corresponding original row.
    pub orig_row: Vec<usize>,
}

impl ReducedProblem {
    /// Number of nonzero entries in the constraint matrix.
    pub fn num_nonzeros(&self) -> usize {
        self.values.len()
    }

    /// Returns the nonzero entries `(column, value)` of the given reduced row.
    pub fn row(&self, row: usize) -> impl Iterator<Item = (usize, f64)> + '_ {
        let start = self.row_start[row];
        let end = self.row_start[row + 1];
        (start..end).map(move |k| (self.col_indices[k], self.values[k]))
    }
}

/// Error returned when [`Presolver::postsolve`](crate::presolver::Presolver::postsolve) fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostsolveError {
    /// The provided reduced solution did not have the expected length.
    WrongLength {
        /// The number of values that were expected (the reduced column count).
        expected: usize,
        /// The number of values that were provided.
        got: usize,
    },
    /// PaPILO's postsolve routine failed to reconstruct the original solution.
    Failed,
}

impl std::fmt::Display for PostsolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostsolveError::WrongLength { expected, got } => write!(
                f,
                "reduced solution has wrong length: expected {expected}, got {got}"
            ),
            PostsolveError::Failed => write!(f, "postsolve failed"),
        }
    }
}

impl std::error::Error for PostsolveError {}
