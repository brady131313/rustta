use std::{convert::TryFrom, ffi::CStr};

use crate::{
    ffi::{TA_GetOutputParameterInfo, TA_OutputParameterType, TA_RetCode},
    meta::func_handle::FuncHandle,
    types::TaError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    Real,
    Integer,
}

impl From<TA_OutputParameterType> for OutputType {
    fn from(param_type: TA_OutputParameterType) -> Self {
        match param_type {
            TA_OutputParameterType::TA_Output_Integer => Self::Integer,
            TA_OutputParameterType::TA_Output_Real => Self::Real,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    name: String,
    param_type: OutputType,
    flags: OutputFlags,
}

impl Output {
    pub fn new(name: String, param_type: OutputType, flags: OutputFlags) -> Self {
        Self {
            name,
            param_type,
            flags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn param_type(&self) -> OutputType {
        self.param_type
    }

    pub fn flags(&self) -> OutputFlags {
        self.flags
    }
}

impl TryFrom<(&FuncHandle, usize)> for Output {
    type Error = TaError;

    fn try_from(handle: (&FuncHandle, usize)) -> Result<Self, Self::Error> {
        let mut param_ptr = std::ptr::null();
        let ret_code =
            unsafe { TA_GetOutputParameterInfo(**(handle.0), handle.1 as u32, &mut param_ptr) };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(ret_code.into());
        }

        let name = unsafe {
            CStr::from_ptr((*param_ptr).paramName)
                .to_string_lossy()
                .into_owned()
        };

        let param_type = unsafe { OutputType::from((*param_ptr).type_) };
        let flags = unsafe { OutputFlags::from_bits((*param_ptr).flags as u32).unwrap() };

        Ok(Self {
            name,
            param_type,
            flags,
        })
    }
}

bitflags! {
    #[derive(Default)]
    pub struct OutputFlags: u32 {
        const LINE = 0x00000001;
        const DOT_LINE = 0x00000002;
        const DASH_LINE = 0x00000004;
        const DOT = 0x00000008;
        const HISTOGRAM = 0x00000010;
        const PATTERN_BOOL = 0x00000020;
        const PATTERN_BULL_BEAR = 0x00000040;
        const PATTERN_STRENGTH = 0x00000080;
        const POSITIVE = 0x00000100;
        const NEGATIVE = 0x00000200;
        const ZERO = 0x00000400;
        const UPPER_LIMIT = 0x00000800;
        const LOWER_LIMIT = 0x00001000;
    }
}
