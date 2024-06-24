use crate::shared::domain::regex::valid_email;
use thiserror::Error;

use crate::users::domain::users::user_email::UserEmailErrors::InvalidEmail;

#[derive(Debug)]
pub struct UserEmail<'a>(pub(crate) &'a str);

#[derive(Error, Debug)]
pub enum UserEmailErrors {
    #[error("Invalid format of an email")]
    InvalidEmail,
}

impl<'a> TryFrom<&'a str> for UserEmail<'a> {
    type Error = UserEmailErrors;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if !valid_email(&value) {
            return Err(InvalidEmail);
        }

        Ok(UserEmail(value))
    }
}

impl<'a> UserEmail<'a> {
    pub fn new(value: &'a str) -> Result<UserEmail<'a>, UserEmailErrors> {
        UserEmail::try_from(value)
    }

    pub fn get(&self) -> &'a str {
        self.0
    }
}
