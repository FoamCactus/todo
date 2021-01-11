extern crate diesel;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use log::info;
use std::env;
//use actix_files::Files;
use models;
mod project;
mod todo;

type Connection = SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<Connection>>;
type DieselError = diesel::result::Error;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let mananger = ConnectionManager::<Connection>::new(database_url());
    info!("Building pool");
    let pool = r2d2::Pool::builder()
        .build(mananger)
        .expect("pool was not built");
    info!("built pool");

    info!("Starting Server at port 8088");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(
                web::scope("/api")
                    .service(web::scope("/project").configure(project::scoped_config))
                    .service(web::scope("/todo").configure(todo::scoped_config)),
            )
            .service(Files::new("/node_modules", "./frontend/node_modules/"))
            .service(Files::new("/", "./frontend/static/").index_file("index.html"))
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .await
}

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
