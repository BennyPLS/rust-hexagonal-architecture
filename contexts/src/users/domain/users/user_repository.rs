use shaku::Interface;
use thiserror::Error;

use crate::users::domain::users::user_id::UserID;
use crate::users::domain::users::User;

#[derive(Error, Debug)]
pub enum SaveErrors {
    #[error("The data trying to be stored is already there")]
    AlreadyExists,
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[from]
        source: anyhow::Error,
    },
}

#[derive(Error, Debug)]
pub enum FindErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[from]
        source: anyhow::Error,
    },
}

#[derive(Error, Debug)]
pub enum DeleteErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[from]
        source: anyhow::Error,
    },
}

#[derive(Error, Debug)]
pub enum UpdateErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[from]
        source: anyhow::Error,
    },
}

pub trait UserRepository: Interface {
    fn save(&self, user: &User) -> Result<(), SaveErrors>;
    fn find_by(&self, id: &UserID) -> Result<Option<User>, FindErrors>;
    fn get_all(&self) -> Result<Vec<User>, FindErrors>;
    fn delete_by(&self, id: &UserID) -> Result<(), DeleteErrors>;
    fn update(&self, user: &User) -> Result<(), UpdateErrors>;
}
