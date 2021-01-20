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

#[cfg_attr(feature = "backend", derive(Queryable,Identifiable,AsChangeset))]
#[cfg_attr(feature = "backend",table_name="todo")]
#[derive(Serialize,Deserialize,Clone,Debug,Eq,PartialEq)]
pub struct Todo {
    pub id: i32,
    pub project_id:  Option<i32>,
    pub parent_id: Option<i32>,
    pub title: String,
    pub details: Option<String>,
    pub uuid: String,
    pub complete: bool,
    pub inserted_date: Option<i32>
}
#[derive(Serialize,Deserialize,Clone,Debug)]
#[cfg_attr(feature = "backend", derive(Insertable))]
#[cfg_attr(feature = "backend",table_name="todo")]
pub struct NewTodo {
    pub project_id:  Option<i32>,
    pub parent_id: Option<i32>,
    pub title: String,
    pub details: Option<String>,
    pub uuid: Option<String>,
    pub complete: bool,
    pub inserted_date: Option<i32>
}


#[derive(Serialize,Deserialize,Clone,Debug,Copy)]
pub enum TodoID {
    Parent(i32),
    Project(i32),
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct TodoBuilder{
    id: Option<TodoID>,
    inserted_date: Option<i32>,
    title: String,
    details: Option<String>,
    uuid: Option<String>,
    complete: bool
}


impl TodoBuilder {
    pub fn new(title: &str) -> Self {
        info!("building new todo");
        Self {
            id: None,
            inserted_date: None,
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

    pub fn id(&mut self, id: TodoID) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn with_details(&mut self,details: &str) -> &mut Self {
        self.details = Some(String::from(details));
        self
    }

    pub fn with_insert_date(&mut self, date: i32 ) -> &mut Self {
        self.inserted_date = Some(date);
        self
    }

    pub fn build(self) -> Result<NewTodo,MissingFieldError> {
        if let Some(id_val) = self.id {
            let new_todo = match id_val {
                TodoID::Project(id) => {
                    NewTodo{
                        project_id: Some(id),
                        parent_id: None,
                        title: self.title,
                        details: self.details,
                        uuid: self.uuid,
                        complete: self.complete,
                        inserted_date: self.inserted_date
                    }
                },
                TodoID::Parent(id) =>  {
                    NewTodo{
                        project_id: None,
                        parent_id: Some(id),
                        title: self.title,
                        details: self.details,
                        uuid: self.uuid,
                        complete: self.complete,
                        inserted_date: self.inserted_date
                    }
                }
            };
            Ok(new_todo)
        }else {
            Err(MissingFieldError::MissingID)
        }
    }
}

pub enum MissingFieldError {
    MissingID,
    MissingTitle
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
    todo.filter(project_id.eq(p_id).and(complete.eq(false))).load(con)
}

#[cfg(feature= "backend")]
pub fn get_by_parent(con: &SqliteConnection, p_id: i32) -> Result<Vec<Todo>, Error> {
    use crate::schema::todo::dsl::*;
    todo.filter(parent_id.eq(p_id).and(complete.eq(false))).load(con)
}

#[cfg(feature= "backend")]
pub fn update_todo(con: &SqliteConnection, data: &Todo) -> Result<(),diesel::result::Error> {
    diesel::update(data).set(data).execute(con)?;
    Ok(())
}

