use shaku::Component;
use sqlite::{ConnectionThreadSafe, Error, State};

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
const STMT_FIND_BY_ID: &str = "SELECT * FROM users WHERE id = ?";
const STMT_FIND_LIKE_NAME: &str = "SELECT * FROM users WHERE name like '%?%'";

impl UserRepository for UserRepositorySQLite {
    fn save(&self, user: &User) -> Result<(), RepositoryErrors> {
        let mut stmt = self.connection.prepare(STMT_INSERT)?;

        stmt.bind((1, user.get_id()))?;
        stmt.bind((2, user.get_name()))?;
        stmt.bind((3, user.get_password()))?;
        stmt.bind((4, user.get_email()))?;

        stmt.next()?;

        Ok(())
    }

    fn find_by(&self, id: &str) -> Option<User> {
        let mut stmt = self.connection.prepare(STMT_FIND_BY_ID).ok()?;

        stmt.bind((1, id)).ok()?;



        None
    }

    fn find_like(&self, name: &str) -> Option<Vec<User>> {
        let mut stmt = self.connection.prepare(STMT_FIND_LIKE_NAME).ok()?;

        stmt.bind((1, name)).ok()?;

        while let Ok(State::Row) = stmt.next() {
            dbg!(stmt.read::<String, _>(0));
        }

        None
    }
}
