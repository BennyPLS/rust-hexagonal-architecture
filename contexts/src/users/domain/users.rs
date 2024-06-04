use thiserror::Error;

use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_name::{UserName, UserNameErrors};
use crate::users::domain::users::user_password::UserPassword;

pub mod user_id;
pub mod user_name;
mod user_password;
pub mod user_repository;
mod user_email;

#[derive(Error, Debug)]
enum UserErrors {
    #[error("Failed to validate User ID")]
    UserIDError {
        #[source]
        source: UserIDErrors,
    },
    #[error("Failed to validate User Name")]
    UserNameError {
        #[source]
        source: UserNameErrors,
    },
}

#[derive(Debug)]
pub struct User<'a> {
    id: UserID,
    name: UserName,
    password: UserPassword<'a>,
    email: UserEmail,
}

impl<'a> User<'a> {
    pub fn new(id: UserID, name: UserName, password: UserPassword<'a>, email: UserEmail) -> User {
        User {
            id,
            name,
            password,
            email,
        }
    }

    pub fn create(id: String, name: String, password: &'a str, email: String) -> User {
        User {
            id: UserID::try_from(id).unwrap(), // TODO: Add good error management.
            name: UserName::new(name).unwrap(),
            password: UserPassword::new(password).unwrap(),
            email: UserEmail::new(email),
        }
    }

    pub fn get_id(&self) -> &str {
        ""
    }

    pub fn get_name(&self) -> &str {
        &self.name.0
    }

    pub fn get_password(&self) -> &str {
        &self.password.0
    }

    pub fn get_email(&self) -> &str {
        &self.email.0
    }
}
