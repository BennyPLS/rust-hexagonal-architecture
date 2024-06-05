use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, SaltString};
use regex::Regex;
use thiserror::Error;

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

fn has_number(haystack: &str) -> bool {
    let regex = Regex::new(r"^.*(?=.*\d).*$").unwrap();

    regex.is_match(haystack)
}

fn has_symbol(haystack: &str) -> bool {
    let regex = Regex::new(r"^.*(?=.*[!@#$%^&*()_+?/:;\[\]{}|<>.,]).*$").unwrap();

    regex.is_match(haystack)
}

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
            Ok(ok) => Ok(UserPassword(ok.to_string())),
            Err(err) => Err(PHCFormatError { source: anyhow::Error::from(err) }),
        }
    }
}

impl UserPassword {
    pub fn new(password: &str) -> Result<UserPassword, UserPasswordErrors> {
        let length = password.chars().count();
        if length < MIN_PASSWORD_LENGTH {
            return Err(NotLongEnough(length));
        }

        if has_symbol(password) {
            return Err(Missing("Symbols"));
        }

        if has_number(password) {
            return Err(Missing("Numbers"));
        }

        let password_hash = hash_password_with_salt(password);

        Ok(UserPassword(password_hash))
    }
}
