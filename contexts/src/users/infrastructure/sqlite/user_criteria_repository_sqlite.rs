use crate::shared::domain::criteria::order::OrderType;
use crate::shared::domain::criteria::Criteria;
use crate::users::domain::users::user_criteria_repository::{
    CriteriaRepositoryErrors, Result, UserCriteriaRepository,
};
use crate::users::domain::users::User;
use crate::users::infrastructure::sqlite::mappers::get_user;
use crate::users::infrastructure::sqlite::DATABASE_FILE;
use shaku::Component;
use sqlite::{Error as SQLiteError, State};

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

// language=SQL
const STMT_INSERT: &str = "SELECT * FROM users";
const ADDITIONAL_WHERE: &str = " WHERE ? = ?";
const ORDER_BY_ASC: &str = " ORDER BY ?";
const ORDER_BY_DESC: &str = " ORDER BY ? DESC";
const LIMIT: &str = " LIMIT ?";
const OFFSET: &str = " OFFSET ?";

impl UserCriteriaRepository for UserCriteriaRepositorySQLite {
    fn find_by(&self, criteria: &Criteria) -> Result<Vec<User>> {
        let mut criteria_query = STMT_INSERT.to_owned();
        let mut parameters: Vec<(usize, &str)> = Vec::new();

        for (index, filter) in criteria.filters.iter().enumerate() {
            parameters.push((index + 1, &filter.value));
            criteria_query += ADDITIONAL_WHERE;
        }

        if let Some(order) = &criteria.order {
            match order.ty {
                OrderType::ASC => criteria_query += ORDER_BY_ASC,
                OrderType::DESC => criteria_query += ORDER_BY_DESC,
            }
            parameters.push((parameters.len() + 1, &order.field))
        }

        if let Some(limit) = &criteria.limit {
            criteria_query += LIMIT;
            parameters.push((parameters.len() + 1, limit.to_string().as_str()))
        }

        if let Some(offset) = &criteria.offset {
            criteria_query += LIMIT;
            parameters.push((parameters.len() + 1, offset.to_string().as_str()))
        }

        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(criteria_query)?;

        for parameter in parameters {
            stmt.bind(parameter)?
        }

        let mut users = vec![];
        while let Ok(State::Row) = stmt.next() {
            users.push(get_user(&stmt))
        }

        Ok(users)
    }
}
