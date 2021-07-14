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
