use crate::{
    db::{self, DbError},
    models::{Room, User},
    session::{self},
};
use actix::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use rand::{self, rngs::ThreadRng, Rng};
use serde_json::json;
use std::collections::{HashMap, HashSet};
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize, //Can this be string= and the user id?
    pub msg: String,
    pub room: Option<String>,
    pub game: Option<String>,
}
pub struct ListRooms;
impl actix::Message for ListRooms {
    type Result = Vec<String>;
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>, // I wint this to be the id instead of usize
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ChatServer {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> ChatServer {
        println!("Chat server starting...");
        let mut rooms = HashMap::new();
        rooms.insert("main".to_string(), HashSet::new());
        Self {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            pool,
        }
    }
    //we could pass in ChatMessage instead of message
    fn send_message_to_room(&self, room: &str, message: &str, ws_id: usize) {
        let mut conn = self.pool.get().unwrap();

        let receivers = db::get_users_in_room(&mut conn, room.to_string()).unwrap();

        for receiver in receivers {
            let id = receiver.web_socket_session;
            if id != ws_id.to_string() {
                if let Some(addr) = self.sessions.get(&id.parse::<usize>().unwrap()) {
                    addr.do_send(Message(message.to_owned()));
                }
            }
        }
    }
    fn send_message_to_game(&self, game: &str, message: &str, ws_id: usize) {
        let mut conn = self.pool.get().unwrap();

        //If I want to to send to both players I cant use ws_id
        let opponents_wsc =
            db::get_oponent(&mut conn, game.to_string(), ws_id.to_string()).unwrap();
        println!("opsc: {}", opponents_wsc);
        if let Some(addr) = self.sessions.get(&opponents_wsc.parse::<usize>().unwrap()) {
            println!("Sending to oppnent");
            addr.do_send(Message(message.to_owned()));
        }
        if let Some(addr) = self.sessions.get(&ws_id) {
            println!("Sending to self");
            addr.do_send(Message(message.to_owned()));
        }
    }
    fn send_message_self(&self, room: &str, message: &str, self_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id == self_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}
impl Actor for ChatServer {
    type Context = Context<Self>;
}
impl Handler<Connect> for ChatServer {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        self.rooms
            .entry("main".to_string())
            .or_insert_with(HashSet::new)
            .insert(id);
        self.send_message_self(
            "main",
            &json!({
                "value": vec![format!("{}", id)],
                "chat_type": session::ChatType::CONNECT
            })
            .to_string(),
            id,
        );
        id
    }
}

fn get_user_and_room(
    pool: &mut Pool<ConnectionManager<SqliteConnection>>,
    ws_id: String,
) -> Result<Option<(Room, User)>, DbError> {
    let mut conn = pool.get()?;
    let option_room = db::get_current_room(&mut conn, ws_id.clone())?;
    if option_room.is_none() {
        return Ok(None);
    }

    let current_room = option_room.unwrap();
    let current_user = db::find_user_by_ws(&mut conn, ws_id)?;
    let _ = db::leave_room(&mut conn, &current_room.id, &current_user.id)?;
    Ok(Some((current_room, current_user)))
}
impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let mut rooms: Vec<String> = vec![];
        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        //this should send a message that the user left the current chat channel.
        //and you should leave the room ()

        //we also need to send it to the game if the user disconnects.
        match get_user_and_room(&mut self.pool, msg.id.to_string()) {
            Ok(option) => match option {
                Some((current_room, current_user)) => {
                    self.send_message_to_room(
                        &current_room.id,
                        &json!({
                            "room": current_room.name,
                            "value": current_user.username,
                            "chat_type": session::ChatType::DISCONNECT
                        })
                        .to_string(),
                        0,
                    );
                }
                None => return,
            },
            Err(_) => return,
        }
    }
}
//Can I have a handler each for games and chat?
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(room) = msg.room {
            self.send_message_to_room(&room, &msg.msg, msg.id);
        }
        if let Some(game) = msg.game {
            self.send_message_to_game(&game, &msg.msg, msg.id);
        }
    }
}
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
        let mut rooms = vec![];
        for key in self.rooms.keys() {
            rooms.push(key.to_owned());
        }
        MessageResult(rooms)
    }
}
impl Handler<Join> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
        let Join { id, name } = msg;
        let mut rooms = vec![];
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        for room in rooms {
            self.send_message_to_room(
                &room,
                &json!({
                    "room": room,
                    "value": vec![format!("Someone disconnect!")],
                    "chat_type": session::ChatType::DISCONNECT
                })
                .to_string(),
                0,
            );
        }
        self.rooms
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);
    }
}
