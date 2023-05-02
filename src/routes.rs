use crate::db;
use crate::models;
use crate::models::RoomData;
use crate::server;
use crate::session;
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use std::time::Instant;
use uuid::Uuid;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

pub async fn chat_server(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<server::ChatServer>>,
) -> HttpResponse {
    match ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_string(),
            name: None,
            addr: srv.get_ref().clone(),
            db_pool: pool,
        },
        &req,
        stream,
    ) {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_user_wrapper(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username, &form.phone)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> HttpResponse {
    match create_user_wrapper(pool, form).await {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_uid(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone: {id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

pub async fn get_data_from_room_wrapper(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = uid.to_owned();
    let (messages, users, games) = web::block(move || {
        let mut conn = pool.get().unwrap();
        //this one needs to have Usernames in them (To display them correctly)
        //content and username is enough
        // I really need to learn how to join
        let m = db::get_display_messages_by_room_uid(&mut conn, room_id).unwrap();

        let u = db::get_users_in_room(&mut conn, room_id.to_string()).unwrap();

        let g = db::get_games_in_room(&mut conn, room_id.to_string()).unwrap();

        (m, u, g)
    })
    .await?;
    let room_response = RoomData {
        users,
        messages,
        games,
    };
    Ok(HttpResponse::Ok().json(room_response))
}

#[get("/rooms/{uid}/data")]
pub async fn get_data_from_room(pool: web::Data<DbPool>, uid: web::Path<Uuid>) -> HttpResponse {
    match get_data_from_room_wrapper(pool, uid).await {
        Ok(response) => response,
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_user_by_phone_wrapper(
    pool: web::Data<DbPool>,
    phone: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_phone = phone.to_string();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_phone(&mut conn, user_phone)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone: {}", phone.to_string())
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/users/phone/{user_phone}")]
pub async fn get_user_by_phone(pool: web::Data<DbPool>, phone: web::Path<String>) -> HttpResponse {
    match get_user_by_phone_wrapper(pool, phone).await {
        Ok(ok) => ok,
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/users/{user_id}/session/{session_id}")]
pub async fn update_user_session(
    pool: web::Data<DbPool>,
    params: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let user_id = params.0.to_string();
    let session_id = params.1.to_string();

    web::block(move || {
        let mut conn = pool.get()?;
        db::update_user_session(&mut conn, user_id, session_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().into())
}

#[get("/rooms")]
pub async fn get_rooms(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let rooms = web::block(move || {
        let mut conn = pool.get()?;
        db::get_rooms(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if !rooms.is_empty() {
        Ok(HttpResponse::Ok().json(rooms))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No rooms available at the moment.",
            })
            .to_string(),
        );
        Ok(res)
    }
}

//this sould be a web-socket thing instead
#[post("/rooms/join")]
pub async fn join_room(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewRoomToUser>,
) -> Result<HttpResponse, Error> {
    let joined = web::block(move || {
        let mut conn = pool.get()?;
        db::join_room(&mut conn, &form.room, &form.user)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(joined))
}
