use shaku::Component;
use sqlite::Error as SQLiteError;
use sqlite::State;

use crate::users::domain::users::user_id::UserID;
use crate::users::domain::users::user_repository::{
    DeleteErrors, ReadErrors, SaveErrors, UpdateErrors, UserRepository,
};
use crate::users::domain::users::User;
use crate::users::infrastructure::sqlite::mappers::get_user;
use crate::users::infrastructure::sqlite::DATABASE_FILE;

impl From<SQLiteError> for SaveErrors {
    fn from(value: SQLiteError) -> Self {
        if let Some(code) = value.code {
            match code {
                19 => SaveErrors::AlreadyExists,
                _ => value.into(),
            }
        } else {
            value.into()
        }
    }
}

macro_rules! anyhow_from {
    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                value.into()
            }
        }
    };
}

anyhow_from!(SQLiteError, ReadErrors);
anyhow_from!(SQLiteError, DeleteErrors);
anyhow_from!(SQLiteError, UpdateErrors);

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositorySQLite {}

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
    fn save(&self, user: &User) -> Result<(), SaveErrors> {
        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(STMT_INSERT)?;

        stmt.bind((1, user.get_id()))?;
        stmt.bind((2, user.get_name()))?;
        stmt.bind((3, user.get_password()))?;
        stmt.bind((4, user.get_email()))?;

        stmt.next()?;

        Ok(())
    }

    fn find_by(&self, id: &UserID) -> Result<Option<User>, ReadErrors> {
        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(STMT_FIND_BY_ID)?;

        stmt.bind((1, id.to_string().as_str()))?;

        let state = stmt.next()?;

        if state == State::Row {
            Ok(Some(get_user(&stmt)))
        } else {
            Ok(None)
        }

    }

    fn get_all(&self) -> Result<Vec<User>, ReadErrors> {
        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(STMT_GET_ALL)?;

        let mut users: Vec<User> = vec![];

        while let Ok(State::Row) = stmt.next() {
            users.push(get_user(&stmt))
        }

        Ok(users)
    }

    fn delete_by(&self, id: &UserID) -> Result<(), DeleteErrors> {
        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(STMT_DELETE)?;

        stmt.bind((1, id.to_string().as_str()))?;

        stmt.next()?;

        Ok(())
    }

    fn update(&self, user: &User) -> Result<(), UpdateErrors> {
        let conn = sqlite::Connection::open(DATABASE_FILE)?;

        let mut stmt = conn.prepare(STMT_UPDATE)?;

        stmt.bind((1, user.get_name()))?;
        stmt.bind((2, user.get_password()))?;
        stmt.bind((3, user.get_email()))?;
        stmt.bind((4, user.get_id()))?;

        stmt.next()?;

        Ok(())
    }
}
