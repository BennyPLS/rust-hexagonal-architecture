use crate::users::domain::users::user_email::UserEmail;
use crate::users::domain::users::user_id::UserID;
use crate::users::domain::users::user_name::UserName;
use crate::users::domain::users::user_password::UserPassword;
use crate::users::domain::users::User;
use sqlite::Statement;

pub fn get_user(statement: &Statement) -> User<'static> {
    User::new(
        UserID::try_from(
            statement
                .read::<String, _>(0)
                .expect("Expected String User ID"),
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
                .expect("Expected String User Password"),
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
