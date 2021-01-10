use diesel::Queryable;
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use crate::schema::*;

#[derive(Queryable,Serialize,Deserialize,Clone,Debug)]
pub struct Todo {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub details: Option<String>,
    pub uuid: String
}
#[derive(Insertable,Serialize,Deserialize,Clone,Debug)]
#[table_name="todo"]
pub struct NewTodo {
    pub project_id: i32,
    pub title: String,
    pub details: Option<String>,
    pub uuid: Option<String>,
}

pub struct TodoBuilder{
    project_id: i32,
    title: String,
    details: Option<String>,
    uuid: Option<String>

}


impl TodoBuilder {
    pub fn new(project_id: i32,title: &str) -> Self {
        Self {
            project_id,
            title: String::from(title),
            details: None,
            uuid: Some(Uuid::new_v4().to_string())
        }
    }

    pub fn with_details(&mut self,details: &str) -> &mut Self {
        self.details = Some(String::from(details));
        self
    }

    pub fn build(&self) -> NewTodo {
        NewTodo {
            project_id:self.project_id,
            title: self.title.clone(),
            details: self.details.clone(),
            uuid: self.uuid.clone()
        }
    }
}
