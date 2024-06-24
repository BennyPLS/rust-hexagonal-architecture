use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;

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
pub struct OwnedUserID(pub(crate) String);

impl TryFrom<String> for OwnedUserID {
    type Error = UserIDErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let err = Uuid::parse_str(value.as_str());

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

        Ok(OwnedUserID(value))
    }
}

impl OwnedUserID {
    pub fn new() -> Self {
        let uuid = Uuid::new_v7(Timestamp::now(NoContext)).to_string();
        OwnedUserID(uuid)
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserID<'a>(pub(crate) &'a str);

impl<'a> TryFrom<&'a str> for UserID<'a> {
    type Error = UserIDErrors;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
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

        Ok(UserID(value))
    }
}

impl<'a> From<&'a OwnedUserID> for UserID<'a> {
    fn from(owned: &'a OwnedUserID) -> Self {
        UserID(&owned.0)
    }
}

impl<'a> Display for UserID<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> UserID<'a> {
    pub fn get(&self) -> &'a str {
        self.0
    }

    pub fn to_owned(&self) -> OwnedUserID {
        OwnedUserID(self.0.to_owned())
    }
}
