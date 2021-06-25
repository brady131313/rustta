#[macro_use]
extern crate derive_builder;

#[allow(dead_code)]
pub mod indicators {
    include!(concat!(env!("OUT_DIR"), "/indicators.rs"));
}

#[cfg(test)]
mod tests {
    use super::indicators::overlap_studies::*;
    use super::*;

    #[test]
    fn it_works() {
        let sma = SmaBuilder::default().time_period(4).build();
        println!("{:#?}", sma);

        panic!()
    }
}
