use shaku::Component;
use sqlite::{ConnectionThreadSafe, Error};

use crate::users::domain::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::User;

impl From<Error> for RepositoryErrors {
    fn from(value: Error) -> Self {
        if let Some(code) = value.code {
            return match code {
                19 => RepositoryErrors::AlreadyExists,
                _ => unmapped_error(value),
            };
        }

        unmapped_error(value)
    }
}

fn unmapped_error(error: sqlite::Error) -> RepositoryErrors {
    dbg!(error);
    RepositoryErrors::InternalServerError
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositorySQLite {
    connection: ConnectionThreadSafe,
}

const STMT_INSERT: &str = "INSERT INTO users (id, name, password, email) VALUES (?, ?, ?, ?)";

impl UserRepository for UserRepositorySQLite {
    fn save(&self, user: User) -> Result<(), RepositoryErrors> {
        let mut stmt = self.connection.prepare(STMT_INSERT)?;

        stmt.bind((1, user.get_id()))?;
        stmt.bind((2, user.get_name()))?;
        stmt.bind((3, user.get_password()))?;
        stmt.bind((4, user.get_email()))?;

        let result = stmt.next()?;

        dbg!(result);

        Ok(())
    }
}
