use std::{convert::TryFrom, ffi::CStr};

use crate::{
    ffi::*,
    types::{TaError, TaResult},
};

use super::{
    func_handle::FuncHandle,
    params::{input::Input, opt_input::OptInput, output::Output},
};

#[derive(Debug)]
pub struct FuncInfo {
    handle: FuncHandle,
    name: String,
    camel_case_name: String,
    hint: String,
    group: String,
    params: Vec<OptInput>,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

impl FuncInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn camel_case_name(&self) -> &str {
        &self.camel_case_name
    }

    pub fn hint(&self) -> &str {
        &self.hint
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn params(&self) -> &[OptInput] {
        &self.params
    }

    pub fn inputs(&self) -> &[Input] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    fn func_params<'a, T: TryFrom<(&'a FuncHandle, usize), Error = TaError>>(
        handle: &'a FuncHandle,
        count: usize,
    ) -> TaResult<Vec<T>> {
        let mut params = Vec::with_capacity(count);

        for i in 0..count {
            let param = T::try_from((handle, i))?;
            params.push(param)
        }

        Ok(params)
    }
}

impl TryFrom<FuncHandle> for FuncInfo {
    type Error = TaError;

    fn try_from(handle: FuncHandle) -> Result<Self, Self::Error> {
        let mut info_ptr = std::ptr::null();
        let ret_code = unsafe { TA_GetFuncInfo(*handle, &mut info_ptr) };

        if ret_code != TA_RetCode::TA_SUCCESS {
            return Err(ret_code.into());
        }

        let name = unsafe {
            CStr::from_ptr((*info_ptr).name)
                .to_string_lossy()
                .into_owned()
        };

        let camel_case_name = unsafe {
            CStr::from_ptr((*info_ptr).camelCaseName)
                .to_string_lossy()
                .into_owned()
        };

        let hint = unsafe {
            CStr::from_ptr((*info_ptr).hint)
                .to_string_lossy()
                .into_owned()
        };

        let group = unsafe {
            CStr::from_ptr((*info_ptr).group)
                .to_string_lossy()
                .into_owned()
        };

        let param_count = unsafe { (*info_ptr).nbOptInput as usize };
        let params = Self::func_params(&handle, param_count)?;

        let input_count = unsafe { (*info_ptr).nbInput as usize };
        let inputs = Self::func_params(&handle, input_count)?;

        let output_count = unsafe { (*info_ptr).nbOutput as usize };
        let outputs = Self::func_params(&handle, output_count)?;

        Ok(Self {
            handle,
            name,
            camel_case_name,
            hint,
            group,
            params,
            inputs,
            outputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::meta::params::input::{InputFlags, InputType};
    use crate::meta::params::opt_input::{OptInputFlags, OptInputType};
    use crate::meta::params::output::{OutputFlags, OutputType};

    #[test]
    fn test_func_info() -> Result<(), Box<dyn Error>> {
        let handle = FuncHandle::try_from("BBANDS")?;
        let info = FuncInfo::try_from(handle)?;

        assert_eq!(info.name(), "BBANDS");
        assert_eq!(info.camel_case_name(), "Bbands");
        assert_eq!(info.hint(), "Bollinger Bands");
        assert_eq!(info.group(), "Overlap Studies");

        let inputs = info.inputs();
        assert_eq!(inputs.len(), 1);
        assert_eq!(
            inputs[0],
            Input::new(
                String::from("inReal"),
                0,
                InputType::Real,
                InputFlags::empty()
            )
        );

        let params = info.params();
        assert_eq!(params.len(), 4);
        assert_eq!(
            params[0],
            OptInput::new(
                String::from("optInTimePeriod"),
                0,
                OptInputType::Integer,
                String::from("Time Period"),
                5.0,
                String::from("Number of period"),
                OptInputFlags::empty()
            )
        );

        let outputs = info.outputs();
        assert_eq!(outputs.len(), 3);
        assert_eq!(
            outputs[0],
            Output::new(
                String::from("outRealUpperBand"),
                0,
                OutputType::Real,
                OutputFlags::UPPER_LIMIT
            )
        );

        Ok(())
    }
}
