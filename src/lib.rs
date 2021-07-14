#[macro_use]
extern crate derive_builder;

#[cfg_attr(test, macro_use)]
extern crate approx;

pub mod indicators;
pub mod input;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::indicators::overlap_studies::*;

    mod real_input_indicator {
        use super::*;

        #[test]
        fn accepts_vec() -> Result<(), Box<dyn Error>> {
            let sma = SmaBuilder::default().time_period(4).build()?;
            let expected = vec![2.5, 3.5];

            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let output = sma.calculate(data)?;
            assert_relative_eq!(output.as_slice(), expected.as_slice());

            Ok(())
        }

        #[test]
        fn accepts_slice() -> Result<(), Box<dyn Error>> {
            let sma = SmaBuilder::default().time_period(4).build()?;
            let expected = vec![2.5, 3.5];

            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let output = sma.calculate(data.as_slice())?;
            assert_relative_eq!(output.as_slice(), expected.as_slice());

            Ok(())
        }
    }

    mod price_input_indicator {
        use super::*;

        #[test]
        fn accepts_data() -> Result<(), Box<dyn Error>> {
            let midprice = MidPriceBuilder::default().time_period(5).build()?;

            Ok(())
        }
    }
}
