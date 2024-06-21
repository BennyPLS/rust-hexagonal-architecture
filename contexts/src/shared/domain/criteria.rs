use crate::shared::domain::criteria::filter::Filter;
use crate::shared::domain::criteria::order::Order;

pub mod filter;
pub mod order;

pub struct Criteria<'a> {
    pub filters: Vec<Filter<'a>>,
    pub order: Option<Order<'a>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl<'a> Criteria<'a> {
    fn new(
        filters: Vec<Filter<'a>>,
        order: Option<Order<'a>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Criteria<'a> {
        Criteria {
            filters,
            order,
            limit,
            offset,
        }
    }
}
