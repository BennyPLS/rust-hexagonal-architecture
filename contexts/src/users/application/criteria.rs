use crate::shared::domain::criteria::Criteria;
use crate::users::domain::users::user_criteria_repository::{
    CriteriaRepositoryErrors, UserCriteriaRepository,
};
use crate::users::domain::users::User;
use shaku::{Component, Interface};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserCriteriaErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: Option<anyhow::Error>,
    },
    #[error("The field {0} don't exist for user")]
    FieldNotFound(String),
}

impl From<CriteriaRepositoryErrors> for UserCriteriaErrors {
    fn from(value: CriteriaRepositoryErrors) -> Self {
        match value {
            CriteriaRepositoryErrors::InternalServerError { source } => {
                UserCriteriaErrors::InternalServerError {
                    source: Some(source),
                }
            }
            CriteriaRepositoryErrors::FieldNotFound(field) => {
                UserCriteriaErrors::FieldNotFound(field)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, UserCriteriaErrors>;

pub trait UserCriteria: Interface {
    fn find_by(&self, criteria: &Criteria) -> Result<Vec<User>>;
}

#[derive(Component)]
#[shaku(interface = UserCriteria)]
pub struct UserCriteriaService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserCriteriaRepository>,
}

impl UserCriteria for UserCriteriaService {
    fn find_by(&self, criteria: &Criteria) -> Result<Vec<User>> {
        Ok(self.user_repository.find_by(criteria)?)
    }
}
