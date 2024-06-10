use thiserror::Error;
use crate::shared::domain::regex::valid_email;

use crate::users::domain::users::user_email::UserEmailErrors::InvalidEmail;

#[derive(Debug)]
pub struct UserEmail(pub(crate) String);

#[derive(Error, Debug)]
pub enum UserEmailErrors {
    #[error("Invalid format of an email")]
    InvalidEmail,
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
        UserEmail::try_from(value.as_str())
    }
}

impl UserEmail {
    pub fn new(value: String) -> Result<UserEmail, UserEmailErrors> {
        UserEmail::try_from(value)
    }
}
