use std::{convert::TryFrom, ffi::CStr};

use crate::{
    ffi::{TA_GetOptInputParameterInfo, TA_OptInputParameterType, TA_RetCode},
    meta::func_handle::FuncHandle,
    types::TaError,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OptInputType {
    Real,
    Integer,
}

impl From<TA_OptInputParameterType> for OptInputType {
    fn from(param_type: TA_OptInputParameterType) -> Self {
        match param_type {
            TA_OptInputParameterType::TA_OptInput_IntegerList
            | TA_OptInputParameterType::TA_OptInput_IntegerRange => Self::Integer,
            TA_OptInputParameterType::TA_OptInput_RealList
            | TA_OptInputParameterType::TA_OptInput_RealRange => Self::Real,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptInput {
    name: String,
    param_type: OptInputType,
    display_name: String,
    default: f64,
    hint: String,
    flags: OptInputFlags,
}

impl OptInput {
    pub fn new(
        name: String,
        param_type: OptInputType,
        display_name: String,
        default: f64,
        hint: String,
        flags: OptInputFlags,
    ) -> Self {
        Self {
            name,
            param_type,
            display_name,
            default,
            hint,
            flags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn param_type(&self) -> OptInputType {
        self.param_type
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn default(&self) -> f64 {
        self.default
    }

    pub fn hint(&self) -> &str {
        &self.hint
    }

    pub fn flags(&self) -> OptInputFlags {
        self.flags
    }
}

impl TryFrom<(&FuncHandle, usize)> for OptInput {
    type Error = TaError;

    fn try_from(handle: (&FuncHandle, usize)) -> Result<Self, Self::Error> {
        let mut param_ptr = std::ptr::null();
        let ret_code =
            unsafe { TA_GetOptInputParameterInfo(**(handle.0), handle.1 as u32, &mut param_ptr) };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(ret_code.into());
        }

        let name = unsafe {
            CStr::from_ptr((*param_ptr).paramName)
                .to_string_lossy()
                .into_owned()
        };

        let display_name = unsafe {
            CStr::from_ptr((*param_ptr).displayName)
                .to_string_lossy()
                .into_owned()
        };

        let hint = unsafe {
            CStr::from_ptr((*param_ptr).hint)
                .to_string_lossy()
                .into_owned()
        };

        let param_type = unsafe { OptInputType::from((*param_ptr).type_) };
        let flags = unsafe { OptInputFlags::from_bits((*param_ptr).flags as u32).unwrap() };
        let default = unsafe { (*param_ptr).defaultValue };

        Ok(Self {
            name,
            param_type,
            display_name,
            default,
            hint,
            flags,
        })
    }
}

bitflags! {
    #[derive(Default)]
    pub struct OptInputFlags: u32 {
        const PERCENT = 0x00100000;
        const DEGREE = 0x00200000;
        const CURRENCY = 0x00400000;
        const ADVANCED = 0x01000000;
    }
}
