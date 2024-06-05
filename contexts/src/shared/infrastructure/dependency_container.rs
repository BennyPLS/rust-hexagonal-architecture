use shaku::ModuleBuilder;

use crate::users::application::delete::UserDeleteService;
use crate::users::application::find::UserFindService;
use crate::users::application::register::UserRegisterService;
use crate::users::application::update::UserUpdateService;
use crate::users::infrastructure::sqlite::SQLiteDatabase;
use crate::users::infrastructure::sqlite::user_repository_sqlite::UserRepositorySQLite;

shaku::module! {
    pub SQLiteImplementation {
        components = [
            UserRepositorySQLite,
            UserRegisterService,
            UserFindService,
            UserUpdateService,
            UserDeleteService,
            SQLiteDatabase
        ],
        providers = []
    }
}

pub fn build_sqlite_container() -> ModuleBuilder<SQLiteImplementation> {
    SQLiteImplementation::builder()
}
