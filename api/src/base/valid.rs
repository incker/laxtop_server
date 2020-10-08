use std::borrow::Cow;
use std::string::ToString;

use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct RespErrors {
    pub errors: Vec<(String, String)>,
}

impl RespErrors {
    pub fn new(errors: Vec<(String, String)>) -> Self {
        RespErrors { errors }
    }

    pub fn new_error(error: (String, String)) -> Self {
        RespErrors {
            errors: vec![error],
        }
    }
}

pub trait ValidateFormatter {
    fn run_validator(&self) -> Result<(), RespErrors>;
}

impl<T> ValidateFormatter for T
where
    T: Validate,
{
    fn run_validator(&self) -> Result<(), RespErrors> {
        use std::collections::HashMap;

        self.validate().map_err(|err: ValidationErrors| {
            let errors: HashMap<&str, validator::ValidationErrorsKind> = err.into_errors();
            let mut err_entries = Vec::with_capacity(errors.len());

            for (field, err) in errors {
                match err {
                    ValidationErrorsKind::Field(vec) => {
                        if let Some(msg) = vec.into_iter().next() {
                            let message = msg
                                .message
                                .map(|err_text| err_text.to_string())
                                .unwrap_or_else(|| "field input is invalid".to_string());

                            let camel_field_name = snake_to_camel_case(field)
                                .unwrap_or_else(|_| panic!("Invalid snake field: {}", field))
                                .to_string();

                            err_entries.push((camel_field_name, message));
                        } else {
                            panic!("Some other type was given");
                        }
                    }
                    _ => panic!("Some other type was given"),
                }
            }
            RespErrors {
                errors: err_entries,
            }
        })
    }
}

pub fn snake_to_camel_case(snake: &str) -> Result<Cow<str>, ()> {
    if snake.contains('_') {
        if snake.starts_with('_') || snake.ends_with('_') {
            Err(())
        } else {
            let mut camel = "".to_string();
            for word in snake.split('_') {
                if &camel == "" {
                    camel.push_str(word);
                } else {
                    let (first, rest) = word.split_at(1);
                    camel.push_str(&first.to_uppercase());
                    camel.push_str(rest);
                }
            }
            Ok(Cow::Owned(camel))
        }
    } else {
        Ok(Cow::Borrowed(snake))
    }
}
