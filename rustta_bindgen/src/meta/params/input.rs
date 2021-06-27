use std::{convert::TryFrom, ffi::CStr};

use crate::{
    ffi::{TA_GetInputParameterInfo, TA_InputParameterType, TA_RetCode},
    meta::func_handle::FuncHandle,
    types::TaError,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputType {
    Real,
    Integer,
    Price,
}

impl From<TA_InputParameterType> for InputType {
    fn from(param_type: TA_InputParameterType) -> Self {
        match param_type {
            TA_InputParameterType::TA_Input_Integer => Self::Integer,
            TA_InputParameterType::TA_Input_Real => Self::Real,
            TA_InputParameterType::TA_Input_Price => Self::Price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    name: String,
    position: usize,
    param_type: InputType,
    flags: InputFlags,
}

impl Input {
    pub fn new(name: String, position: usize, param_type: InputType, flags: InputFlags) -> Self {
        Self {
            name,
            position,
            param_type,
            flags,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn param_type(&self) -> InputType {
        self.param_type
    }

    pub fn flags(&self) -> InputFlags {
        self.flags
    }
}

impl TryFrom<(&FuncHandle, usize)> for Input {
    type Error = TaError;

    fn try_from(handle: (&FuncHandle, usize)) -> Result<Self, Self::Error> {
        let position = handle.1;
        let mut param_ptr = std::ptr::null();
        let ret_code =
            unsafe { TA_GetInputParameterInfo(**(handle.0), position as u32, &mut param_ptr) };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(ret_code.into());
        }

        let name = unsafe {
            CStr::from_ptr((*param_ptr).paramName)
                .to_string_lossy()
                .into_owned()
        };

        let param_type = unsafe { InputType::from((*param_ptr).type_) };
        let flags = unsafe { InputFlags::from_bits((*param_ptr).flags as u32).unwrap() };

        Ok(Self {
            name,
            position,
            param_type,
            flags,
        })
    }
}

bitflags! {
    #[derive(Default)]
    pub struct InputFlags: u32 {
        const OPEN = 0x00000001;
        const HIGH = 0x00000002;
        const LOW = 0x00000004;
        const CLOSE = 0x00000008;
        const VOLUME = 0x00000010;
        const OPEN_INTEREST = 0x00000020;
        const TIMESTAMP = 0x00000040;
    }
}
