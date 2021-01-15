#[cfg(feature = "backend")]
use diesel::{Queryable,Insertable};
#[cfg(feature = "backend")]
use diesel::sqlite::SqliteConnection;
#[cfg(feature = "backend")]
use diesel::result::Error;
#[cfg(feature = "backend")]
use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use uuid::Uuid;
#[cfg(feature = "backend")]
use crate::schema::*;

#[derive(Serialize,Deserialize,Clone,Debug)]
#[cfg_attr(feature = "backend", derive(Queryable))]
pub struct Project{
    pub id: i32,
    pub title: String,
    pub uuid: String,
}



#[derive(Serialize,Deserialize,Clone,Debug)]
#[cfg_attr(feature = "backend", derive(Insertable))]
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


#[cfg(feature= "backend")]
pub fn get_all(con: &SqliteConnection) -> Result<Vec<Project>, Error> {
    use crate::schema::project::dsl::*;
    project.load::<Project>(con)
}

#[cfg(feature= "backend")]
pub fn save(proj: NewProject, con: &SqliteConnection) -> Result<Option<Project>, Error> {
    use crate::schema::project::dsl::*;
    diesel::insert_into(project).values(&proj).execute(con)?;
    if let Some(s) = proj.uuid {
        project.filter(uuid.eq(s)).first::<Project>(con).optional()
    } else {
        Ok(None)
    }
}
