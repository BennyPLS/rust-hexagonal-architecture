use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_repository::{UserRepositoryErrors, UserRepository};
use crate::users::domain::users::{User};

#[derive(Error, Debug)]
pub enum UserFindErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: Option<anyhow::Error>,
    },
    #[error("UserID validation error")]
    UserIDError {
        #[from]
        source: UserIDErrors,
    },
}

impl From<UserRepositoryErrors> for UserFindErrors {
    fn from(value: UserRepositoryErrors) -> Self {
        match value {
            UserRepositoryErrors::InternalServerError { source } => {
                UserFindErrors::InternalServerError {
                    source: Some(source),
                }
            }
            _ => UserFindErrors::InternalServerError { source: None },
        }
    }
}

pub trait UserFind: Interface {
    fn find_by(&self, id: &str) -> Result<Option<User>, UserFindErrors>;
    fn get_all(&self) -> Vec<User>;
}

#[derive(Component)]
#[shaku(interface = UserFind)]
pub struct UserFindService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserFind for UserFindService {
    fn find_by(&self, id: &str) -> Result<Option<User>, UserFindErrors> {
        Ok(self.user_repository.find_by(&UserID::try_from(id)?))
    }

    fn get_all(&self) -> Vec<User> {
        self.user_repository.get_all()
    }
}
