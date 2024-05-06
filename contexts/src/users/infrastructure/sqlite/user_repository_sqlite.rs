use sqlite::Connection;

struct UserRepositorySQLite {
    connection: Connection,
}

impl UserRepositorySQLite {
    fn new() -> UserRepositorySQLite {
        let db = sqlite::open("user.sqlite").expect("Error, couldn't open sqlite database");

        let init_tables = "CREATE TABLE users(id TEXT,name TEXT,password TEXT,email TEXT)";

        db.execute(init_tables)
            .expect("Error, couldn't init Table 'users'");

        UserRepositorySQLite { connection: db }
    }
}

impl UserRepository for UserRepositorySQLite {
    fn save(user: User) -> anyhow::Result<()> {
        
        
        
        Ok(())
    }
}
