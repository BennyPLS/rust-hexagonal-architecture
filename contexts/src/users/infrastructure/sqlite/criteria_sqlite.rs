use crate::shared::domain::criteria::filter::Filter;
use crate::shared::domain::criteria::order::Order;
use crate::shared::domain::criteria::Criteria;
use crate::users::domain::users::user_criteria_repository::CriteriaRepositoryErrors::FieldNotFound;
use crate::users::domain::users::user_criteria_repository::Result;
use crate::users::infrastructure::sqlite::{ToSQLite, OP_LIKE};
use sqlite::{Connection, State, Statement};

const ORDER_BY: &str = " ORDER BY ? ";
const LIMIT: &str = " LIMIT ?";
const OFFSET: &str = " OFFSET ?";

#[derive(Debug)]
struct CriteriaQuery {
    pub query: String,
    pub parameters: Vec<String>,
}

impl CriteriaQuery {
    fn new(table: &str) -> CriteriaQuery {
        CriteriaQuery {
            query: format!("SELECT * FROM {}", table),
            parameters: Vec::new(),
        }
    }

    fn add_filter(&mut self, filter: &Filter, valid_fields: &[&str]) -> Result<()> {
        if !valid_fields.contains(&filter.field) {
            return Err(FieldNotFound(filter.field.to_owned()));
        };

        self.query += &format!(" WHERE {} {} ?", filter.field, &filter.operator.to_sql());

        if OP_LIKE.contains(&filter.operator) {
            self.parameters.push(format!("%{}%", filter.value));
        } else {
            self.parameters.push(filter.value.to_owned());
        }

        Ok(())
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

pub fn find_by<T>(
    conn: &Connection,
    table: &str,
    valid_fields: &[&str],
    mapper: impl Fn(&Statement) -> T,
    criteria: &Criteria,
) -> Result<Vec<T>> {
    let mut query = CriteriaQuery::new(table);

    for filter in &criteria.filters {
        query.add_filter(filter, valid_fields)?;
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

    let mut stmt = conn.prepare(query.query)?;

    for (index, value) in query.parameters.iter().enumerate() {
        stmt.bind((index + 1, value.as_str()))?;
    }

    let mut objects = vec![];
    while let Ok(State::Row) = stmt.next() {
        objects.push(mapper(&stmt));
    }

    Ok(objects)
}
