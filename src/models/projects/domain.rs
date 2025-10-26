use chrono::NaiveDateTime;

use crate::{models::robot::domain::ProjectTestRunSummary, utils};

use super::api::ProjectResponse;

pub struct NewProject {
    pub name: String,
}

pub struct SavedProject {
    pub id: i32,
    pub name: String,
    pub create_date: NaiveDateTime,
}

impl SavedProject {
    pub fn to_project_response(
        &self,
        test_runs_summaries: Vec<ProjectTestRunSummary>,
    ) -> ProjectResponse {
        ProjectResponse {
            id: self.id,
            name: self.name.clone(),
            create_date: utils::date::format_datetime(self.create_date),
            test_run_count: test_runs_summaries.len() as i32,
            test_runs_summaries: test_runs_summaries.iter().map(|tr| tr.to_api()).collect(),
        }
    }
}
