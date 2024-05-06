use crate::users::domain::users::User;
use shaku::Interface;

pub trait UserRepository: Interface {
    type Error;

    fn save(&self, user: User) -> Result<(), Self::Error>;
}
