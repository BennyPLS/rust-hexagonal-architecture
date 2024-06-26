use crate::users::application::criteria::UserCriteriaService;
use shaku::HasComponent;
use std::sync::Arc;

use crate::users::application::delete::UserDeleteService;
use crate::users::application::find::UserFindService;
use crate::users::application::register::UserRegisterService;
use crate::users::application::update::UserUpdateService;
use crate::users::domain::users::user_criteria_repository::UserCriteriaRepository;
use crate::users::domain::users::user_repository::UserRepository;

pub trait DatabaseModule:
    HasComponent<dyn UserRepository> + HasComponent<dyn UserCriteriaRepository>
{
}

shaku::module! {
    pub AppContainer {
        components = [
            UserRegisterService,
            UserFindService,
            UserUpdateService,
            UserDeleteService,
            UserCriteriaService
        ],
        providers = [],

        use dyn DatabaseModule {
            components = [
                dyn UserRepository,
                dyn UserCriteriaRepository
            ],
            providers = [],
        }
    }
}

pub fn build_container<T: DatabaseModule>(database: T) -> AppContainer {
    AppContainer::builder(Arc::new(database)).build()
}
