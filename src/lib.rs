#[macro_use]
extern crate derive_builder;

macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::max;
            max($x, max!( $($xs),+ ))
        }
    };
}

#[allow(dead_code)]
pub mod indicators {
    include!(concat!(env!("OUT_DIR"), "/indicators.rs"));
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::indicators::overlap_studies::*;
    use super::indicators::pattern_recognition::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ohlc_data = (
            data.as_slice(),
            data.as_slice(),
            data.as_slice(),
            data.as_slice(),
            data.as_slice(),
            data.as_slice(),
        );

        let sma = SmaBuilder::default().time_period(4).build()?;
        let output_size = sma.calculate(data.as_slice())?;
        dbg!(sma);
        dbg!(output_size);

        let cdl2_crows = Cdl2CrowsBuilder::default().build()?;
        let output_size = cdl2_crows.calculate(ohlc_data)?;
        dbg!(cdl2_crows);
        dbg!(output_size);

        panic!();
        Ok(())
    }
}
