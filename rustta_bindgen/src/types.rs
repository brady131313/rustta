use super::ffi::*;
use std::{error::Error, fmt};

pub type TaResult<T> = Result<T, TaError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaError {
    BadAllocation,
    BadParam,
    FuncNotFound(String),
    Misc(String),
}

impl fmt::Display for TaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaError::BadAllocation => write!(f, "Bad allocation"),
            TaError::BadParam => write!(f, "Bad param"),
            TaError::FuncNotFound(e) => write!(f, "Function with name {} not found", e),
            TaError::Misc(e) => write!(f, "Misc error: {}", e),
        }
    }
}

impl Error for TaError {}

impl From<TA_RetCode> for TaError {
    fn from(code: TA_RetCode) -> Self {
        match code {
            TA_RetCode::TA_ALLOC_ERR => Self::BadAllocation,
            TA_RetCode::TA_BAD_PARAM => Self::BadParam,
            code => Self::Misc(format!("Unknown return code: {:?}", code)),
        }
    }
}
