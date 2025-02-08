use chrono::NaiveDateTime;

use super::domain::SavedProject;

#[derive(sqlx::FromRow)]
pub struct ProjectDB {
    pub id: i32,
    pub name: String,
    pub create_date: NaiveDateTime,
}

impl ProjectDB {
    pub fn into_saved(self) -> SavedProject {
        SavedProject {
            id: self.id,
            name: self.name,
            create_date: self.create_date,
        }
    }
}
