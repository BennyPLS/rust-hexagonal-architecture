use crate::users::domain::user_repository::UserRepository;
use crate::users::domain::users::User;
use shaku::Component;
use sqlite::ConnectionThreadSafe;

#[derive(Component)]
#[shaku(interface = UserRepository<Error = sqlite::Error>)]
pub struct UserRepositorySQLite {
    connection: ConnectionThreadSafe,
}

const STMT_INSERT: &str = "INSERT INTO users (id, name, password, email) VALUES (?, ?, ?, ?)";

impl UserRepository for UserRepositorySQLite {
    type Error = sqlite::Error;

    fn save(&self, user: User) -> Result<(), Self::Error> {
        let mut stmt = self.connection.prepare(STMT_INSERT)?;

        stmt.bind((1, user.get_id()))?;
        stmt.bind((2, user.get_name()))?;
        stmt.bind((3, user.get_password()))?;
        stmt.bind((4, user.get_email()))?;

        Ok(())
    }
}
