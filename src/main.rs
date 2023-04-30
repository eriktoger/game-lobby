#[macro_use]
extern crate diesel;
use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
mod db;
mod models;
mod routes;
mod schema;
mod server;
mod session;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn_spec = "chat.db";
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .max_size(1) // https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
        .build(manager)
        .expect("Failed to create pool.");
    let server_addr = "0.0.0.0";
    let server_port = 8000;
    let server = server::ChatServer::new(pool.clone()).start();
    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_origin("https://game-lobby-ttc.netlify.app")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(web::resource("/").to(routes::index))
            .route("/ws", web::get().to(routes::chat_server))
            .service(routes::create_user)
            .service(routes::get_user_by_id)
            .service(routes::get_user_by_phone)
            .service(routes::get_data_from_room)
            .service(routes::get_rooms)
            .service(routes::join_room)
            .service(routes::update_user_session)
            .service(Files::new("/", "./static"))
    })
    .workers(2)
    .bind((server_addr, server_port))?
    .run();
    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}
