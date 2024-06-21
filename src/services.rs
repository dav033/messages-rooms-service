use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::{
    models::{AddMessage, AddUser, CreateRoom, GetRooms, GetUserRooms},
    AppState, DbActor,
};
use actix::Addr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRoomBody {
    pub name: String,
    pub type_room: String,
    pub users: String,
}

#[derive(Deserialize)]
pub struct AddMessageBody {
    pub message_id: i32,
}

#[post("/create_room")]
pub async fn create_room(state: Data<AppState>, body: Json<CreateRoomBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    print!("AAAAAA");
    match db
        .send(CreateRoom {
            name: body.name.clone(),
            type_room: body.type_room.clone(),
            users: body.users.clone(),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Err(err) => {
            println!("Error sending message to DbActor: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
        _ => HttpResponse::MethodNotAllowed().json("Failed to create room"),
    }
}

#[put("/create_message/{room_id}")]
pub async fn add_message(
    state: Data<AppState>,
    body: Json<AddMessageBody>,
    room_id: Path<i32>,
) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(AddMessage {
            room_id: room_id.into_inner(),
            message_id: body.message_id,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create message"),
    }
}

#[get("/rooms")]
pub async fn get_rooms(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetRooms).await {
        Ok(Ok(rooms)) => HttpResponse::Ok().json(rooms),
        Ok(Err(_)) => HttpResponse::InternalServerError().json("Error loading rooms"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to communicate with DB actor"),
    }
}

#[put("/add_user/{room_id}/{user_id}")]
pub async fn add_user(state: Data<AppState>, path: Path<(i32, i32)>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let (room_id, user_id) = path.into_inner();

    match db.send(AddUser { room_id, user_id }).await {
        Ok(Ok(room)) => HttpResponse::Ok().json(room),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to add user")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}

#[get("/user_rooms/{user_id}")]
pub async fn get_user_rooms(state: Data<AppState>, user_id: Path<i32>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let user_id = user_id.into_inner();

    match db.send(GetUserRooms { user_id }).await {
        Ok(Ok(rooms)) => HttpResponse::Ok().json(rooms),
        Ok(Err(e)) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to get user rooms")
        }
        Err(e) => {
            eprintln!("Mailbox error: {}", e);
            HttpResponse::InternalServerError().json("Failed to send message to database")
        }
    }
}
