use argon2::PasswordHash;
use uuid::Uuid;

pub struct User<'a> {
    uuid: Uuid,
    username: &'a str,
    password_hash: PasswordHash<'a>,
}

impl<'a> User<'a> {
    fn new() {}
}
