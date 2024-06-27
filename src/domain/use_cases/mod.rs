use crate::domain::entities::User;

pub fn create_user(id: u32, name: String) -> User {
    User { id, name }
}

pub fn get_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User {
            id,
            name: "Toto".to_string(),
        })
    } else {
        None
    }
}
