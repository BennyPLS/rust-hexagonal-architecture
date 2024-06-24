use thiserror::Error;

use crate::users::domain::users::user_email::{UserEmail, UserEmailErrors};
use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_name::{UserName, UserNameErrors};
use crate::users::domain::users::user_password::{UserPassword, UserPasswordErrors};

pub mod user_criteria_repository;
pub mod user_email;
pub mod user_id;
pub mod user_name;
pub mod user_password;
pub mod user_repository;

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
pub struct User<'a> {
    id: UserID<'a>,
    name: UserName<'a>,
    password: UserPassword<'a>,
    email: UserEmail<'a>,
}

impl<'a> User<'a> {
    pub fn new(
        id: UserID<'a>,
        name: UserName<'a>,
        password: UserPassword<'a>,
        email: UserEmail<'a>,
    ) -> Self {
        User {
            id,
            name,
            password,
            email,
        }
    }

    pub fn create(
        id: &'a str,
        name: &'a str,
        password: &'a str,
        email: &'a str,
    ) -> Result<User<'a>, UserErrors> {
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
        name: Option<&'a str>,
        password: Option<&'a str>,
        email: Option<&'a str>,
    ) -> Result<User<'a>, UserErrors> {
        let password = match password {
            None => self.password,
            Some(password) => UserPassword::new(password)?,
        };

        let name = match name {
            None => self.name,
            Some(name) => UserName::try_from(name)?,
        };

        let email = match email {
            None => self.email,
            Some(email) => UserEmail::try_from(email)?,
        };

        Ok(User {
            id: self.id,
            name,
            password,
            email,
        })

        // TODO : Event Driven Design (Update Events)
    }

    pub fn delete(&self) {
        // TODO : Event Driven Design (Delete Events)
    }

    pub fn get_id(&self) -> &str {
        self.id.get()
    }

    pub fn get_name(&self) -> &str {
        &self.name.get()
    }

    pub fn get_password(&self) -> &str {
        &self.password.get()
    }

    pub fn get_email(&self) -> &str {
        &self.email.get()
    }
    
    pub fn into_inners(self) -> (String, String, String, String) {
        (self.id.into_owned(), self.name.into_owned(), self.password.into_owned(), self.email.into_owned())
    }
}
