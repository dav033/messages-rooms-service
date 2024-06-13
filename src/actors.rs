use crate::db_utils::DbActor;
use crate::models::{
    AddMessage, AddUser, CreateRoom, GetRooms, GetUserRooms, NewRoom, Room, RoomResponse,
};
use crate::schema::rooms;
use crate::schema::rooms::dsl::*;
use crate::schema::rooms::id as room_id;
use crate::schema::rooms::{messages, users};
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

        let mut db_messages = room.get_messages();
        db_messages.push(msg.message_id);
        room.set_messages(db_messages);

        // Actualiza la sala en la base de datos
        let _ = diesel::update(rooms.find(msg.room_id))
            .set(messages.eq(&serde_json::to_string(&room.get_messages()).unwrap()))
            .execute(&mut conn);

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
    type Result = QueryResult<Vec<RoomResponse>>;

    fn handle(
        &mut self,
        msg: GetUserRooms,
        _: &mut Self::Context,
    ) -> QueryResult<Vec<RoomResponse>> {
        let mut conn = self
            .0
            .get()
            .expect("Create Room: Error connecting to database");

        
        let rooms_result: Result<Vec<Room>, diesel::result::Error> = rooms
            .filter(users.like(format!("%{}%", msg.user_id)))
            .load::<Room>(&mut conn);

        rooms_result.map(|rooms_result| rooms_result.into_iter().map(RoomResponse::from).collect())
    }
}
