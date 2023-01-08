use crate::models::{Message, NewMessage, Room, RoomResponse, RoomToUser, User};
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

pub fn find_user_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
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

pub fn get_messages_by_room_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<Vec<Message>>, DbError> {
    use crate::schema::messages;

    let convo = messages::table
        .filter(messages::room_id.eq(uid.to_string()))
        .load(conn)
        .optional()?;

    Ok(convo)
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str, pn: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        phone: pn.to_owned(),
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
    new: NewMessage,
) -> Result<Message, DbError> {
    use crate::schema::messages::dsl::*;
    let new_message = Message {
        id: Uuid::new_v4().to_string(),
        room_id: new.room_id,
        content: new.content,
        created_at: iso_date(),
    };
    diesel::insert_into(messages)
        .values(&new_message)
        .execute(conn)?;
    Ok(new_message)
}

pub fn get_all_rooms(conn: &mut SqliteConnection) -> Result<Vec<RoomResponse>, DbError> {
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
