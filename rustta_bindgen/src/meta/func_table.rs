use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
};

use crate::{
    ffi::*,
    types::{TaError, TaResult},
};

pub struct FuncTable(*mut TA_StringTable);

impl FuncTable {
    pub fn new(group: &CStr) -> TaResult<Self> {
        let mut table = std::ptr::null_mut();
        let ret_code = unsafe { TA_FuncTableAlloc(group.as_ptr(), &mut table) };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(ret_code.into());
        }

        Ok(Self(table))
    }

    pub fn iter(&self) -> impl Iterator<Item = &CStr> {
        unsafe {
            std::slice::from_raw_parts((*self.0).string, (*self.0).size as usize)
                .iter()
                .map(|ptr| CStr::from_ptr(*ptr))
        }
    }
}

impl Drop for FuncTable {
    fn drop(&mut self) {
        unsafe { TA_FuncTableFree(self.0) };
    }
}

impl TryFrom<&str> for FuncTable {
    type Error = TaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let group = CString::new(value).map_err(|_| TaError::Misc("null str".into()))?;
        Self::new(&group)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_from_str() -> Result<(), Box<dyn Error>> {
        let func_table = FuncTable::try_from("Math Operators");
        assert!(func_table.is_ok());

        let func_table = FuncTable::try_from("Some group");
        assert!(func_table.is_err());
        Ok(())
    }

    #[test]
    fn test_func_iter() -> Result<(), Box<dyn Error>> {
        let func_table = FuncTable::try_from("Math Operators")?;
        let mut iter = func_table.iter();

        let expected: &CStr = &CString::new("ADD")?;
        assert_eq!(expected, iter.next().unwrap());

        let expected: &CStr = &CString::new("DIV")?;
        assert_eq!(expected, iter.next().unwrap());

        Ok(())
    }
}
