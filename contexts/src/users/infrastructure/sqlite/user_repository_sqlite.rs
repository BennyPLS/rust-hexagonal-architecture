use crate::users::domain::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::{User, UserEmail, UserID, UserName, UserPassword};
use shaku::Component;
use sqlite::{ConnectionThreadSafe, Error, State, Statement};

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

fn unmapped_error(error: Error) -> RepositoryErrors {
    dbg!(error);
    RepositoryErrors::InternalServerError
}

fn get_user(statement: &Statement) -> User {
    User::new(
        UserID::new(statement.read::<String, _>("id").unwrap()),
        UserName::new(statement.read::<String, _>("name").unwrap()),
        UserPassword::new(statement.read::<String, _>("password").unwrap()),
        UserEmail::new(statement.read::<String, _>("email").unwrap()),
    )
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositorySQLite {
    connection: ConnectionThreadSafe,
}

const STMT_INSERT: &str = "INSERT INTO users (id, name, password, email) VALUES (?, ?, ?, ?)";
const STMT_FIND_BY_ID: &str = "SELECT * FROM users WHERE id = ?";
const STMT_GET_ALL: &str = "SELECT * FROM users";
const STMT_UPDATE: &str = "UPDATE users SET name = ? AND password = ? AND email = ? WHERE id = ?";
const STMT_DELETE: &str = "DELETE FROM users WHERE id = ?";

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

        if let Ok(State::Row) = stmt.next() {
            Some(get_user(&stmt))
        } else {
            None
        }
    }

    fn get_all(&self) -> Vec<User> {
        let mut stmt = self.connection.prepare(STMT_GET_ALL).unwrap();

        let mut users: Vec<User> = vec![];
        while let Ok(State::Row) = stmt.next() {
            users.push(get_user(&stmt))
        }

        users
    }
}
