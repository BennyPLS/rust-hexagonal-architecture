use crate::users::domain::user_id::UserID;
use shaku::Component;
use sqlite::{ConnectionThreadSafe, Error, State, Statement};

use crate::users::domain::user_repository::{RepositoryErrors, UserRepository};
use crate::users::domain::users::{User, UserEmail, UserName, UserPassword};

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
    dbg!(&error);
    RepositoryErrors::InternalServerError {
        source: anyhow::Error::from(error),
    }
}

fn get_user(statement: &Statement) -> User {
    User::new(
        UserID::try_from(
            statement
                .read::<String, _>(0)
                .expect("Expected String User ID").as_str(),
        ).unwrap(), // TODO: Change Error Management
        UserName::new(
            statement
                .read::<String, _>(1)
                .expect("Expected String User Name"),
        ),
        UserPassword::new(
            statement
                .read::<String, _>(2)
                .expect("Expected String User Password"),
        ),
        UserEmail::new(
            statement
                .read::<String, _>(3)
                .expect("Expected String User Email"),
        ),
    )
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositorySQLite {
    connection: ConnectionThreadSafe,
}
// language=SQL
const STMT_INSERT: &str = "INSERT INTO users (id, name, password, email) VALUES (?, ?, ?, ?)";
// language=SQL
const STMT_FIND_BY_ID: &str = "SELECT * FROM users WHERE id = ?";
// language=SQL
const STMT_GET_ALL: &str = "SELECT * FROM users";
// language=SQL
const STMT_UPDATE: &str = "UPDATE users SET name = ? AND password = ? AND email = ? WHERE id = ?";
// language=SQL
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
        let stmt = self.connection.prepare(STMT_GET_ALL);

        if stmt.is_err() {
            return vec![];
        }

        let mut stmt = stmt.unwrap();

        let mut users: Vec<User> = vec![];
        while let Ok(State::Row) = stmt.next() {
            users.push(get_user(&stmt))
        }

        users
    }

    fn delete_by(&self, id: &str) -> Result<(), RepositoryErrors> {
        let mut stmt = self.connection.prepare(STMT_DELETE)?;

        dbg!(id);

        stmt.bind((1, id))?;

        stmt.next()?;

        Ok(())
    }

    fn update(&self, user: &User) -> Result<(), RepositoryErrors> {
        let mut stmt = self.connection.prepare(STMT_UPDATE)?;

        stmt.bind((1, user.get_name()))?;
        stmt.bind((2, user.get_password()))?;
        stmt.bind((3, user.get_email()))?;
        stmt.bind((4, user.get_id()))?;

        stmt.next()?;

        Ok(())
    }
}
