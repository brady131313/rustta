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

macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::min;
            min($x, min!( $($xs),+ ))
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

    use super::indicators::momentum_indicators::*;
    use super::indicators::overlap_studies::*;
    use super::indicators::pattern_recognition::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let sma = SmaBuilder::default().time_period(4).build()?;
        let output = sma.calculate(data.as_slice())?;
        dbg!(sma);
        dbg!(output);

        panic!();
        Ok(())
    }
}
