use crate::shared::domain::criteria::filter::Filter;
use crate::shared::domain::criteria::order::Order;

pub mod filter;
pub mod order;

pub struct Criteria {
    pub filters: Vec<Filter>,
    pub order: Option<Order>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
