use crate::schema::rooms;
use actix::Message as ActixMessage;
use diesel::Insertable;
use diesel::QueryResult;
use diesel::Queryable;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Room {
    pub id: i32,
    pub type_room: String,
    pub name: String,
    pub users: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    pub id: i32,
    pub body: String,
    pub typeM: String,
    pub datetime: String,
    pub sender: String,
    pub receiver: String,
    pub readed: Vec<i32>,
}

#[derive(Queryable, Serialize, Debug)]
pub struct RoomInitialInformation {
    pub id: i32,
    pub type_room: String,
    pub name: String,
    pub users: Vec<i32>,
    pub last_message: Option<MessageResponse>, // Opcional en caso de que no haya mensajes
    pub unreaded_messages: Option<Vec<MessageResponse>>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct MessagesRoomInformation {
    pub last_message: Option<MessageResponse>,
    pub unreaded_messages: Vec<MessageResponse>,
}

#[derive(Queryable, Serialize, Debug)]
pub struct RoomResponse {
    pub id: i32,
    pub type_room: String,
    pub name: String,
    pub users: Vec<i32>,
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=rooms)]
pub struct NewRoom {
    pub name: String,
    pub type_room: String,
    pub users: String,
}

#[derive(ActixMessage)]
#[rtype(result = "QueryResult<RoomResponse>")]
pub struct CreateRoom {
    pub name: String,
    pub type_room: String,
    pub users: String,
}

#[derive(ActixMessage)]
#[rtype(result = "QueryResult<RoomResponse>")]
pub struct AddUser {
    pub room_id: i32,
    pub user_id: i32,
}

#[derive(ActixMessage)]
#[rtype(result = "QueryResult<RoomResponse>")]
pub struct AddMessage {
    pub room_id: i32,
    pub message_id: i32,
}

#[derive(ActixMessage)]
#[rtype(result = "QueryResult<Vec<RoomResponse>>")]
pub struct GetRooms;

#[derive(ActixMessage)]
#[rtype(result = "QueryResult<Vec<RoomInitialInformation>>")]
pub struct GetUserRooms {
    pub user_id: i32,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct getRoomInformationApiExternal {
    pub room_id: i32,
    pub user_id: i32,
}


impl Room {


    pub fn get_users(&self) -> Vec<i32> {
        serde_json::from_str(&self.users).unwrap_or_else(|_| vec![])
    }



    pub fn set_users(&mut self, users: Vec<i32>) {
        self.users = serde_json::to_string(&users).unwrap();
    }
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        RoomResponse {
            id: room.id,
            type_room: room.type_room,
            name: room.name,
            users: serde_json::from_str(&room.users).unwrap_or_else(|_| vec![]),
        }
    }
}
