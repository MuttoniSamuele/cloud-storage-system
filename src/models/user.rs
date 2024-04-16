#[derive(sqlx::FromRow)]
pub struct User {
    pub(super) id: i32,
    pub(super) username: String,
    pub(super) email: String,
    pub(super) password: String,
}

impl User {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    fn get_password(&self) -> &String {
        &self.password
    }
}
