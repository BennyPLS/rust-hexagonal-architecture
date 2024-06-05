use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::application::find::{UserFind, UserFindErrors};
use crate::users::application::update::UserUpdateErrors::NotFound;
use crate::users::domain::users::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::UserErrors;

#[derive(Error, Debug)]
pub enum UserUpdateErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: Option<anyhow::Error>,
    },
    #[error("User validation error")]
    UserError {
        #[from]
        source: UserErrors
    },
    #[error("User not found")]
    NotFound,
}

impl From<RepositoryErrors> for UserUpdateErrors {
    fn from(value: RepositoryErrors) -> Self {
        match value {
            RepositoryErrors::InternalServerError { source } => {
                UserUpdateErrors::InternalServerError {
                    source: Some(source),
                }
            }
            _ => UserUpdateErrors::InternalServerError { source: None },
        }
    }
}

impl From<UserFindErrors> for UserUpdateErrors {
    fn from(value: UserFindErrors) -> Self {
        match value {
            UserFindErrors::InternalServerError { source} => UserUpdateErrors::InternalServerError { source },
        }
    }
}

pub trait UserUpdate: Interface {
    fn update(
        &self,
        id: &str,
        name: Option<&str>,
        password: Option<&str>,
        email: Option<&str>,
    ) -> Result<(), UserUpdateErrors>;
}

#[derive(Component)]
#[shaku(interface = UserUpdate)]
pub struct UserUpdateService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
    user_find_service: Arc<dyn UserFind>,
}

impl UserUpdate for UserUpdateService {
    fn update(
        &self,
        id: &str,
        name: Option<&str>,
        password: Option<&str>,
        email: Option<&str>,
    ) -> Result<(), UserUpdateErrors> {
        let user = self.user_find_service.find_by(id);

        let user = if let Some(user) = user {
            user.update(name, password, email)?
        } else {
            return Err(NotFound);
        };

        self.user_repository.update(&user)?;

        Ok(())
    }
}
