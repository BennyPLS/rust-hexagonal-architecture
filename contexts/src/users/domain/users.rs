#[derive(Debug)]
pub struct UserID(String);

impl UserID {
    pub fn new(value: String) -> UserID {
        UserID(value)
    }
}
#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(value: String) -> UserName {
        UserName(value)
    }
}

#[derive(Debug)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn new(value: String) -> UserPassword {
        UserPassword(value)
    }
}

#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(value: String) -> UserEmail {
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
    pub fn new(id: UserID, name: UserName, password: UserPassword, email: UserEmail) -> User {
        User {
            id,
            name,
            password,
            email,
        }
    }

    pub fn create(id: String, name: String, password: String, email: String) -> User {
        User {
            id: UserID::new(id),
            name: UserName::new(name),
            password: UserPassword::new(password),
            email: UserEmail::new(email),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id.0
    }

    pub fn get_name(&self) -> &str {
        &self.name.0
    }

    pub fn get_password(&self) -> &str {
        &self.password.0
    }

    pub fn get_email(&self) -> &str {
        &self.email.0
    }
}
