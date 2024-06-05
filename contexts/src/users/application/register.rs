use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::users::{User, UserErrors};
use crate::users::domain::users::user_repository::{RepositoryErrors, UserRepository};

#[derive(Error, Debug)]
pub enum UserRegisterErrors {
    #[error("The user that is trying to register is already registered")]
    AlreadyExists,
    #[error("The server has found an unexpected situation")]
    InternalServerError,
    #[error("User validation error")]
    UserError {
        #[from]
        source: UserErrors
    }
}

impl From<RepositoryErrors> for UserRegisterErrors {
    fn from(value: RepositoryErrors) -> Self {
        match value {
            RepositoryErrors::AlreadyExists => UserRegisterErrors::AlreadyExists,
            RepositoryErrors::InternalServerError { .. } => UserRegisterErrors::InternalServerError,
        }
    }
}

pub trait UserRegister: Interface {
    fn register(
        &self,
        uuid: &str,
        name: &str,
        password: &str,
        email: &str,
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
        uuid: &str,
        name: &str,
        password: &str,
        email: &str,
    ) -> Result<(), UserRegisterErrors> {
        let user = User::create(uuid, name, password, email)?;

        Ok(self.user_repository.save(&user)?)
    }
}
