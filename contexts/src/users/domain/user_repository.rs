use anyhow::Result;

use crate::users::domain::users::User;

pub trait UserRepository {
    fn save(user: User) -> Result<()>;
}
