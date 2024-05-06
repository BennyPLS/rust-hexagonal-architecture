use crate::users::application::user_register_service::UserRegisterService;
use shaku::ModuleBuilder;

shaku::module! {
    pub DependencyContainer {
        components = [
            UserRegisterService
        ],
        providers = []
    }
}

pub async fn default_module() -> ModuleBuilder<DependencyContainer> {
    DependencyContainer::builder()
}
