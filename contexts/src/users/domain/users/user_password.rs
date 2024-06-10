use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, SaltString};
use thiserror::Error;
use crate::shared::domain::regex::{has_number, has_symbol};

use crate::users::domain::users::user_password::UserPasswordErrors::{
    Missing, NotLongEnough, PHCFormatError,
};

fn hash_password_with_salt(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Eq, PartialEq)]
pub struct UserPassword(pub(crate) String);

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

impl TryFrom<&str> for UserPassword {
    type Error = UserPasswordErrors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let res = PasswordHash::try_from(value);

        match res {
            Ok(phc_string) => Ok(UserPassword(phc_string.to_string())),
            Err(err) => Err(PHCFormatError { source: anyhow::Error::from(err) }),
        }
    }
}

impl TryFrom<String> for UserPassword {
    type Error = UserPasswordErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        UserPassword::try_from(value.as_str())
    }
}

impl UserPassword {
    pub fn new(password: &str) -> Result<UserPassword, UserPasswordErrors> {
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

        Ok(UserPassword(password_hash))
    }
}
