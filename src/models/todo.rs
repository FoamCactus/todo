#[cfg(feature= "backend")]
use diesel::{Queryable,Identifiable,AsChangeset};
#[cfg(feature= "backend")]
use diesel::result::Error;
#[cfg(feature= "backend")]
use diesel::sqlite::SqliteConnection;
#[cfg(feature= "backend")]
use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use log::info;
use uuid::Uuid;
#[cfg(feature= "backend")]
use crate::schema::*;

#[derive(Serialize,Deserialize,Clone,Debug)]
#[cfg_attr(feature = "backend", derive(Queryable,Identifiable,AsChangeset))]
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
#[derive(Serialize,Deserialize,Clone,Debug)]
#[cfg_attr(feature = "backend", derive(Insertable))]
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

#[cfg(feature= "backend")]
pub fn get_all(con: &SqliteConnection) -> Result<Vec<Todo>, Error> {
    use crate::schema::todo::dsl::*;
    todo.load::<Todo>(con)
}

#[cfg(feature= "backend")]
pub fn new(td: NewTodo, con: &SqliteConnection) -> Result<Option<Todo>, Error> {
    use crate::schema::todo::dsl::*;
    diesel::insert_into(todo).values(&td).execute(con)?;
    info!("saving todo: {:?}",td);
    if let Some(s) = td.uuid {
        todo.filter(uuid.eq(s)).first::<Todo>(con).optional()
    }else {
        Ok(None)
    }
}

#[cfg(feature= "backend")]
pub fn get_by_project(con: &SqliteConnection, p_id: i32) -> Result<Vec<Todo>, Error> {
    use crate::schema::todo::dsl::*;
    todo.filter(project_id.eq(p_id)).load(con)
}

#[cfg(feature= "backend")]
pub fn get_by_parent(con: &SqliteConnection, p_id: i32) -> Result<Vec<Todo>, Error> {
    use crate::schema::todo::dsl::*;
    todo.filter(parent_id.eq(p_id)).load(con)
}

#[cfg(feature= "backend")]
pub fn update_todo(con: &SqliteConnection, data: &Todo) -> Result<(),diesel::result::Error> {
    diesel::update(data).set(data).execute(con)?;
    Ok(())
}

