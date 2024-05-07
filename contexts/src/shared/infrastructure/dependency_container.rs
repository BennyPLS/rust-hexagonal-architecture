use crate::users::application::register::UserRegisterService;
use crate::users::infrastructure::sqlite::user_repository_sqlite::{
    UserRepositorySQLite, UserRepositorySQLiteParameters,
};
use shaku::ModuleBuilder;
use sqlite::ConnectionThreadSafe;

shaku::module! {
    pub SQLiteImplementation {
        components = [
            UserRepositorySQLite,
            UserRegisterService,
        ],
        providers = []
    }
}

pub fn build_sqlite_container(conn: ConnectionThreadSafe) -> ModuleBuilder<SQLiteImplementation> {
    SQLiteImplementation::builder().with_component_parameters::<UserRepositorySQLite>(
        UserRepositorySQLiteParameters { connection: conn },
    )
}
