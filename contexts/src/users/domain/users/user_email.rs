use crate::shared::domain::regex::valid_email;
use std::borrow::Cow;
use thiserror::Error;

use crate::users::domain::users::user_email::UserEmailErrors::InvalidEmail;

#[derive(Error, Debug)]
pub enum UserEmailErrors {
    #[error("Invalid format of an email")]
    InvalidEmail,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserEmail<'a>(pub(crate) Cow<'a, str>);

impl UserEmail<'_> {
    fn validate(value: &str) -> Result<(), UserEmailErrors> {
        if !valid_email(&value) {
            return Err(InvalidEmail);
        }

        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for UserEmail<'a> {
    type Error = UserEmailErrors;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::validate(value)?;

        Ok(UserEmail(value.into()))
    }
}

impl TryFrom<String> for UserEmail<'_> {
    type Error = UserEmailErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(value.as_str())?;

        Ok(UserEmail(value.into()))
    }
}

impl<'a> UserEmail<'a> {
    pub fn get(&self) -> &'a str {
        self.0.as_ref()
    }
}
