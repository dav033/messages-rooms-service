mod actors;
mod db_utils;
mod models;
mod schema;
mod services;
use actix::SyncArbiter;
use actix_cors::Cors;
use actix_web::web;
use actix_web::{App, HttpServer, http};
use db_utils::{get_pool, AppState, DbActor};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};
use dotenv::dotenv;
use services::{add_message, add_user, create_room, get_rooms, get_user_rooms};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("HOLA");
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<MysqlConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://localhost:8082")
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(AppState {
                db: db_addr.clone(),
            }))
            .wrap(cors)
            .service(create_room)
            .service(add_message)
            .service(get_rooms)
            .service(add_user)
            .service(get_user_rooms)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
