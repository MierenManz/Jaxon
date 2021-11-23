#[derive(PartialEq, PartialOrd)]
pub enum Number {
    F64(f64),
    U64(u64),
    I64(i64),
    Usize(usize),
}

impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Number::F64(val)
    }
}

impl From<u64> for Number {
    fn from(val: u64) -> Self {
        Number::U64(val)
    }
}

impl From<i64> for Number {
    fn from(val: i64) -> Self {
        Number::I64(val)
    }
}

impl From<usize> for Number {
    fn from(val: usize) -> Self {
        Number::Usize(val)
    }
}

pub struct MinMax<T: Into<Number>> {
    pub min: T,
    pub max: T,
}

impl MinMax<Number> {
    pub fn new<T: Into<Number>>(min: T, max: T) -> Self {
        MinMax {
            min: min.into(),
            max: max.into(),
        }
    }
}
