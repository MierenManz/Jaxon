use super::util::MinMax;
use super::util::Number;

pub struct StringValidatorOptions {
    pub length: Option<MinMax<usize>>,
    pub end_with: Option<String>,
    pub starts_with: Option<String>,
}

impl StringValidatorOptions {
    pub fn new(length: Option<MinMax<usize>>, end_with: Option<String>, starts_with: Option<String>) -> Self {
        StringValidatorOptions { length, end_with, starts_with }
    }
}

pub struct NumberValidatorOptions<T: Into<Number>> {
    pub range: Option<MinMax<T>>,
}
