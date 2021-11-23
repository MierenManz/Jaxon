use super::options::NumberValidatorOptions;
use super::options::StringValidatorOptions;
use super::util::Number;

trait Validator<E> {
    fn is_valid<T: Into<E>>(self, val: T) -> bool;
}

#[repr(C)]
pub struct StringValidator {
    options: Option<StringValidatorOptions>,
}

impl StringValidator {
    pub fn new<T: Into<String>>(opt: Option<StringValidatorOptions>) -> Self {
        StringValidator { options: opt }
    }
}

impl Validator<String> for StringValidator {
    fn is_valid<T: Into<String>>(self, val: T) -> bool {
        let str_value = val.into();

        let opts = match self.options {
            Some(opt) => opt,
            None => return true,
        };

        if let Some(end) = opts.end_with {
            if !str_value.ends_with(&end) {
                return false;
            }
        }

        if let Some(start) = opts.starts_with {
            if !str_value.starts_with(&start) {
                return false;
            }
        }

        if let Some(range) = opts.length {
            if str_value.len() < range.min || str_value.len() > range.max {
                return false;
            }
        }

        true
    }
}

#[repr(C)]
pub struct NumberValidator<T: Into<Number>> {
    options: Option<NumberValidatorOptions<T>>,
}

macro_rules! number {
    ($number:ty, $e:ident) => {
        impl NumberValidator<$number> {
            pub fn new(opt: Option<NumberValidatorOptions<$number>>) -> Self {
                NumberValidator { options: opt }
            }
        }

        impl Validator<Number> for NumberValidator<$number> {
            fn is_valid<T: Into<Number>>(self, val: T) -> bool {
                let real_val = val.into();
                let opts = match self.options {
                    Some(opts) => opts,
                    None => return true,
                };

                if let Some(range) = opts.range {
                    if real_val < Number::$e(range.min) || real_val > Number::$e(range.max) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

number!(u64, U64);
number!(f64, F64);
number!(i64, I64);
number!(usize, Usize);
