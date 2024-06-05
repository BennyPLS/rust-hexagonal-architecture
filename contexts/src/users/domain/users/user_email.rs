use regex::Regex;
use thiserror::Error;

use crate::users::domain::users::user_email::UserEmailErrors::InvalidEmail;

#[derive(Debug)]
pub struct UserEmail(pub(crate) String);


#[derive(Error, Debug)]
pub enum UserEmailErrors {
    #[error("Invalid format of an email")]
    InvalidEmail,
}

fn valid_email(email: &str) -> bool {
    let regex = Regex::new(r"^[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+$").unwrap();

    regex.is_match(email)
}

impl TryFrom<&str> for UserEmail {
    type Error = UserEmailErrors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !valid_email(&value) {
            return Err(InvalidEmail);
        }

        Ok(UserEmail(value.to_owned()))
    }
}

impl TryFrom<String> for UserEmail {
    type Error = UserEmailErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !valid_email(&value) {
            return Err(InvalidEmail);
        }

        Ok(UserEmail(value))
    }
}

impl UserEmail {
    pub fn new(value: String) -> Result<UserEmail, UserEmailErrors> {
        if !valid_email(&value) {
            return Err(InvalidEmail);
        }

        Ok(UserEmail(value))
    }
}
