use regex::Regex;
use thiserror::Error;
use crate::users::domain::users::user_email::UserEmailErrors::InvalidEmail;

#[derive(Debug)]
pub struct UserEmail(pub(crate) String);

const SYMBOL_REGEX: Regex = Regex::new(r"^[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+$").unwrap();

#[derive(Error, Debug)]
pub enum UserEmailErrors {
    #[error("Invalid format of an email")]
    InvalidEmail
}

impl UserEmail {
    pub fn new(value: String) -> Result<UserEmail, UserEmailErrors> {

        if !SYMBOL_REGEX.is_match(&value) {
            return Err(InvalidEmail);
        }

        Ok(UserEmail(value))
    }
}

