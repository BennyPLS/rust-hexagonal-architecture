use thiserror::Error;
use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_name::{UserName, UserNameErrors};

pub mod user_repository;
pub mod user_name;
pub mod user_id;
mod user_password;

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(value: String) -> UserEmail {
        UserEmail(value)
    }
}

#[derive(Error, Debug)]
enum UserErrors {
    #[error("Failed to validate User ID")]
    UserIDError {
        #[source]
        source: UserIDErrors
    },
    #[error("Failed to validate User Name")]
    UserNameError {
        #[source]
        source: UserNameErrors
    },
}

#[derive(Debug)]
pub struct User {
    id: UserID,
    name: UserName,
    password: UserPassword,
    email: UserEmail,
}

impl User {
    pub fn new(id: UserID, name: UserName, password: UserPassword, email: UserEmail) -> User {
        User {
            id,
            name,
            password,
            email,
        }
    }

    pub fn create(id: String, name: String, password: String, email: String) -> User {
        User {
            id: UserID::try_from(id).unwrap(), // TODO: Add good error management.
            name: UserName::try_from(name).unwrap(),
            password: UserPassword::new(password),
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
