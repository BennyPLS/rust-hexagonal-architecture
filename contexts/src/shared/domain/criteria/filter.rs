use crate::shared::domain::criteria::filter::Operator::{CO, EQ, GE, GT, LE, LT, NC};
use thiserror::Error;

pub struct Filter<'a> {
    pub field: &'a str,
    pub operator: Operator,
    pub value: &'a str,
}

impl<'a> Filter<'a> {
    fn new(field: &'a str, operator: Operator, value: &'a str) -> Filter<'a> {
        Filter {
            field,
            operator,
            value,
        }
    }
}

#[derive(Error, Debug)]
#[error("Operator not valid")]
pub struct OperatorNotFound;

pub enum Operator {
    EQ,
    GT,
    GE,
    LT,
    LE,
    CO,
    NC,
}

impl TryFrom<&str> for Operator {
    type Error = OperatorNotFound;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "eq" => Ok(EQ),
            "gt" => Ok(GT),
            "ge" => Ok(GE),
            "lt" => Ok(LT),
            "le" => Ok(LE),
            "co" => Ok(CO),
            "nc" => Ok(NC),
            _ => Err(OperatorNotFound),
        }
    }
}
