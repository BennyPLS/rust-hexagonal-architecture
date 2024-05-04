use crate::domain::users;
use crate::domain::users::User;
use uuid::Uuid;

trait UserRegister {
    fn register(uuid: String, name: String, password: String, email: String);
}

struct UserRegisterService;

impl UserRegister for UserRegisterService {
    fn register(uuid: String, name: String, password: String, email: String) {
        let user = User::create(uuid, name, password, email);
    }
}
