use crate::shared::domain::criteria::filter::{Filter};
use crate::shared::domain::criteria::order::{Order};
use crate::shared::domain::criteria::Criteria;
use crate::users::domain::users::user_criteria_repository::{
    CriteriaRepositoryErrors, Result, UserCriteriaRepository,
};
use crate::users::domain::users::User;
use crate::users::infrastructure::sqlite::mappers::get_user;
use crate::users::infrastructure::sqlite::{ToSQLite, DATABASE_FILE, OP_LIKE};
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

// language=SQL
const BASE_STMT: &str = "SELECT * FROM users";
const ORDER_BY: &str = " ORDER BY ? ";
const LIMIT: &str = " LIMIT ?";
const OFFSET: &str = " OFFSET ?";

struct CriteriaQuery {
    pub query: String,
    pub parameters: Vec<String>,
}

impl CriteriaQuery {
    fn new() -> CriteriaQuery {
        CriteriaQuery {
            query: BASE_STMT.to_owned(),
            parameters: Vec::new(),
        }
    }

    fn add_filter(&mut self, filter: &Filter) {
        self.query += &format!(" WHERE ? {} ?", &filter.operator.to_sql());

        self.parameters.push(filter.field.to_string());

        if OP_LIKE.contains(&filter.operator) {
            self.parameters.push(format!("%{}%", filter.value));
        } else {
            self.parameters.push(filter.value.to_owned());
        }
    }

    fn add_order(&mut self, order: &Order) {
        self.query += ORDER_BY;
        self.query += order.ty.to_sql();

        self.parameters.push(order.field.to_owned());
    }

    fn add_offset(&mut self, offset: &u32) {
        self.query += OFFSET;
        self.parameters.push(offset.to_string());
    }

    fn add_limit(&mut self, limit: &u32) {
        self.query += LIMIT;
        self.parameters.push(limit.to_string());
    }
}

#[derive(Component)]
#[shaku(interface = UserCriteriaRepository)]
pub struct UserCriteriaRepositorySQLite {}

impl UserCriteriaRepository for UserCriteriaRepositorySQLite {
    fn find_by(&self, criteria: &Criteria) -> Result<Vec<User>> {
        let mut query = CriteriaQuery::new();

        for filter in &criteria.filters {
            query.add_filter(filter);
        }

        if let Some(order) = &criteria.order {
            query.add_order(order);
        }

        if let Some(limit) = &criteria.limit {
            query.add_limit(limit);
        }

        if let Some(offset) = &criteria.offset {
            query.add_offset(offset);
        }

        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(query.query)?;

        for (index, value) in query.parameters.iter().enumerate() {
            stmt.bind((index, value.as_str()))?;
        }

        let mut users = vec![];
        while let Ok(State::Row) = stmt.next() {
            users.push(get_user(&stmt));
        }

        Ok(users)
    }
}
