use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::User;

#[derive(Error, Debug)]
pub enum UserUpdateErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError,
}

impl From<RepositoryErrors> for UserUpdateErrors {
    fn from(value: RepositoryErrors) -> Self {
        match value {
            _ => UserUpdateErrors::InternalServerError,
        }
    }
}

pub trait UserUpdate: Interface {
    fn update(&self, user: &User) -> Result<(), UserUpdateErrors>;
}

#[derive(Component)]
#[shaku(interface = UserUpdate)]
pub struct UserUpdateService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserUpdate for UserUpdateService {
    fn update(&self, user: &User) -> Result<(), UserUpdateErrors> {
        self.user_repository.update(user)?;

        Ok(())
    }
}
