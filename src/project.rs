use crate::models::project::{Project,NewProject};
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use diesel::prelude::*;
use log::{info};
use actix_web::web;
use actix_web::web::{HttpResponse,ServiceConfig};
use crate::DbPool;

pub fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(get);
    cfg.service(save);
}

#[actix_web::get("/")]
async fn get(pool: web::Data::<DbPool> ) -> Result<HttpResponse,actix_web::Error> {
    info!("getting all projects");
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || get_all(&connection)).await {
        Ok(d) => Ok(HttpResponse::Ok().json(d)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

#[actix_web::post("/")]
async fn save(pool: web::Data::<DbPool>, proj: web::Json<NewProject> ) -> Result<HttpResponse,actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || new(proj.clone(),&connection)).await {
        Ok(proj) => Ok(HttpResponse::Ok().json(proj)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

fn get_all(con: &SqliteConnection) -> Result<Vec<Project>,Error> {
    use crate::models::schema::project::dsl::*;
    project.load::<Project>(con)
}

fn new(proj: NewProject, con: &SqliteConnection) -> Result<Option<Project>,Error> {
    use crate::models::schema::project::dsl::*;
    diesel::insert_into(project).values(&proj).execute(con)?;
    if let Some(s) = proj.uuid {
        project.filter(uuid.eq(s)).first::<Project>(con).optional()
    }else {
        Ok(None)
    }
}

