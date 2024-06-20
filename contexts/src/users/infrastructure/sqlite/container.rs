use crate::shared::infrastructure::dependency_container::DatabaseModule;
use crate::users::infrastructure::sqlite::init;
use crate::users::infrastructure::sqlite::user_repository_sqlite::UserRepositorySQLite;
use shaku::{module, Component};

module! {
    SQLiteDatabaseModule: DatabaseModule {
        components = [
            UserRepositorySQLite,
        ],
        providers = []
    }
}

pub fn build_container() -> SQLiteDatabaseModule {
    init();

    SQLiteDatabaseModule::builder().build()
}
