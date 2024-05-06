use shaku::{Component, Interface};

use crate::users::domain::users::User;

pub trait UserRegister: Interface {
    fn register(&self, uuid: String, name: String, password: String, email: String);
}

#[derive(Component)]
#[shaku(interface = UserRegister)]
pub struct UserRegisterService;

impl UserRegister for UserRegisterService {
    fn register(&self, uuid: String, name: String, password: String, email: String) {
        let user = User::create(uuid, name, password, email);
        
        dbg!(user);
    }
}
