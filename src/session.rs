use crate::db;
use crate::models::DisplayMessage;
use crate::server;
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const HEARBEET: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct WsChatSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
    pub db_pool: web::Data<DbPool>,
}
#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum ChatType {
    TYPING,
    TEXT,
    JOIN,  //Join a room with websocket instead of routes
    LEAVE, // Leave a room (volentary disconnect I guess)
    CONNECT,
    DISCONNECT,
    CREATEGAME,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub chat_type: ChatType,
    pub value: String,
    pub user_id: String,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}
impl Handler<server::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let data_json = serde_json::from_str::<ChatMessage>(&text.to_string());
                if let Err(err) = data_json {
                    println!("{err}");
                    println!("Failed to parse message: {text}");
                    return;
                }
                let input = data_json.as_ref().unwrap();
                match &input.chat_type {
                    ChatType::TEXT => {
                        let input = data_json.as_ref().unwrap();

                        let mut conn = self.db_pool.get().unwrap();
                        let display_message =
                            serde_json::from_str::<DisplayMessage>(&input.value).unwrap();
                        let _ = db::insert_new_message(
                            &mut conn,
                            self.id.to_string(),
                            display_message.content,
                        );
                        let msg = serde_json::to_string(input).unwrap();
                        let current_room =
                            db::get_current_room(&mut conn, self.id.to_string()).unwrap();
                        self.addr.do_send(server::ClientMessage {
                            id: self.id,
                            msg,
                            room: current_room.id,
                        })
                    }
                    ChatType::JOIN => {
                        let mut conn = self.db_pool.get().unwrap();
                        let current_room = db::get_current_room(&mut conn, self.id.to_string());

                        let current_user = db::find_user_by_ws(&mut conn, self.id.to_string());

                        match current_room {
                            Some(r) => {
                                if r.id == input.value {
                                    return; // we are already in the room
                                } else {
                                    //we should leave the room and join another

                                    let chat_msg = ChatMessage {
                                        chat_type: ChatType::LEAVE,
                                        value: current_user.username.clone(), //to tell who has left
                                        user_id: current_user.id.clone(),
                                    };
                                    let msg = serde_json::to_string(&chat_msg).unwrap();

                                    // the problem is that do_send is async the acutal sending happens after leave_room
                                    // which causes the message to be sent to the room wrong
                                    //maybe just use room.id instead?
                                    self.addr.do_send(server::ClientMessage {
                                        id: self.id,
                                        msg,
                                        room: r.id.clone(),
                                    });

                                    let _ = db::leave_room(&mut conn, &r.id, &current_user.id);
                                    let _ =
                                        db::join_room(&mut conn, &input.value, &current_user.id);
                                }
                            }

                            None => {
                                let _ = db::join_room(&mut conn, &input.value, &current_user.id);
                            } //we should only join a room
                        }

                        //comparing input and current_room should be enough to see if we should do something
                        // join the room (if we are not already in it)
                        // leave our old room if we have one
                        // send the appropriate messages to the old and new room

                        let chat_msg = ChatMessage {
                            chat_type: ChatType::JOIN,
                            value: serde_json::to_string(&current_user).unwrap(), //to tell who has joined
                            user_id: current_user.id,
                        };
                        let msg = serde_json::to_string(&chat_msg).unwrap();
                        self.addr.do_send(server::ClientMessage {
                            id: self.id,
                            msg,
                            room: input.value.clone(),
                        })
                    }
                    ChatType::LEAVE => {
                        //Not sure when this should be called
                        println!("LEAVING{:?}", input);
                    }
                    ChatType::CREATEGAME => {
                        //we may use the fact that it is a tic-tac-toe -game later
                        //let input = data_json.as_ref().unwrap();

                        let mut conn = self.db_pool.get().unwrap();
                        let new_game =
                            db::create_tic_tac_toe(&mut conn, self.id.to_string()).unwrap();

                        let current_user = db::find_user_by_ws(&mut conn, self.id.to_string());
                        let chat_msg = ChatMessage {
                            chat_type: ChatType::CREATEGAME,
                            value: serde_json::to_string(&new_game).unwrap(),
                            user_id: current_user.id,
                        };

                        let msg = serde_json::to_string(&chat_msg).unwrap();
                        let current_room =
                            db::get_current_room(&mut conn, self.id.to_string()).unwrap();
                        self.addr.do_send(server::ClientMessage {
                            id: 0, //sends it to yourself as well
                            msg,
                            room: current_room.id,
                        })
                    }
                    _ => {}
                }
            }
            ws::Message::Binary(_) => println!("Unsupported binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARBEET, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}
