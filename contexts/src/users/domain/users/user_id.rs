use std::borrow::{Cow};
use std::fmt::Display;

use thiserror::Error;
use uuid::{NoContext, Timestamp, Uuid};

use crate::users::domain::users::user_id::UserIDErrors::{InvalidUuid, InvalidUuidVersion};

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserID<'a>(Cow<'a, str>);

impl UserID<'_> {
    fn validate(value: &str) -> Result<(), UserIDErrors> {
        let err = Uuid::parse_str(value);

        let uuid = match err {
            Ok(uuid) => uuid,
            Err(err) => {
                return Err(InvalidUuid {
                    source: anyhow::Error::from(err),
                })
            }
        };

        let uuid_version = uuid.get_version_num();
        if uuid_version != UUID_TIMESTAMP_RAND_VERSION {
            return Err(InvalidUuidVersion(uuid_version));
        }

        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for UserID<'a> {
    type Error = UserIDErrors;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::validate(value)?;

        Ok(UserID(Cow::Borrowed(&value)))
    }
}

impl TryFrom<String> for UserID<'_> {
    type Error = UserIDErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(value.as_str())?;
        
        Ok(UserID(Cow::Owned(value)))
    }
}

impl<'a> Display for UserID<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserID<'_> {
    pub fn new() -> Self {
        let now = Timestamp::now(NoContext);
        let uuid = Uuid::new_v7(now).to_string();
        UserID(Cow::Owned(uuid))
    }
}

impl<'a> UserID<'a> {
    pub fn get(&self) -> &str {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> String {
        self.0.into_owned()
    }
}
