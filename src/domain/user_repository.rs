use crate::domain::users::User;
use anyhow::Result;

pub trait UserRepository {
    fn save(user: User) -> Result<()>;
}
