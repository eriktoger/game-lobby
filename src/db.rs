use crate::models::{DisplayMessage, Message, Room, RoomResponse, RoomToUser, User};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::{collections::HashMap, time::SystemTime};
use uuid::Uuid;
type DbError = Box<dyn std::error::Error + Send + Sync>;

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

pub fn find_user_by_ws(conn: &mut SqliteConnection, ws_id: String) -> User {
    use crate::schema::users::dsl::*;
    print!("{}", ws_id);
    users
        .filter(web_socket_session.eq(ws_id.to_string()))
        .first::<User>(conn)
        .unwrap()
}

pub fn find_user_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn update_user_session(
    conn: &mut SqliteConnection,
    user_id: String,
    session_id: String,
) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let result = diesel::update(users)
        .filter(id.eq(user_id.to_string()))
        .set(web_socket_session.eq(session_id))
        .execute(conn)?;

    Ok(result)
}

pub fn find_user_by_phone(
    conn: &mut SqliteConnection,
    user_phone: String,
) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(phone.eq(user_phone))
        .first::<User>(conn)
        .optional()?;
    Ok(user)
}

pub fn get_display_messages_by_room_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Vec<DisplayMessage>, DbError> {
    use crate::schema::messages;
    use crate::schema::messages::dsl::{content, room_id, user_id};
    use crate::schema::users;
    use crate::schema::users::dsl::{id, username};

    let join1: Vec<(String, String)> = messages::table
        .filter(room_id.eq(uid.to_string()))
        .inner_join(users::table.on(id.eq(user_id)))
        .select((username, content))
        .load(conn)?;
    let display_messages: Vec<DisplayMessage> = join1
        .iter()
        .map(|(u, c)| DisplayMessage {
            username: u.to_string(),
            content: c.to_string(),
        })
        .collect();

    Ok(display_messages)
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str, pn: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        phone: pn.to_owned(),
        web_socket_session: "0".to_string(),
        created_at: iso_date(),
    };
    // let error = diesel::insert_into(users).values(&new_user).execute(conn);
    // match error {
    //     Ok(v) => println!("working with version: {v:?}"),
    //     Err(e) => println!("error parsing header: {e:?}"),
    // }
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    Ok(new_user)
}

pub fn insert_new_message(
    conn: &mut SqliteConnection,
    ws_id: String,
    message: String,
) -> Result<Message, DbError> {
    use crate::schema::messages::dsl::*;
    use crate::schema::room_to_users::dsl::*;

    let current_user = find_user_by_ws(conn, ws_id);

    let current_room = room_to_users
        .filter(user.eq(current_user.id.to_string()))
        .first::<RoomToUser>(conn)
        .optional()?
        .unwrap();

    let new_message = Message {
        id: Uuid::new_v4().to_string(),
        room_id: current_room.room,
        user_id: current_user.id,
        content: message,
        created_at: iso_date(),
    };
    diesel::insert_into(messages)
        .values(&new_message)
        .execute(conn)?;
    Ok(new_message)
}

pub fn get_rooms(conn: &mut SqliteConnection) -> Result<Vec<Room>, DbError> {
    use crate::schema::rooms;

    let rooms_data = rooms::table.get_results(conn)?;
    Ok(rooms_data)
}

pub fn get_rooms_with_users(conn: &mut SqliteConnection) -> Result<Vec<RoomResponse>, DbError> {
    use crate::schema::room_to_users;
    use crate::schema::rooms;
    use crate::schema::users;

    let users_data: Vec<User> = users::table.get_results(conn)?;
    let rooms_data: Vec<Room> = rooms::table.get_results(conn)?;

    let rooms_to_user_data: Vec<RoomToUser> = room_to_users::table.get_results(conn)?;
    let mut response_rooms_map: HashMap<Room, Vec<User>> = HashMap::new();

    for room in &rooms_data {
        response_rooms_map.insert((*room).clone(), vec![]);
    }

    for data in rooms_to_user_data {
        let user = users_data.iter().find(|u| u.id == data.user);
        let room = rooms_data.iter().find(|r| r.id == data.room);
        if user.is_none() || room.is_none() {
            continue;
        }
        let real_user = user.unwrap().clone();
        let real_room = room.unwrap().clone();
        if response_rooms_map.contains_key(&real_room) {
            response_rooms_map
                .get_mut(&real_room)
                .unwrap()
                .push(real_user);
        } else {
            response_rooms_map.insert(real_room, vec![real_user]);
        }
    }
    let response_rooms2: Vec<(Room, Vec<User>)> = response_rooms_map.into_iter().collect();
    let response_room = response_rooms2
        .iter()
        .map(|(room, users)| RoomResponse {
            room: (*room).clone(),
            users: (*users).clone(),
        })
        .collect();
    Ok(response_room)
}

pub fn get_current_room_with_users(
    conn: &mut SqliteConnection,
    ws_id: String,
) -> Result<RoomResponse, DbError> {
    use crate::schema::room_to_users::dsl::*;
    use crate::schema::users;

    let users_data: Vec<User> = users::table.get_results(conn)?;
    let current_room = get_current_room(conn, ws_id).unwrap();

    let rooms_to_user_data: Vec<RoomToUser> = room_to_users
        .filter(room.eq(current_room.id.clone()))
        .get_results(conn)?;

    let mut room_response = RoomResponse {
        room: current_room,
        users: vec![],
    };

    for r in &rooms_to_user_data {
        let current_user = users_data.iter().find(|u| u.id == r.user).unwrap();
        room_response.users.push((*current_user).clone());
    }
    Ok(room_response)
}

pub fn get_users_in_room(
    conn: &mut SqliteConnection,
    room_id: String,
) -> Result<Vec<User>, DbError> {
    use crate::schema::room_to_users::dsl::*;
    use crate::schema::users;

    let users_data: Vec<User> = users::table.get_results(conn)?;

    let rooms_to_user_data: Vec<RoomToUser> = room_to_users
        .filter(room.eq(room_id.clone()))
        .get_results(conn)?;

    let mut current_users = vec![];

    for r in &rooms_to_user_data {
        let current_user = users_data.iter().find(|u| u.id == r.user).unwrap();
        current_users.push((*current_user).clone());
    }
    Ok(current_users)
}

pub fn join_room(
    conn: &mut SqliteConnection,
    room_id: &str,
    user_id: &str,
) -> Result<RoomToUser, DbError> {
    use crate::schema::room_to_users;
    use crate::schema::room_to_users::dsl::*;
    let rooms_data: Vec<RoomToUser> = room_to_users::table.get_results(conn)?;
    let exists = rooms_data
        .iter()
        .find(|x| x.room == room_id && x.user == user_id);

    let new_room_to_user = RoomToUser {
        id: Uuid::new_v4().to_string(),
        room: room_id.to_string(),
        user: user_id.to_string(),
    };
    match exists {
        Some(_) => println!("{:?}", exists),
        None => {
            diesel::insert_into(room_to_users)
                .values(&new_room_to_user)
                .execute(conn)?;
        }
    }

    Ok(new_room_to_user)
}

pub fn leave_room(
    conn: &mut SqliteConnection,
    room_id: &str,
    user_id: &str,
) -> Result<(), DbError> {
    use crate::schema::room_to_users::dsl::*;
    diesel::delete(room_to_users.filter(room.eq(room_id)))
        .filter(user.eq(user_id))
        .execute(conn)?;
    Ok(())
}

pub fn get_current_room<'a>(conn: &'a mut SqliteConnection, ws_id: String) -> Option<Room> {
    use crate::schema::room_to_users;
    use crate::schema::rooms::dsl::*;
    let current_user = find_user_by_ws(conn, ws_id);
    let rooms_data: Vec<RoomToUser> = room_to_users::table.get_results(conn).unwrap();

    let exists = rooms_data.iter().find(|x| x.user == current_user.id);

    if exists.is_none() {
        return None;
    }
    let r = exists.unwrap();
    Some(
        rooms
            .filter(id.eq((r.room).to_string()))
            .first::<Room>(conn)
            .unwrap(),
    )
}
