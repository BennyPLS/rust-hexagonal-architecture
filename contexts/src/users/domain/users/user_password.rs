use crate::shared::domain::regex::{has_number, has_symbol};
use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, SaltString};
use std::borrow::Cow;
use thiserror::Error;

use crate::users::domain::users::user_password::UserPasswordErrors::{
    Missing, NotLongEnough, PHCFormatError,
};

fn hash_password_with_salt(password: &str) -> String {
    let salt = SaltString::generate(OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Eq, PartialEq)]
pub struct UserPassword<'a>(Cow<'a, str>);

#[derive(Error, Debug)]
pub enum UserPasswordErrors {
    #[error("Password of {0} is not long enough, minimum {MIN_PASSWORD_LENGTH} characters long")]
    NotLongEnough(usize),
    #[error("Password is missing {0}, at least one required")]
    Missing(&'static str),
    #[error("PHC Format Error, {source}")]
    PHCFormatError {
        #[source]
        source: anyhow::Error,
    },
}

impl<'a> TryFrom<&'a str> for UserPassword<'a> {
    type Error = UserPasswordErrors;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let res = PasswordHash::try_from(value);

        match res {
            Ok(_) => Ok(UserPassword(value.into())),
            Err(err) => Err(PHCFormatError {
                source: anyhow::Error::from(err),
            }),
        }
    }
}

impl TryFrom<String> for UserPassword<'_> {
    type Error = UserPasswordErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let res = PasswordHash::try_from(value.as_str());

        match res {
            Ok(_) => Ok(UserPassword(value.into())),
            Err(err) => Err(PHCFormatError {
                source: anyhow::Error::from(err),
            }),
        }
    }
}

impl UserPassword<'_> {
    pub fn new(password: &str) -> Result<Self, UserPasswordErrors> {
        let length = password.chars().count();
        if length < MIN_PASSWORD_LENGTH {
            return Err(NotLongEnough(length));
        }

        if !has_symbol(password) {
            return Err(Missing("Symbols"));
        }

        if !has_number(password) {
            return Err(Missing("Numbers"));
        }

        let password_hash = hash_password_with_salt(password);

        Ok(UserPassword(password_hash.into()))
    }
}

impl<'a> UserPassword<'a> {
    pub fn get(&self) -> &str {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> String {
        self.0.into_owned()
    }
}
