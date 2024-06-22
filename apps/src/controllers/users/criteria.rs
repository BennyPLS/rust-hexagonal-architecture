use crate::controllers::users::UserResponse;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;
use contexts::shared::domain::criteria::filter::{Filter, Operator, OperatorNotFound};
use contexts::shared::domain::criteria::order::{Order, OrderType, OrderTypeNotFound};
use contexts::shared::domain::criteria::Criteria;
use contexts::users::application::criteria::{UserCriteria, UserCriteriaErrors};
use rocket::http::Status;
use serde_json::json;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, FromForm)]
pub struct CriteriaRequest<'a> {
    pub filters: Vec<FilterRequest<'a>>,
    pub order: Option<OrderRequest<'a>>,
    pub limit: Option<&'a str>,
    pub offset: Option<&'a str>,
}

#[derive(Debug, FromForm)]
pub struct FilterRequest<'a> {
    pub field: &'a str,
    pub operator: &'a str,
    pub value: &'a str,
}

#[derive(Debug, FromForm)]
pub struct OrderRequest<'a> {
    pub field: &'a str,
    pub ty: &'a str,
}

#[derive(Error, Debug)]
pub enum CriteriaError {
    #[error("{source}")]
    OperatorNotFound {
        #[from]
        source: OperatorNotFound,
    },
    #[error("{source}")]
    OrderTypeNotFound {
        #[from]
        source: OrderTypeNotFound,
    },
    #[error("{field}, Cannot be parsed into a digit")]
    ParseInt {
        field: &'static str,
        source: ParseIntError,
    },
}

impl From<CriteriaError> for ProblemDetail {
    fn from(value: CriteriaError) -> Self {
        ProblemDetailBuilder::from(Status::UnprocessableEntity)
            .detail(value.to_string())
            .build()
    }
}

impl<'a> TryFrom<CriteriaRequest<'a>> for Criteria<'a> {
    type Error = CriteriaError;

    fn try_from(value: CriteriaRequest<'a>) -> Result<Self, Self::Error> {
        let mut filters: Vec<Filter> = Vec::new();

        for x in value.filters {
            filters.push(Filter::try_from(x)?)
        }

        let order: Option<Order> = if let Some(value) = value.order {
            Some(Order::try_from(value)?)
        } else {
            None
        };

        let limit: Option<u32> = if let Some(value) = value.limit {
            match value.parse() {
                Ok(value) => Some(value),
                Err(err) => {
                    return Err(CriteriaError::ParseInt {
                        source: err,
                        field: "Limit",
                    })
                }
            }
        } else {
            None
        };

        let offset: Option<u32> = if let Some(value) = value.offset {
            match value.parse() {
                Ok(value) => Some(value),
                Err(err) => {
                    return Err(CriteriaError::ParseInt {
                        source: err,
                        field: "Offset",
                    })
                }
            }
        } else {
            None
        };

        Ok(Criteria::new(filters, order, limit, offset))
    }
}

impl<'a> TryFrom<FilterRequest<'a>> for Filter<'a> {
    type Error = OperatorNotFound;

    fn try_from(value: FilterRequest<'a>) -> Result<Self, Self::Error> {
        Ok(Filter::new(
            value.field,
            Operator::try_from(value.operator)?,
            value.value,
        ))
    }
}

impl<'a> TryFrom<OrderRequest<'a>> for Order<'a> {
    type Error = OrderTypeNotFound;

    fn try_from(value: OrderRequest<'a>) -> Result<Self, Self::Error> {
        Ok(Order::new(value.field, OrderType::try_from(value.ty)?))
    }
}

impl From<UserCriteriaErrors> for ProblemDetail {
    fn from(value: UserCriteriaErrors) -> Self {
        match value {
            UserCriteriaErrors::InternalServerError { source } => {
                let mut err = ProblemDetailBuilder::from(Status::InternalServerError);

                if let Some(source) = source {
                    err = err.detail(source.to_string());
                }

                err.build()
            }
            UserCriteriaErrors::FieldNotFound(_) => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(value.to_string())
                    .build()
            }
        }
    }
}

#[get("/?<filters>&<order>&<limit>&<offset>")]
pub fn user_criteria(
    filters: Vec<FilterRequest>,
    order: Option<OrderRequest>,
    limit: Option<&str>,
    offset: Option<&str>,
    criteria_service: Inject<'_, dyn UserCriteria>,
) -> Result<JsonResponse<Vec<UserResponse>>, ProblemDetail> {
    let criteria = CriteriaRequest {
        order,
        filters,
        limit,
        offset,
    };

    Ok(JsonResponse::ok(
        criteria_service
            .find_by(&Criteria::try_from(criteria)?)?
            .into_iter()
            .map(UserResponse::from)
            .collect(),
    ))
}
