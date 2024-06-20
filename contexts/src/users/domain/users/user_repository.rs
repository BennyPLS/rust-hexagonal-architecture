use shaku::Interface;
use std::result;
use thiserror::Error;

use crate::users::domain::users::user_id::UserID;
use crate::users::domain::users::User;

#[derive(Error, Debug)]
pub enum RepositoryErrors {
    #[error("The data trying to be stored is already there")]
    AlreadyExists,
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: anyhow::Error,
    },
}

type Result<T> = result::Result<T, RepositoryErrors>;

pub trait UserRepository: Interface {
    fn save(&self, user: &User) -> Result<()>;
    fn find_by(&self, id: &UserID) -> Option<User>;
    fn get_all(&self) -> Vec<User>;
    fn delete_by(&self, id: &UserID) -> Result<()>;
    fn update(&self, user: &User) -> Result<()>;
}
