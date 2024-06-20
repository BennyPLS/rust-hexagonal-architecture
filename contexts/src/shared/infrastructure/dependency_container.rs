use shaku::{HasComponent};

use crate::users::application::delete::UserDeleteService;
use crate::users::application::find::UserFindService;
use crate::users::application::register::UserRegisterService;
use crate::users::application::update::UserUpdateService;
use crate::users::domain::users::user_repository::UserRepository;

pub trait DatabaseModule: HasComponent<dyn UserRepository> {}

shaku::module! {
    pub AppContainer {
        components = [
            UserRegisterService,
            UserFindService,
            UserUpdateService,
            UserDeleteService,
        ],
        providers = [],

        use dyn DatabaseModule {
            components = [dyn UserRepository],
            providers = [],
        }
    }
}

pub fn build_container<T: DatabaseModule>(database: T) -> AppContainer {
    AppContainer::builder(database).build()
}
