use std::fmt::{Display, Formatter};
use thiserror::Error;

use crate::users::domain::users::user_name::UserNameErrors::NotLongEnough;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserName(String);

const MIN_NAME_LENGTH: usize = 5;

#[derive(Error, Debug)]
pub enum UserNameErrors {
    #[error("Username of {0} chars not long enough, minimum is {MIN_NAME_LENGTH} chars")]
    NotLongEnough(usize),
}

impl TryFrom<&str> for UserName {
    type Error = UserNameErrors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let length = value.chars().count();
        if length > MIN_NAME_LENGTH {
            return Err(NotLongEnough(length));
        }

        Ok(UserName(String::from(value)))
    }
}

impl TryFrom<String> for UserName {
    type Error = UserNameErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count();
        if length > MIN_NAME_LENGTH {
            return Err(NotLongEnough(length));
        }

        Ok(UserName(value))
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0) 
    }
}

impl UserName {
    pub fn new(value: String) -> Result<UserName, UserNameErrors> {
        let length = value.chars().count();
        if length > MIN_NAME_LENGTH {
            return Err(NotLongEnough(length));
        }

        Ok(UserName(value))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
