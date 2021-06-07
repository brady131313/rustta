use std::{collections::HashMap, convert::TryFrom};

use crate::types::TaError;

use self::{
    func_handle::FuncHandle, func_info::FuncInfo, func_table::FuncTable, group_table::GroupTable,
};

pub mod flags;
pub mod func_handle;
pub mod func_info;
pub mod func_table;
pub mod group_table;
pub mod params;

#[derive(Debug)]
pub struct Meta {
    data: HashMap<String, HashMap<String, FuncInfo>>,
}

impl Meta {
    pub fn new() -> Result<Self, TaError> {
        let mut data = HashMap::new();
        let group_table = GroupTable::new()?;

        for group in group_table.iter() {
            let mut func_data = HashMap::new();
            let func_table = FuncTable::new(group)?;

            for func in func_table.iter() {
                let handle = FuncHandle::new(func)?;
                let info = FuncInfo::try_from(handle)?;
                func_data.insert(func.to_string_lossy().into_owned(), info);
            }

            data.insert(group.to_string_lossy().into_owned(), func_data);
        }

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_meta() -> Result<(), Box<dyn Error>> {
        let meta = Meta::new()?;

        println!("{:#?}", meta);
        panic!();
    }
}
