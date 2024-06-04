#[derive(Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(value: String) -> UserEmail {
        UserEmail(value)
    }
}