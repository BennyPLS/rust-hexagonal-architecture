use crate::shared::domain::criteria::order::OrderType::{ASC, DESC};
use thiserror::Error;

pub struct Order<'a> {
    pub field: &'a str,
    pub ty: OrderType,
}

impl<'a> Order<'a> {
    fn new(field: &'a str, ty: OrderType) -> Order<'a> {
        Order { field, ty }
    }
}

#[derive(Error, Debug)]
#[error("Order type not valid")]
pub struct OrderTypeNotFound;

pub enum OrderType {
    ASC,
    DESC,
}

impl TryFrom<&str> for OrderType {
    type Error = OrderTypeNotFound;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "asc" => Ok(ASC),
            "desc" => Ok(DESC),
            _ => Err(OrderTypeNotFound),
        }
    }
}
