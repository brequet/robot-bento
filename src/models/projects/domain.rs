use chrono::NaiveDateTime;

pub struct NewProject {
    pub name: String,
}

pub struct SavedProject {
    pub id: i32,
    pub name: String,
    pub create_date: NaiveDateTime,
}
