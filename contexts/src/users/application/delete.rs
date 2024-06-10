use std::sync::Arc;

use shaku::{Component, Interface};
use thiserror::Error;

use crate::users::domain::users::user_repository::{RepositoryErrors, UserRepository};

#[derive(Error, Debug)]
pub enum UserDeleteErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: Option<anyhow::Error>,
    },
}

impl From<RepositoryErrors> for UserDeleteErrors {
    fn from(value: RepositoryErrors) -> Self {
        match value {
            RepositoryErrors::InternalServerError { source } => {
                UserDeleteErrors::InternalServerError {
                    source: Some(source),
                }
            }
            _ => UserDeleteErrors::InternalServerError { source: None },
        }
    }
}

pub trait UserDelete: Interface {
    fn delete_by(&self, id: &str) -> Result<(), UserDeleteErrors>;
}

#[derive(Component)]
#[shaku(interface = UserDelete)]
pub struct UserDeleteService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserDelete for UserDeleteService {
    fn delete_by(&self, id: &str) -> Result<(), UserDeleteErrors> {
        self.user_repository.delete_by(id)?;

        Ok(())
    }
}
