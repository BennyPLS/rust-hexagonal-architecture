use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::users::user_id::{UserID, UserIDErrors};
use crate::users::domain::users::user_repository::{ReadErrors, UserRepository};
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

impl From<ReadErrors> for UserFindErrors {
    fn from(value: ReadErrors) -> Self {
        match value {
            ReadErrors::InternalServerError { source } => {
                UserFindErrors::InternalServerError {
                    source: Some(source),
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum UserListErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: Option<anyhow::Error>,
    },
}

impl From<ReadErrors> for UserListErrors {
    fn from(value: ReadErrors) -> Self {
        match value {
            ReadErrors::InternalServerError { source } => {
                UserListErrors::InternalServerError {
                    source: Some(source),
                }
            }
        }
    }
}

pub trait UserFind: Interface {
    fn find_by(&self, id: &str) -> Result<Option<User>, UserFindErrors>;
    fn get_all(&self) ->  Result<Vec<User>, UserListErrors>;
}

#[derive(Component)]
#[shaku(interface = UserFind)]
pub struct UserFindService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserFind for UserFindService {
    fn find_by(&self, id: &str) -> Result<Option<User>, UserFindErrors> {
        Ok(self.user_repository.find_by(&UserID::try_from(id)?)?)
    }

    fn get_all(&self) -> Result<Vec<User>, UserListErrors> {
        Ok(self.user_repository.get_all()?)
    }
}
