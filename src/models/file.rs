use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct File {
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) file_type: Option<String>,
    pub(super) size: i32,
    pub(super) last_modified: NaiveDateTime,
    pub(super) starred: bool,
    pub(super) fk_owner: i32,
    pub(super) fk_parent: i32,
}
