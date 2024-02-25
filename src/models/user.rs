#[derive(sqlx::FromRow)]
pub struct User {
    pub(super) id: i32,
    pub(super) username: String,
    pub(super) email: String,
    pub(super) password: String,
}

impl User {
    pub fn new(id: i32, username: String, email: String, password: String) -> Self {
        User {
            id,
            username,
            email,
            password,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    fn get_password(&self) -> String {
        self.password.clone()
    }
}
