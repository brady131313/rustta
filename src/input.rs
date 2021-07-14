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

impl<T> Length for &[T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> Length for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for (&[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn length(&self) -> usize {
        max!(
            self.0.len(),
            self.1.len(),
            self.2.len(),
            self.3.len(),
            self.4.len()
        )
    }
}

impl Open for (&[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn open(&self) -> &[f64] {
        self.0
    }
}

impl Low for (&[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn low(&self) -> &[f64] {
        self.1
    }
}

impl High for (&[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn high(&self) -> &[f64] {
        self.2
    }
}

impl Close for (&[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn close(&self) -> &[f64] {
        self.3
    }
}

impl Volume for (&[f64], &[f64], &[f64], &[f64], &[f64]) {
    fn volume(&self) -> &[f64] {
        self.4
    }
}

pub struct Ohlcv<'a> {
    pub open: &'a [f64],
    pub high: &'a [f64],
    pub low: &'a [f64],
    pub close: &'a [f64],
    pub volume: &'a [f64],
}

impl<'a> Default for Ohlcv<'a> {
    fn default() -> Self {
        Self {
            open: &[],
            high: &[],
            low: &[],
            close: &[],
            volume: &[],
        }
    }
}

impl<'a> Length for Ohlcv<'a> {
    fn length(&self) -> usize {
        max!(
            self.open.len(),
            self.high.len(),
            self.low.len(),
            self.close.len(),
            self.volume.len()
        )
    }
}

impl<'a> Open for Ohlcv<'a> {
    fn open(&self) -> &[f64] {
        self.open
    }
}
impl<'a> High for Ohlcv<'a> {
    fn high(&self) -> &[f64] {
        self.high
    }
}
impl<'a> Low for Ohlcv<'a> {
    fn low(&self) -> &[f64] {
        self.low
    }
}
impl<'a> Close for Ohlcv<'a> {
    fn close(&self) -> &[f64] {
        self.close
    }
}
impl<'a> Volume for Ohlcv<'a> {
    fn volume(&self) -> &[f64] {
        self.volume
    }
}
