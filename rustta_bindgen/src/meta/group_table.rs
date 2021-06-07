use std::ffi::CStr;

use crate::ffi::*;
use crate::types::TaResult;

pub struct GroupTable(*mut TA_StringTable);

impl GroupTable {
    pub fn new() -> TaResult<Self> {
        let mut table = std::ptr::null_mut();
        let ret_code = unsafe { TA_GroupTableAlloc(&mut table) };

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

impl Drop for GroupTable {
    fn drop(&mut self) {
        unsafe { TA_GroupTableFree(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, ffi::CString};

    use super::*;

    #[test]
    fn test_group_iter() -> Result<(), Box<dyn Error>> {
        let group_table = GroupTable::new()?;
        let mut iter = group_table.iter();

        let expected: &CStr = &CString::new("Math Operators")?;
        assert_eq!(expected, iter.next().unwrap());

        let expected: &CStr = &CString::new("Math Transform")?;
        assert_eq!(expected, iter.next().unwrap());

        Ok(())
    }
}
