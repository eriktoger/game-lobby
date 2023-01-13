use crate::{
    db,
    session::{self, ChatMessage},
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
    pub room: String,
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
    fn send_message(&self, room: &str, message: &str, ws_id: usize) {
        println!("sending message {} {}", message, room);
        let mut conn = self.pool.get().unwrap();

        let current_room = db::get_current_room(&mut conn, ws_id.to_string());
        let rooms = db::get_all_rooms(&mut conn).unwrap();
        let current_rr = db::get_current_room_with_users(&mut conn, ws_id.to_string()).unwrap();
        //let room_members = db::get_room_members(skip_id);
        println!("ws_id {:?} ", ws_id);
        // room is the actuall id to the room, we could just find it in rooms

        for usr in current_rr.users {
            let id = usr.web_socket_session;
            if id != ws_id.to_string() {
                if let Some(addr) = self.sessions.get(&id.parse::<usize>().unwrap()) {
                    addr.do_send(Message(message.to_owned()));
                }
            }
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
        //this should not send a message right?
        println!("msg: {:?}", msg);
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
        for room in rooms {
            self.send_message(
                "main",
                &json!({
                    "room": room,
                    "value": vec![format!("Someone disconnect!")],
                    "chat_type": session::ChatType::DISCONNECT
                })
                .to_string(),
                0,
            );
        }
    }
}
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        println!("handle message");
        self.send_message(&msg.room, &msg.msg, msg.id);
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
        println!("joining chat room");
        let Join { id, name } = msg;
        let mut rooms = vec![];
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        for room in rooms {
            self.send_message(
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
