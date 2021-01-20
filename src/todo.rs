use crate::models::todo::{TodoBuilder, Todo,get_all,update_todo,get_by_project,get_by_parent,new};
use std::convert::TryFrom;
use crate::DbPool;
use actix_web::web;
use actix_web::web::{HttpResponse, ServiceConfig};
use log::info;
use chrono::prelude::*;

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
    td: web::Json<TodoBuilder>,
) -> Result<HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection'");
    let mut todo = td.to_owned();
    let date = match i32::try_from(Utc::now().timestamp()) {
        Ok(val) => val,
        Err(_) => i32::MAX
    };
    todo.with_insert_date(date);
    let new_todo = todo.build();
    match new_todo {
        Ok(todo) => {
            match web::block(move || new(todo, &connection)).await {
                Ok(t) => Ok(HttpResponse::Ok().json(t)),
                Err(e) => Err(actix_web::Error::from(e)),
            }
        },
        Err(_) =>  Err(actix_web::Error::from(()))
    }

}

