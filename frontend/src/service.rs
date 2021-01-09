pub mod requests;

use crate::error::ServiceError;
use requests::{Requests};
use yew::Callback;

use yew::services::fetch::FetchTask;
use crate::models::project::{Project,NewProject};
pub struct ProjectService {
    requests: Requests
}

impl ProjectService {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn all(&mut self, callback: Callback<Result<Vec<Project>,ServiceError>>) -> FetchTask {
        self.requests.get::<Vec<Project>>("api/project/".to_string(),callback)
    }

    pub fn save(&mut self, project: NewProject, callback: Callback<Result<Option<Project>,ServiceError>>) -> FetchTask {
        self.requests.post::<NewProject,Option<Project>>("api/project/".to_string(),project,callback)
    }

}
