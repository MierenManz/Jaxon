pub mod validator;
use validator::validators::StringValidator;
use validator::options::StringValidatorOptions;

#[no_mangle]
pub fn create() -> *const StringValidator {
    let opt = StringValidatorOptions::new(None, Some("ree".to_string()), None);
    let s: StringValidator = StringValidator::new::<String>(Some(opt));

    std::mem::forget(&s);

    &s
}

#[no_mangle]
pub fn len() -> usize {
    std::mem::size_of::<StringValidator>()
}
// pub fn string(
//     length_min: usize,
//     length_max: usize,
//     has_length: bool,
//     ends_with: *const u8,
//     ends_with_length: usize,
//     starts_with: *const u8,
//     starts_with_length: usize,
// ) -> *const StringValidator {
//     let validator = StringValidator::new();
// }
