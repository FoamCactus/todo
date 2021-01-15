use crate::models::todo::{NewTodo, Todo,get_all,update_todo,get_by_project,get_by_parent,new};
use crate::DbPool;
use actix_web::web;
use actix_web::web::{HttpResponse, ServiceConfig};
use log::info;

pub fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(get);
    cfg.service(save);
    cfg.service(project);
    cfg.service(parent);
    cfg.service(update);
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

#[actix_web::put("/")]
async fn update(pool: web::Data<DbPool>, data: web::Json<Todo>) -> Result<HttpResponse,actix_web::Error> {
    let connection = pool.get().expect("couldn't gets db connection'");
    match web::block( move || update_todo(&connection,&data)).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(HttpResponse::NotModified().json(e.to_string()))
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

#[actix_web::get("/parent/{id}")]
async fn parent(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("getting all todos with parent_id = {}", id);
    let connection = pool.get().expect("couldn't gets db connection'");
    match web::block(move || get_by_parent(&connection, *id)).await {
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

