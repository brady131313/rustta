use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
    ops::Deref,
};

use crate::{
    ffi::*,
    types::{TaError, TaResult},
};

#[derive(Debug)]
pub struct FuncHandle(*const TA_FuncHandle);

impl FuncHandle {
    pub fn new(func: &CStr) -> TaResult<Self> {
        let mut handle = std::ptr::null();
        let ret_code = unsafe { TA_GetFuncHandle(func.as_ptr(), &mut handle) };

        match ret_code {
            TA_RetCode::TA_SUCCESS => Ok(Self(handle)),
            TA_RetCode::TA_FUNC_NOT_FOUND => {
                Err(TaError::FuncNotFound(func.to_string_lossy().into_owned()))
            }
            _ => Err(ret_code.into()),
        }
    }
}

impl TryFrom<&str> for FuncHandle {
    type Error = TaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CString::new(value)
            .map_err(|_| TaError::Misc("Can't convert {} to cstring".into()))
            .and_then(|func| Self::new(&func))
    }
}

impl Deref for FuncHandle {
    type Target = *const TA_FuncHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, error::Error, ffi::CString};

    use crate::types::TaError;

    use super::*;

    #[test]
    fn test_handle() -> Result<(), Box<dyn Error>> {
        let func_name = CString::new("MEDPRICE")?;
        assert!(FuncHandle::new(&func_name).is_ok());

        let invalid_name = CString::new("Invalid")?;
        assert_eq!(
            FuncHandle::new(&invalid_name).unwrap_err(),
            TaError::FuncNotFound("Invalid".into())
        );

        Ok(())
    }

    #[test]
    fn test_from_str() {
        assert!(FuncHandle::try_from("MEDPRICE").is_ok());

        assert_eq!(
            FuncHandle::try_from("Invalid").unwrap_err(),
            TaError::FuncNotFound("Invalid".into())
        )
    }
}
