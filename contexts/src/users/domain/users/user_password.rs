use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, SaltString};
use password_hash::rand_core::OsRng;
use regex::Regex;
use thiserror::Error;

use crate::users::domain::users::user_password::UserPasswordErrors::{Missing, NotLongEnough};

const ARGON2: Argon2 = Argon2::default();

fn hash_password_with_salt(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    ARGON2
        .hash_password(password.as_bytes(), &salt)?
        .to_string()
}

const MIN_PASSWORD_LENGTH: usize = 8;
const SYMBOL_REGEX: Regex = Regex::new(r"^.*(?=.*[!@#$%^&*()_+?/:;\[\]{}|<>.,]).*$").unwrap();
const NUMBER_REGEX: Regex = Regex::new(r"^.*(?=.*\d).*$").unwrap();

#[derive(Debug, Eq, PartialEq)]
pub struct UserPassword<'a>(PasswordHash<'a>);

#[derive(Error, Debug)]
enum UserPasswordErrors {
    #[error("Password of {0} is not long enough, minimum {MIN_PASSWORD_LENGTH} characters long")]
    NotLongEnough(usize),
    #[error("Password is missing {0}, at least one required")]
    Missing(&'static str),
}

impl<'a> UserPassword<'a> {
    pub fn new(password: &'a str) -> Result<UserPassword<'a>, UserPasswordErrors> {

        let length = password.chars().count();
        if length < MIN_PASSWORD_LENGTH {
            return Err(NotLongEnough(length));
        }

        if SYMBOL_REGEX.find(password).is_none() {
            return Err(Missing("Symbols"));
        }

        if NUMBER_REGEX.find(password).is_none() {
            return Err(Missing("Numbers"));
        }

        let password_hash = Self::hash_password_with_salt(password);

        Ok(UserPassword(password_hash))
    }
}
