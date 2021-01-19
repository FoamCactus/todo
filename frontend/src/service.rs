pub mod requests;

use crate::error::ServiceError;
use requests::Requests;
use yew::Callback;

use crate::models::project::{NewProject, Project};
use crate::models::todo::{NewTodo, Todo};
use yew::services::fetch::FetchTask;

pub trait Service {
    type Data;
    type NewData;
    fn new() -> Self;
    fn all(&mut self, callback: Callback<Result<Vec<Self::Data>, ServiceError>>) -> FetchTask;
    fn save(
        &mut self,
        new: Self::NewData,
        callback: Callback<Result<Option<Self::Data>, ServiceError>>,
    ) -> FetchTask;
}

pub struct ProjectService {
    requests: Requests,
}

impl Service for ProjectService {
    type Data = Project;
    type NewData = NewProject;
    fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    fn all(&mut self, callback: Callback<Result<Vec<Self::Data>, ServiceError>>) -> FetchTask {
        self.requests.get("api/project/".to_string(), callback)
    }
    fn save(
        &mut self,
        new: Self::NewData,
        callback: Callback<Result<Option<Self::Data>, ServiceError>>,
    ) -> FetchTask {
        self.requests
            .post("api/project/".to_string(), new, callback)
    }
}

pub struct TodoService {
    requests: Requests,
}

impl Service for TodoService {
    type Data = Todo;
    type NewData = NewTodo;

    fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }
    fn all(&mut self, callback: Callback<Result<Vec<Self::Data>, ServiceError>>) -> FetchTask {
        self.requests.get("api/todo/".to_string(), callback)
    }

    fn save(
        &mut self,
        new: Self::NewData,
        callback: Callback<Result<Option<Self::Data>, ServiceError>>,
    ) -> FetchTask {
        self.requests.post("api/todo/".to_string(), new, callback)
    }
}

impl TodoService {
    pub fn get_by_project(
        &mut self,
        id: i32,
        callback: Callback<Result<Vec<Todo>, ServiceError>>,
    ) -> FetchTask {
        self.requests
            .get(format!("api/todo/project/{}", id), callback)
    }
    pub fn get_by_parent(
        &mut self,
        id: i32,
        callback: Callback<Result<Vec<Todo>, ServiceError>>,
    ) -> FetchTask {
        self.requests
            .get(format!("api/todo/parent/{}", id), callback)
    }

    pub fn mark_complete(
        &mut self,
        data: Todo,
        callback: Callback<Result<Todo, ServiceError>>,
    ) -> FetchTask {
        self.requests
            .put("api/todo/".to_string(), data, callback)
    }
}
