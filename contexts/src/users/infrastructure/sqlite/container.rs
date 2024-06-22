use crate::shared::infrastructure::dependency_container::DatabaseModule;
use crate::users::infrastructure::sqlite::init;
use crate::users::infrastructure::sqlite::user_criteria_repository_sqlite::UserCriteriaRepositorySQLite;
use crate::users::infrastructure::sqlite::user_repository_sqlite::UserRepositorySQLite;
use shaku::{module, Component};

module! {
    pub SQLiteDatabaseModule: DatabaseModule {
        components = [
            UserRepositorySQLite,
            UserCriteriaRepositorySQLite
        ],
        providers = []
    }
}

pub fn build_container() -> SQLiteDatabaseModule {
    init();

    SQLiteDatabaseModule::builder().build()
}
