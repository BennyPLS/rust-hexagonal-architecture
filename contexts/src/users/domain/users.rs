use thiserror::Error;

use crate::users::domain::users::user_email::{UserEmail, UserEmailErrors};
use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_name::{UserName, UserNameErrors};
use crate::users::domain::users::user_password::{UserPassword, UserPasswordErrors};

pub mod user_email;
pub mod user_id;
pub mod user_name;
pub mod user_password;
pub mod user_repository;
pub mod user_criteria_repository;

/// Errors that can occur during user validation.
#[derive(Error, Debug)]
pub enum UserErrors {
    /// Represents an error that occurs when validating a user ID.
    #[error("Failed to validate User ID")]
    UserIDError {
        #[from]
        source: UserIDErrors,
    },

    /// Represents an error that occurs when validating a username.
    #[error("Failed to validate User Name")]
    UserNameError {
        #[from]
        source: UserNameErrors,
    },

    /// Represents an error that occurs when validating a user password.
    #[error("Failed to validate User Password")]
    UserPasswordError {
        #[from]
        source: UserPasswordErrors,
    },

    /// Represents an error that occurs when validating a user email.
    #[error("Failed to validate User Email")]
    UserEmailError {
        #[from]
        source: UserEmailErrors,
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

    pub fn create(id: &str, name: &str, password: &str, email: &str) -> Result<User, UserErrors> {
        Ok(User {
            id: UserID::try_from(id)?,
            name: UserName::try_from(name)?,
            password: UserPassword::new(&password)?,
            email: UserEmail::try_from(email)?,
        })

        // TODO : Event Driven Design (Create Events)
    }

    pub fn update(
        self,
        name: Option<&str>,
        password: Option<&str>,
        email: Option<&str>,
    ) -> Result<User, UserErrors> {
        let password = match password {
            None => self.password,
            Some(password) => UserPassword::new(password)?,
        };

        Ok(User {
            id: self.id,
            name: UserName::try_from(name.unwrap_or(&self.name.0))?,
            password,
            email: UserEmail::try_from(email.unwrap_or(&self.email.0))?,
        })

        // TODO : Event Driven Design (Update Events)
    }

    pub fn delete(&self) {
        // TODO : Event Driven Design (Delete Events)
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
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
