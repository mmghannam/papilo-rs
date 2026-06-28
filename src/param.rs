use crate::ffi;
use crate::presolver::Presolver;
use papilo_sys::Papilo_ParamResult;

/// Trait implemented by types that can be used as presolver parameter values.
pub trait Parameter {
    /// Sets the parameter `key` to `value` on the given presolver.
    fn set(presolver: &mut Presolver, key: &str, value: Self) -> Result<(), ParamResult>;
}

/// Maps a raw `Papilo_ParamResult` code into a `Result`.
fn check(res: Papilo_ParamResult) -> Result<(), ParamResult> {
    if res == ffi::Papilo_ParamResult_PAPILO_PARAM_CHANGED {
        Ok(())
    } else {
        Err(res.into())
    }
}

impl Parameter for bool {
    fn set(presolver: &mut Presolver, key: &str, value: bool) -> Result<(), ParamResult> {
        let c_key = std::ffi::CString::new(key).expect("parameter key contains a nul byte");
        check(unsafe {
            ffi::papilo_presolver_set_param_bool(presolver.raw(), c_key.as_ptr(), value as u32)
        })
    }
}

impl Parameter for i32 {
    fn set(presolver: &mut Presolver, key: &str, value: i32) -> Result<(), ParamResult> {
        let c_key = std::ffi::CString::new(key).expect("parameter key contains a nul byte");
        check(unsafe {
            ffi::papilo_presolver_set_param_int(presolver.raw(), c_key.as_ptr(), value)
        })
    }
}

impl Parameter for f64 {
    fn set(presolver: &mut Presolver, key: &str, value: f64) -> Result<(), ParamResult> {
        let c_key = std::ffi::CString::new(key).expect("parameter key contains a nul byte");
        check(unsafe {
            ffi::papilo_presolver_set_param_real(presolver.raw(), c_key.as_ptr(), value)
        })
    }
}

impl Parameter for &str {
    fn set(presolver: &mut Presolver, key: &str, value: &str) -> Result<(), ParamResult> {
        let c_key = std::ffi::CString::new(key).expect("parameter key contains a nul byte");
        let c_value = std::ffi::CString::new(value).expect("parameter value contains a nul byte");
        check(unsafe {
            ffi::papilo_presolver_set_param_string(
                presolver.raw(),
                c_key.as_ptr(),
                c_value.as_ptr(),
            )
        })
    }
}

/// Represents a failure to set a presolver parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamResult {
    /// Parameter does not exist.
    NotFound,
    /// Parameter is of a different type.
    WrongType,
    /// Parameter was set to an invalid value.
    InvalidValue,
}

impl From<Papilo_ParamResult> for ParamResult {
    fn from(result: Papilo_ParamResult) -> Self {
        match result {
            ffi::Papilo_ParamResult_PAPILO_PARAM_NOT_FOUND => ParamResult::NotFound,
            ffi::Papilo_ParamResult_PAPILO_PARAM_WRONG_TYPE => ParamResult::WrongType,
            ffi::Papilo_ParamResult_PAPILO_PARAM_INVALID_VALUE => ParamResult::InvalidValue,
            _ => panic!("Unknown parameter result: {}", result),
        }
    }
}

impl std::fmt::Display for ParamResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamResult::NotFound => write!(f, "parameter not found"),
            ParamResult::WrongType => write!(f, "parameter has a different type"),
            ParamResult::InvalidValue => write!(f, "parameter was set to an invalid value"),
        }
    }
}

impl std::error::Error for ParamResult {}
