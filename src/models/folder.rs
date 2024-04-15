use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct Folder {
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) last_modified: NaiveDateTime,
    pub(super) starred: bool,
    pub(super) fk_owner: i32,
    pub(super) fk_parent: Option<i32>,
}

impl Folder {
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
