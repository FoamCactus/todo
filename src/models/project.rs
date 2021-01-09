use diesel::{Queryable,Insertable};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use crate::schema::*;

#[derive(Queryable,Serialize,Deserialize,Clone,Debug)]
pub struct Project{
    pub id: i32,
    pub title: String,
    pub uuid: String,
}



#[derive(Insertable,Serialize,Deserialize,Clone,Debug)]
#[table_name="project"]
pub struct NewProject {
    pub title: String,
    pub uuid: Option<String>
}

impl NewProject {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            uuid: Some(Uuid::new_v4().to_string())
        }
    }

    pub fn set_title(&mut self, title: &str){
        self.title = String::from(title);
    }
}
