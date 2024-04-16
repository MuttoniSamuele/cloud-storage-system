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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_last_modified(&self) -> &NaiveDateTime {
        &self.last_modified
    }

    pub fn get_starred(&self) -> bool {
        self.starred
    }

    pub fn get_fk_owner(&self) -> i32 {
        self.fk_owner
    }

    pub fn get_fk_parent(&self) -> &Option<i32> {
        &self.fk_parent
    }
}
