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

impl File {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_file_type(&self) -> &Option<String> {
        &self.file_type
    }

    pub fn get_size(&self) -> i32 {
        self.size
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

    pub fn get_fk_parent(&self) -> i32 {
        self.fk_parent
    }
}
