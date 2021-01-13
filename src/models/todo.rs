use diesel::{Queryable,Identifiable,AsChangeset};

use serde::{Serialize,Deserialize};
use uuid::Uuid;
use crate::schema::*;

#[derive(Queryable,Serialize,Deserialize,Clone,Debug,Identifiable,AsChangeset)]
#[table_name="todo"]
pub struct Todo {
    pub id: i32,
    pub project_id:  Option<i32>,
    pub parent_id: Option<i32>,
    pub title: String,
    pub details: Option<String>,
    pub uuid: String,
    pub complete: bool
}
#[derive(Insertable,Serialize,Deserialize,Clone,Debug)]
#[table_name="todo"]
pub struct NewTodo {
    pub project_id:  Option<i32>,
    pub parent_id: Option<i32>,
    pub title: String,
    pub details: Option<String>,
    pub uuid: Option<String>,
    pub complete: bool
}

pub struct TodoBuilder{
    title: String,
    details: Option<String>,
    uuid: Option<String>,
    complete: bool
}


impl TodoBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            details: None,
            uuid: Some(Uuid::new_v4().to_string()),
            complete: false
        }
    }

    pub fn complete(&mut self) -> &mut Self {
        self.complete = true;
        self
    }

    pub fn with_details(&mut self,details: &str) -> &mut Self {
        self.details = Some(String::from(details));
        self
    }

    pub fn project(self, project_id: i32) -> TodoProjectBuilder{
        TodoProjectBuilder {
            project_id,
            title: self.title,
            details: self.details,
            uuid: self.uuid,
            complete: self.complete
        }
    }

    pub fn parent(self,parent_id: i32) -> TodoParentBuilder {
        TodoParentBuilder {
            parent_id,
            title: self.title,
            details: self.details,
            uuid: self.uuid,
            complete: self.complete
        }

    }
}

pub struct TodoProjectBuilder{
    project_id: i32,
    title: String,
    details: Option<String>,
    uuid: Option<String>,
    complete: bool
}

impl TodoProjectBuilder{
    pub fn build(self) -> NewTodo {
        NewTodo{
            project_id: Some(self.project_id),
            parent_id: None,
            title: self.title,
            details: self.details,
            uuid: self.uuid,
            complete: self.complete
        }
    }
}

pub struct TodoParentBuilder{
    parent_id: i32,
    title: String,
    details: Option<String>,
    uuid: Option<String>,
    complete: bool
}
impl TodoParentBuilder{
    pub fn build(self) -> NewTodo {
        NewTodo{
            project_id: None,
            parent_id: Some(self.parent_id),
            title: self.title,
            details: self.details,
            uuid: self.uuid,
            complete: self.complete
        }
    }
}

