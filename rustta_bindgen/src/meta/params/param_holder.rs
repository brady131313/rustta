use std::{cmp::max, convert::TryFrom};

use crate::{
    ffi::*,
    meta::func_handle::FuncHandle,
    types::{TaError, TaResult},
};

macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::max;
            max($x, max!( $($xs),+ ))
        }
    };
}

pub trait Length {
    fn length(&self) -> usize;
}

pub trait Open {
    fn open(&self) -> &[f64];
}

pub trait Low {
    fn low(&self) -> &[f64];
}
pub trait High {
    fn high(&self) -> &[f64];
}

pub trait Close {
    fn close(&self) -> &[f64];
}
pub trait Volume {
    fn volume(&self) -> &[f64];
}

pub trait OpenInterest {
    fn open_interest(&self) -> &[f64];
}

impl<T> Length for &[T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn length(&self) -> usize {
        max!(
            self.0.len(),
            self.1.len(),
            self.2.len(),
            self.3.len(),
            self.4.len(),
            self.5.len()
        )
    }
}

impl Open for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn open(&self) -> &[f64] {
        self.0
    }
}

impl Low for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn low(&self) -> &[f64] {
        self.1
    }
}

impl High for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn high(&self) -> &[f64] {
        self.2
    }
}

impl Close for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn close(&self) -> &[f64] {
        self.3
    }
}

impl Volume for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn volume(&self) -> &[f64] {
        self.4
    }
}

impl OpenInterest for (&[f64], &[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn open_interest(&self) -> &[f64] {
        self.5
    }
}

pub struct Ohlc<'a> {
    pub open: &'a [f64],
    pub low: &'a [f64],
    pub high: &'a [f64],
    pub close: &'a [f64],
    pub volume: &'a [f64],
    pub openinterest: &'a [f64],
}

pub enum InputParam<'a> {
    Real(&'a [f64]),
    Integer(&'a [i32]),
    Ohlc(Ohlc<'a>),
}

pub enum OptInputParam {
    Real(f64),
    Integer(i32),
}

pub enum OutputParam<'a> {
    Real(&'a mut [f64]),
    Integer(&'a mut [i32]),
}

pub struct ParamHolder(*mut TA_ParamHolder);

impl ParamHolder {
    pub fn set_input(&mut self, position: usize, param: InputParam) -> TaResult<()> {
        let position = position as u32;

        let ret_code = match param {
            InputParam::Real(xs) => unsafe {
                TA_SetInputParamRealPtr(self.0, position, xs.as_ptr())
            },
            InputParam::Integer(xs) => unsafe {
                TA_SetInputParamIntegerPtr(self.0, position, xs.as_ptr())
            },
            InputParam::Ohlc(ohlc) => unsafe {
                TA_SetInputParamPricePtr(
                    self.0,
                    position,
                    ohlc.open.as_ptr(),
                    ohlc.high.as_ptr(),
                    ohlc.low.as_ptr(),
                    ohlc.close.as_ptr(),
                    ohlc.volume.as_ptr(),
                    ohlc.openinterest.as_ptr(),
                )
            },
        };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(TaError::BadParam);
        }

        Ok(())
    }

    pub fn set_param(&mut self, position: usize, param: OptInputParam) -> TaResult<()> {
        let position = position as u32;

        let ret_code = match param {
            OptInputParam::Real(x) => unsafe {
                TA_SetOptInputParamReal(self.0, position, x as f64)
            },
            OptInputParam::Integer(x) => unsafe { TA_SetOptInputParamInteger(self.0, position, x) },
        };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(TaError::BadParam);
        }

        Ok(())
    }

    pub fn set_output(&mut self, position: usize, param: OutputParam) -> TaResult<()> {
        let position = position as u32;

        let ret_code = match param {
            OutputParam::Real(xs) => unsafe {
                TA_SetOutputParamRealPtr(self.0, position, xs.as_mut_ptr())
            },
            OutputParam::Integer(xs) => unsafe {
                TA_SetOutputParamIntegerPtr(self.0, position, xs.as_mut_ptr())
            },
        };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(TaError::BadParam);
        }

        Ok(())
    }

    pub fn lookback(&self) -> Option<usize> {
        let lookback_ptr: *mut i32 = &mut 0;
        let ret_code = unsafe { TA_GetLookback(self.0, lookback_ptr) };

        match ret_code {
            TA_RetCode::TA_SUCCESS => {
                let lookback = unsafe { *lookback_ptr };
                Some(lookback as usize)
            }
            _ => None,
        }
    }

    pub fn required_output_size(&self, start: i32, end: i32) -> Option<usize> {
        let start = start as usize;
        let end = end as usize;

        let lookback = self.lookback()?;
        let temp = max(lookback, start);

        if temp > end {
            Some(0)
        } else {
            Some(end - temp + 1)
        }
    }
}

impl TryFrom<FuncHandle> for ParamHolder {
    type Error = TaError;

    fn try_from(handle: FuncHandle) -> Result<Self, Self::Error> {
        let mut holder = std::ptr::null_mut();
        let ret_code = unsafe { TA_ParamHolderAlloc(*handle, &mut holder) };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(ret_code.into());
        }

        Ok(Self(holder))
    }
}

impl Drop for ParamHolder {
    fn drop(&mut self) {
        unsafe { TA_ParamHolderFree(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_from_handle() -> Result<(), Box<dyn Error>> {
        let handle = FuncHandle::try_from("BBANDS")?;
        let param_holder = ParamHolder::try_from(handle);

        assert!(param_holder.is_ok());
        Ok(())
    }

    #[test]
    fn test_can_set_input_param() -> Result<(), Box<dyn Error>> {
        let handle = FuncHandle::try_from("BBANDS")?;
        let mut param_holder = ParamHolder::try_from(handle)?;

        let result = param_holder.set_input(0, InputParam::Real(&[1.0, 2.0, 3.0]));
        assert!(result.is_ok());

        // Cant set invalid param
        let result = param_holder.set_input(1, InputParam::Real(&[1.0, 2.0, 3.0]));
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_can_set_opt_input_param() -> Result<(), Box<dyn Error>> {
        let handle = FuncHandle::try_from("BBANDS")?;
        let mut param_holder = ParamHolder::try_from(handle)?;

        let result = param_holder.set_param(0, OptInputParam::Integer(1));
        assert!(result.is_ok());

        // Cant set invalid param
        let result = param_holder.set_param(10, OptInputParam::Integer(1));
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_can_set_output_param() -> Result<(), Box<dyn Error>> {
        let handle = FuncHandle::try_from("TYPPRICE")?;
        let mut param_holder = ParamHolder::try_from(handle)?;

        let mut data = vec![1.0, 2.0, 3.0];

        let result = param_holder.set_output(0, OutputParam::Real(&mut data));
        assert!(result.is_ok());

        // Cant set invalid param
        let result = param_holder.set_output(1, OutputParam::Real(&mut data));
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_can_get_lookback() -> Result<(), Box<dyn Error>> {
        let handle = FuncHandle::try_from("SMA")?;
        let mut param_holder = ParamHolder::try_from(handle)?;

        // Default period for SMA is 30, so look back is 30 - 1
        assert_eq!(param_holder.lookback().unwrap(), 29);

        param_holder.set_param(0, OptInputParam::Integer(5))?;
        assert_eq!(param_holder.lookback().unwrap(), 4);

        Ok(())
    }
}
