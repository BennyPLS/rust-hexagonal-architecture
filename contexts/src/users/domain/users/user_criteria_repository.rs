use crate::shared::domain::criteria::Criteria;
use crate::users::domain::users::User;
use shaku::Interface;
use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CriteriaRepositoryErrors {
    #[error("The server has found an unexpected situation")]
    InternalServerError {
        #[source]
        source: anyhow::Error,
    },
}

pub type Result<T> = result::Result<T, CriteriaRepositoryErrors>;

pub trait UserCriteriaRepository: Interface {
    fn find_by(&self, criteria: Criteria) -> Result<Vec<User>>;
}
