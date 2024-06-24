use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use thiserror::Error;

use crate::users::domain::users::user_name::UserNameErrors::NotLongEnough;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserName<'a>(Cow<'a, str>);

pub const MIN_NAME_LENGTH: usize = 5;

#[derive(Error, Debug)]
pub enum UserNameErrors {
    #[error("Username of {0} chars not long enough, minimum is {MIN_NAME_LENGTH} chars")]
    NotLongEnough(usize),
}

impl<'a> TryFrom<&'a str> for UserName<'a> {
    type Error = UserNameErrors;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let length = value.chars().count();
        if length < MIN_NAME_LENGTH {
            return Err(NotLongEnough(length));
        }

        Ok(UserName(value.into()))
    }
}

impl TryFrom<String> for UserName<'_> {
    type Error = UserNameErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count();
        if length < MIN_NAME_LENGTH {
            return Err(NotLongEnough(length));
        }

        Ok(UserName(value.into()))
    }
}

impl Display for UserName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserName<'_> {
    pub fn new(value: String) -> Result<Self, UserNameErrors> {
        UserName::try_from(value)
    }
}

impl<'a> UserName<'a> {
    pub fn get(&self) -> &str {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> String {
        self.0.into_owned()
    }
}
