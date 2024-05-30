use crate::users::application::register::UserRegisterService;
use crate::users::infrastructure::sqlite::user_repository_sqlite::{
    UserRepositorySQLite, UserRepositorySQLiteParameters,
};
use shaku::ModuleBuilder;
use sqlite::ConnectionThreadSafe;
use crate::users::application::delete::UserDeleteService;
use crate::users::application::find::UserFindService;
use crate::users::application::update::UserUpdateService;

shaku::module! {
    pub SQLiteImplementation {
        components = [
            UserRepositorySQLite,
            UserRegisterService,
            UserFindService,
            UserUpdateService,
            UserDeleteService
        ],
        providers = []
    }
}

pub fn build_sqlite_container(conn: ConnectionThreadSafe) -> ModuleBuilder<SQLiteImplementation> {
    SQLiteImplementation::builder().with_component_parameters::<UserRepositorySQLite>(
        UserRepositorySQLiteParameters { connection: conn },
    )
}
