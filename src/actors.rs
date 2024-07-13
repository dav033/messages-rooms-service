use crate::db_utils::DbActor;
use crate::models::*;
use crate::schema::rooms;
use crate::schema::rooms::dsl::*;
use crate::schema::rooms::id as room_id;
use crate::schema::rooms::{users};
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<CreateRoom> for DbActor {
    type Result = QueryResult<RoomResponse>;

    fn handle(&mut self, msg: CreateRoom, _: &mut Self::Context) -> Self::Result {
        let mut conn: diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<MysqlConnection>,
        > = self
            .0
            .get()
            .expect("Create Room: Error connecting to database");

        let new_room = NewRoom {
            name: msg.name,
            type_room: msg.type_room,
            users: msg.users,
        };

        let _ = diesel::insert_into(rooms)
            .values(&new_room)
            .execute(&mut conn);

        let result: Result<Room, diesel::result::Error> =
            rooms.order(room_id.desc()).first(&mut conn);

        result.map(RoomResponse::from)
    }
}

impl Handler<AddMessage> for DbActor {
    type Result = QueryResult<RoomResponse>;

    fn handle(&mut self, msg: AddMessage, _: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Update Room: Error connecting to database");

        let mut room: Room = rooms.find(msg.room_id).get_result(&mut conn)?;

    

        // Devuelve la sala actualizada
        let result: Result<Room, diesel::result::Error> =
            rooms.find(msg.room_id).get_result(&mut conn);

        result.map(RoomResponse::from)
    }
}
// Import the `rooms` table from the schema module

impl Handler<GetRooms> for DbActor {
    type Result = QueryResult<Vec<RoomResponse>>;

    fn handle(&mut self, _msg: GetRooms, _: &mut Self::Context) -> QueryResult<Vec<RoomResponse>> {
        let mut conn = self
            .0
            .get()
            .expect("Create Room: Error connecting to database");

        // Use `rooms::table` instead of `rooms`
        let rooms_result: Result<Vec<Room>, diesel::result::Error> =
            rooms::table.load::<Room>(&mut conn);

        rooms_result.map(|rooms_result| rooms_result.into_iter().map(RoomResponse::from).collect())
    }
}

impl Handler<AddUser> for DbActor {
    type Result = QueryResult<RoomResponse>;

    fn handle(&mut self, msg: AddUser, _: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Add User: Error connecting to database");

        let mut room: Room = rooms.find(msg.room_id).get_result(&mut conn)?;

        let mut db_users = room.get_users();
        db_users.push(msg.user_id);
        room.set_users(db_users);

        // Actualiza la sala en la base de datos
        let _ = diesel::update(rooms.find(msg.room_id))
            .set(users.eq(&serde_json::to_string(&room.get_users()).unwrap()))
            .execute(&mut conn);

        // Devuelve la sala actualizada
        let result: Result<Room, diesel::result::Error> =
            rooms.find(msg.room_id).get_result(&mut conn);

        result.map(RoomResponse::from)
    }
}

impl Handler<GetUserRooms> for DbActor {
    type Result = QueryResult<Vec<RoomInitialInformation>>;

    fn handle(
        &mut self,
        msg: GetUserRooms,
        _: &mut Self::Context,
    ) -> QueryResult<Vec<RoomInitialInformation>> {
        let mut conn = self
            .0
            .get()
            .expect("GetUserRooms: Error connecting to database");

        let rooms_result: Result<Vec<Room>, diesel::result::Error> = rooms
            .filter(users.like(format!("%{}%", msg.user_id)))
            .load::<Room>(&mut conn);

        let client = reqwest::blocking::Client::new();

        rooms_result.and_then(|owo| {
            owo.into_iter()
                .map(|room| {
                    // Construir la URL para la solicitud HTTP
                    let url = format!(
                        "http://localhost:8082/messages/{}/info/{}",
                        room.id, msg.user_id
                    );

                    // Realizar la solicitud HTTP

                    let users_parsed = room.get_users();
                    match client.get(&url).send() {
                        Ok(response) => {
                            if response.status().is_success() {
                                match response.json::<MessagesRoomInformation>() {
                                    Ok(info) => Ok(RoomInitialInformation {
                                        id: room.id,
                                        type_room: room.type_room,
                                        name: room.name,
                                        users: users_parsed.clone(),
                                        last_message: info.last_message,
                                        unreaded_messages: Some(info.unreaded_messages),
                                    }),
                                    Err(_) => Err(diesel::result::Error::NotFound),
                                }
                            } else {
                                Err(diesel::result::Error::NotFound)
                            }
                        }
                        Err(_) => Err(diesel::result::Error::NotFound),
                    }
                })
                .collect()
        })
    }
}
