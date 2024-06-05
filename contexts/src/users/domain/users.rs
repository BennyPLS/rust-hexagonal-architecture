use thiserror::Error;
use crate::users::domain::users::user_email::{UserEmail, UserEmailErrors};

use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_name::{UserName, UserNameErrors};
use crate::users::domain::users::user_password::{UserPassword, UserPasswordErrors};

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
    #[error("Failed to validate User Password")]
    UserPasswordError { 
        #[source]
        source: UserPasswordErrors
    },
    #[error("Failed to validate User Email")]
    UserEmailError {
        #[source]
        source: UserEmailErrors
    }
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

    pub fn create(id: String, name: String, password: String, email: String) -> Result<User, UserErrors> {
        Ok(User {
            id: UserID::try_from(id)?,
            name: UserName::try_from(name)?,
            password: UserPassword::new(&password)?,
            email: UserEmail::new(email)?,
        })
    }

    pub fn get_id(&self) -> &str {
        &self.id.to_string()
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
