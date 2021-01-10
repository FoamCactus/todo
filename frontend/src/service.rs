pub mod requests;

use crate::error::ServiceError;
use requests::{Requests};
use yew::Callback;

use yew::services::fetch::FetchTask;
use crate::models::project::{Project,NewProject};
use crate::models::todo::{Todo,NewTodo};

pub trait Service {
    type Data;
    type NewData;
    fn new() -> Self;
    fn all(&mut self, callback: Callback<Result<Vec<Self::Data>,ServiceError>>) -> FetchTask;
    fn save(&mut self,new: Self::NewData, callback: Callback<Result<Option<Self::Data>,ServiceError>>) -> FetchTask;
    
}

pub struct ProjectService {
    requests: Requests
}

impl Service for ProjectService {
    type Data = Project;
    type NewData = NewProject;
     fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

     fn all(&mut self, callback: Callback<Result<Vec<Self::Data>,ServiceError>>) -> FetchTask {
        self.requests.get("api/project/".to_string(),callback)
    }
     fn save(&mut self, new: Self::NewData, callback: Callback<Result<Option<Self::Data>,ServiceError>>) -> FetchTask {
         
        self.requests.post("api/project/".to_string(),new,callback)
     }


}

pub struct TodoService {
    requests: Requests
}

impl Service for TodoService {
    type Data = Todo;
    type NewData = NewTodo;

    fn new() -> Self {
        Self {
            requests: Requests::new()
        }
    }
    fn all(&mut self, callback: Callback<Result<Vec<Self::Data>,ServiceError>>) -> FetchTask {
        self.requests.get("api/todo".to_string(),callback)
    }

    fn save(&mut self, new: Self::NewData, callback: Callback<Result<Option<Self::Data>,ServiceError>>) -> FetchTask {
       self.requests.post("api/todo".to_string(),new,callback) 
    }

    

}

