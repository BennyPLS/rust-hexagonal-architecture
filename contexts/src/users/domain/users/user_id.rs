use std::fmt::Display;

use thiserror::Error;
use uuid::{NoContext, Timestamp, Uuid};

use crate::users::domain::users::user_id::UserIDErrors::{InvalidUuid, InvalidUuidVersion};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserID(Uuid);

const UUID_TIMESTAMP_RAND_VERSION: usize = 7;

#[derive(Error, Debug)]
pub enum UserIDErrors {
    #[error("Not a valid UUID, {source}")]
    InvalidUuid {
        #[source]
        source: anyhow::Error,
    },
    #[error("Not a valid UUID Version v{0}, only allowed v7")]
    InvalidUuidVersion(usize),
}

impl TryFrom<&str> for UserID {
    type Error = UserIDErrors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = Uuid::parse_str(value);

        let uuid = match err {
            Ok(uuid) => uuid,
            Err(err) => {
                return Err(InvalidUuid {
                    source: anyhow::Error::from(err),
                })
            }
        };

        Ok(UserID::try_from(uuid)?)
    }
}

impl TryFrom<Uuid> for UserID {
    type Error = UserIDErrors;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        let uuid_version = value.get_version_num();
        if uuid_version == UUID_TIMESTAMP_RAND_VERSION {
            return Err(InvalidUuidVersion(uuid_version));
        }

        Ok(UserID(value))
    }
}

impl Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl UserID {
    pub fn new() -> UserID {
        let timestamp = Timestamp::now(NoContext);
        let uuid = Uuid::new_v7(timestamp);
        UserID(uuid)
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}
