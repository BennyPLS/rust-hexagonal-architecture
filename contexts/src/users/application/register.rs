use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::User;

#[derive(Error, Debug)]
pub enum UserRegisterErrors {
    #[error("The user that is trying to register is already registered")]
    AlreadyExists,
    #[error("The server has found an unexpected situation")]
    InternalServerError,
}

impl From<RepositoryErrors> for UserRegisterErrors {
    fn from(value: RepositoryErrors) -> Self {
        match value {
            RepositoryErrors::AlreadyExists => UserRegisterErrors::AlreadyExists,
            RepositoryErrors::InternalServerError => UserRegisterErrors::InternalServerError,
        }
    }
}

pub trait UserRegister: Interface {
    fn register(
        &self,
        uuid: String,
        name: String,
        password: String,
        email: String,
    ) -> Result<(), UserRegisterErrors>;
}

#[derive(Component)]
#[shaku(interface = UserRegister)]
pub struct UserRegisterService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserRegister for UserRegisterService {
    fn register(
        &self,
        uuid: String,
        name: String,
        password: String,
        email: String,
    ) -> Result<(), UserRegisterErrors> {
        let user = User::create(uuid, name, password, email);

        Ok(self.user_repository.save(&user)?)
    }
}
