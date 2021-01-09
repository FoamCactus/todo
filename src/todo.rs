use crate::models::todo::{Todo,NewTodo};
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
async fn save(pool: web::Data::<DbPool>, td: web::Json<NewTodo> ) -> Result<HttpResponse,actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || new(td.clone(),&connection)).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

fn get_all(con: &SqliteConnection) -> Result<Vec<Todo>,Error> {
    use crate::models::schema::todo::dsl::*;
    todo.load::<Todo>(con)
}

fn new(td: NewTodo, con: &SqliteConnection) -> Result<(),Error> {
    use crate::models::schema::todo::dsl::*;
    diesel::insert_into(todo).values(td).execute(con)?;
    Ok(())
}

