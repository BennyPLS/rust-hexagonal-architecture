#[derive(Debug)]
pub struct UserID(String);

impl UserID {
    fn new(value: String) -> UserID {
        UserID(value)
    }
}
#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    fn new(value: String) -> UserName {
        UserName(value)
    }
}

#[derive(Debug)]
pub struct UserPassword(String);

impl UserPassword {
    fn new(value: String) -> UserPassword {
        UserPassword(value)
    }
}

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    fn new(value: String) -> UserEmail {
        UserEmail(value)
    }
}

#[derive(Debug)]
pub struct User {
    id: UserID,
    name: UserName,
    password: UserPassword,
    email: UserEmail,
}

impl User {
    pub fn create(id: String, name: String, password: String, email: String) -> User {
        User {
            id: UserID::new(id),
            name: UserName::new(name),
            password: UserPassword::new(password),
            email: UserEmail::new(email),
        }
    }
}
