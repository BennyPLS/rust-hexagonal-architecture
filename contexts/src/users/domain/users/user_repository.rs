use shaku::Interface;
use thiserror::Error;

use crate::users::domain::users::User;

#[derive(Error, Debug)]
pub enum RepositoryErrors {
    #[error("The data trying to be stored is already there")]
    AlreadyExists,
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: anyhow::Error,
    }
}

type UserRepositoryResult<T> = Result<T, RepositoryErrors>;

pub trait UserRepository: Interface {
    fn save(&self, user: &User) -> UserRepositoryResult<()>;
    fn find_by(&self, id: &str) -> Option<User>;
    fn get_all(&self) -> Vec<User>;
    fn delete_by(&self, id: &str) -> UserRepositoryResult<()>;
    fn update(&self, user: &User) -> UserRepositoryResult<()>;
}
