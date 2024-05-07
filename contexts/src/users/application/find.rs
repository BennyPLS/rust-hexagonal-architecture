use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::User;

#[derive(Error, Debug)]
pub enum UserFindErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError,
}

impl From<RepositoryErrors> for UserFindErrors {
    fn from(value: RepositoryErrors) -> Self {
        match value {
            _ => UserFindErrors::InternalServerError,
        }
    }
}

pub trait UserFind: Interface {
    fn find_by(&self, id: String) -> Option<User>;
    fn find_like(&self, name: String) -> Option<Vec<User>>;
}

#[derive(Component)]
#[shaku(interface = UserFind)]
pub struct UserFindService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserFind for UserFindService {
    fn find_by(&self, id: String) -> Option<User> {
        todo!()
    }

    fn find_like(&self, name: String) -> Option<Vec<User>> {
        todo!()
    }
}
