use papilo_sys::Papilo_ParamResult;
use crate::ffi;
use crate::solver::Solver;

pub trait Parameter {
    fn set(solver: &mut Solver, key: &str, value: Self) ->  Result<(), String>;
}

impl Parameter for bool {
    fn set(solver: &mut Solver, key: &str, value: bool) -> Result<(), String> {
        let c_key = std::ffi::CString::new(key).map_err(|e| e.to_string())?;
        let res = unsafe {
            ffi::papilo_solver_set_param_bool(solver.raw(),
                                              c_key.as_ptr(),
                                              value as u32)
        };

        if res != ffi::Papilo_ParamResult_PAPILO_PARAM_CHANGED {
            Err(format!("{:?}", res))
        } else {
            Ok(())
        }
    }
}

impl Parameter for i32 {
    fn set(solver: &mut Solver, key: &str, value: i32) -> Result<(), String> {
        let c_key = std::ffi::CString::new(key).map_err(|e| e.to_string())?;
        let res = unsafe {
            ffi::papilo_solver_set_param_int(solver.raw(),
                                              c_key.as_ptr(),
                                              value)
        };

        if res != ffi::Papilo_ParamResult_PAPILO_PARAM_CHANGED {
            Err(format!("{:?}", res))
        } else {
            Ok(())
        }
    }
}

impl Parameter for f64 {
    fn set(solver: &mut Solver, key: &str, value: f64) -> Result<(), String> {
        let c_key = std::ffi::CString::new(key).map_err(|e| e.to_string())?;
        let res = unsafe {
            ffi::papilo_solver_set_param_real(solver.raw(),
                                              c_key.as_ptr(),
                                              value)
        };
        if res != ffi::Papilo_ParamResult_PAPILO_PARAM_CHANGED {
            Err(format!("{:?}", res))
        } else {
            Ok(())
        }
    }
}


impl Parameter for &str {
    fn set(solver: &mut Solver, key: &str, value: &str) -> Result<(), String> {
        let c_key = std::ffi::CString::new(key).map_err(|e| e.to_string())?;
        let c_value = std::ffi::CString::new(value).map_err(|e| e.to_string())?;
        let res = unsafe {
            ffi::papilo_solver_set_param_string(solver.raw(),
                                                c_key.as_ptr(),
                                                c_value.as_ptr())
        };

        if res != ffi::Papilo_ParamResult_PAPILO_PARAM_CHANGED {
            Err(format!("{:?}", res))
        } else {
            Ok(())
        }
    }
}


/// Represents the result of a parameter operation in Papilo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamResult {
    /// Parameter does not exist
    NotFound,
    /// Parameter is of a different type
    WrongType,
    /// Parameter was set to an invalid value
    InvalidValue,
}

impl From<Papilo_ParamResult> for ParamResult {
    fn from(result: Papilo_ParamResult) -> Self {
        match result {
            ffi::Papilo_ParamResult_PAPILO_PARAM_NOT_FOUND => ParamResult::NotFound,
            ffi::Papilo_ParamResult_PAPILO_PARAM_WRONG_TYPE => ParamResult::WrongType,
            ffi::Papilo_ParamResult_PAPILO_PARAM_INVALID_VALUE => ParamResult::InvalidValue,
            _ => panic!("Unknown parameter result"),
        }
    }
}