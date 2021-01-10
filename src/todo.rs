use crate::models::todo::{NewTodo, Todo};
use crate::DbPool;
use actix_web::web;
use actix_web::web::{HttpResponse, ServiceConfig};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use log::info;

pub fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(get);
    cfg.service(save);
    cfg.service(project);
}

#[actix_web::get("/")]
async fn get(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    info!("getting all todos");
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || get_all(&connection)).await {
        Ok(d) => Ok(HttpResponse::Ok().json(d)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

#[actix_web::get("/project/{id}")]
async fn project(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("getting all todos with project_id = {}", id);
    let connection = pool.get().expect("couldn't gets db connection'");
    match web::block(move || get_by_project(&connection, *id)).await {
        Ok(vec) => Ok(HttpResponse::Ok().json(vec)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

#[actix_web::post("/")]
async fn save(
    pool: web::Data<DbPool>,
    td: web::Json<NewTodo>,
) -> Result<HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || new(td.clone(), &connection)).await {
        Ok(t) => Ok(HttpResponse::Ok().json(t)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

fn get_all(con: &SqliteConnection) -> Result<Vec<Todo>, Error> {
    use crate::models::schema::todo::dsl::*;
    todo.load::<Todo>(con)
}

fn new(td: NewTodo, con: &SqliteConnection) -> Result<Option<Todo>, Error> {
    use crate::models::schema::todo::dsl::*;
    diesel::insert_into(todo).values(&td).execute(con)?;
    info!("saving todo: {:?}",td);
    if let Some(s) = td.uuid {
        todo.filter(uuid.eq(s)).first::<Todo>(con).optional()
    }else {
        Ok(None)
    }
}

fn get_by_project(con: &SqliteConnection, p_id: i32) -> Result<Vec<Todo>, Error> {
    use crate::models::schema::todo::dsl::*;
    todo.filter(project_id.eq(p_id)).load(con)
}
