use crate::models::project::{NewProject,save,get_all};
use crate::DbPool;
use actix_web::web;
use actix_web::web::{HttpResponse, ServiceConfig};
use log::info;

pub fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(get);
    cfg.service(save_project);
}

#[actix_web::get("/")]
async fn get(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    info!("getting all projects");
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || get_all(&connection)).await {
        Ok(d) => Ok(HttpResponse::Ok().json(d)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

#[actix_web::post("/")]
async fn save_project(
    pool: web::Data<DbPool>,
    proj: web::Json<NewProject>,
) -> Result<HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("couldn't get db connection'");
    match web::block(move || save(proj.clone(), &connection)).await {
        Ok(proj) => Ok(HttpResponse::Ok().json(proj)),
        Err(e) => Err(actix_web::Error::from(e)),
    }
}

