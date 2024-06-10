use std::sync::Arc;

use shaku::Component;
use sqlite::{Error, State, Statement};

use crate::users::domain::users::User;
use crate::users::domain::users::user_email::UserEmail;
use crate::users::domain::users::user_id::UserID;
use crate::users::domain::users::user_name::UserName;
use crate::users::domain::users::user_password::UserPassword;
use crate::users::domain::users::user_repository::{RepositoryErrors, UserRepository};
use crate::users::infrastructure::sqlite::Database;

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
                .expect("Expected String User ID")
                .as_str(),
        )
        .expect("Invalid Database UserID"),
        UserName::try_from(
            statement
                .read::<String, _>(1)
                .expect("Expected String User Name"),
        )
        .expect("Invalid Database UserName"),
        UserPassword::try_from(
            statement
                .read::<String, _>(2)
                .expect("Expected String User Password")
                .as_str(),
        )
        .expect("Invalid Database UserPassword"),
        UserEmail::try_from(
            statement
                .read::<String, _>(3)
                .expect("Expected String User Email"),
        )
        .expect("Invalid Database UserEmail"),
    )
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositorySQLite {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}
// language=SQL
const STMT_INSERT: &str = "INSERT INTO users (id, name, password, email) VALUES (?, ?, ?, ?)";
// language=SQL
const STMT_FIND_BY_ID: &str = "SELECT * FROM users WHERE id = ?";
// language=SQL
const STMT_GET_ALL: &str = "SELECT * FROM users";
// language=SQL
const STMT_UPDATE: &str = "UPDATE users SET name = ?, password = ?, email = ? WHERE id = ?";
// language=SQL
const STMT_DELETE: &str = "DELETE FROM users WHERE id = ?";

impl UserRepository for UserRepositorySQLite {
    fn save(&self, user: &User) -> Result<(), RepositoryErrors> {
        let conn = self.database.get_connection();

        let mut stmt = conn.prepare(STMT_INSERT)?;

        stmt.bind((1, user.get_id().as_str()))?;
        stmt.bind((2, user.get_name()))?;
        stmt.bind((3, user.get_password()))?;
        stmt.bind((4, user.get_email()))?;

        stmt.next()?;

        Ok(())
    }

    fn find_by(&self, id: &str) -> Option<User> {
        let conn = self.database.get_connection();

        let mut stmt = conn.prepare(STMT_FIND_BY_ID).ok()?;

        stmt.bind((1, id)).ok()?;

        if let Ok(State::Row) = stmt.next() {
            Some(get_user(&stmt))
        } else {
            None
        }
    }

    fn get_all(&self) -> Vec<User> {
        let conn = self.database.get_connection();

        let stmt = conn.prepare(STMT_GET_ALL);

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
        let conn = self.database.get_connection();

        let mut stmt = conn.prepare(STMT_DELETE)?;

        dbg!(id);

        stmt.bind((1, id))?;

        stmt.next()?;

        Ok(())
    }

    fn update(&self, user: &User) -> Result<(), RepositoryErrors> {
        let conn = self.database.get_connection();

        let mut stmt = conn.prepare(STMT_UPDATE)?;

        dbg!(&user);
        
        stmt.bind((1, user.get_name()))?;
        stmt.bind((2, user.get_password()))?;
        stmt.bind((3, user.get_email()))?;
        stmt.bind((4, user.get_id().as_str()))?;

        stmt.next()?;

        Ok(())
    }
}
