use crate::shared::domain::criteria::Criteria;
use crate::users::domain::users::user_criteria_repository::{
    CriteriaRepositoryErrors, Result, UserCriteriaRepository,
};
use crate::users::domain::users::User;
use crate::users::infrastructure::sqlite::mappers::get_user;
use crate::users::infrastructure::sqlite::{
    criteria_sqlite, DATABASE_FILE, USER_TABLE_FIELDS, USER_TABLE_NAME,
};
use shaku::Component;
use sqlite::{Error as SQLiteError};

impl From<SQLiteError> for CriteriaRepositoryErrors {
    fn from(value: SQLiteError) -> Self {
        if let Some(code) = value.code {
            match code {
                _ => unmapped_error(value),
            }
        } else {
            unmapped_error(value)
        }
    }
}

fn unmapped_error(error: SQLiteError) -> CriteriaRepositoryErrors {
    dbg!(&error);
    CriteriaRepositoryErrors::InternalServerError {
        source: anyhow::Error::from(error),
    }
}

#[derive(Component)]
#[shaku(interface = UserCriteriaRepository)]
pub struct UserCriteriaRepositorySQLite {}

impl UserCriteriaRepository for UserCriteriaRepositorySQLite {
    fn find_by(&self, criteria: &Criteria) -> Result<Vec<User>> {
        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        Ok(criteria_sqlite::find_by(
            &conn,
            USER_TABLE_NAME,
            &USER_TABLE_FIELDS,
            get_user,
            criteria,
        )?)
    }
}
