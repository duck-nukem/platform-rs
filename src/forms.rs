use std::{error::Error, fmt::Debug, fmt::Display};

#[allow(dead_code)]
enum ValidationError {
    Generic(String),
}

impl Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Generic(reason) => write!(f, "Validation Error: {}", reason),
        }
    }
}

impl Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Generic(reason) => write!(f, "Validation Error: {}", reason),
        }
    }
}

trait Validateable {
    fn validate(&self) -> Result<&Self, ValidationError> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestForm {
        first: String,
        second: String,
    }

    impl Validateable for TestForm {
        fn validate(&self) -> Result<&Self, ValidationError> {
            if self.first == self.second {
                Ok(self)
            } else {
                Err(ValidationError::Generic(
                    "First & second should be equal!".to_string(),
                ))
            }
        }
    }

    #[tokio::test]
    async fn test_forms_should_return_ok_if_valid() -> () {
        let form = TestForm {
            first: String::from("a"),
            second: String::from("a"),
        };

        let is_ok_with_form_data = form
            .validate()
            .is_ok_and(|form| form.first == String::from("a"));

        assert_eq!(is_ok_with_form_data, true)
    }

    #[tokio::test]
    async fn test_forms_should_return_err_if_validation_error_happens() -> () {
        let form = TestForm {
            first: String::from("a"),
            second: String::from("b"),
        };

        form.validate().expect_err("Expected error, got OK");

        assert_eq!(form.validate().is_ok(), false)
    }
}
