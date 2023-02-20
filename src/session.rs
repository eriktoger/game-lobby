use crate::models::{DisplayMessage, TicTacToeInfo};
use crate::server;
use crate::{db, models::NewTicTacToeMove};
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
    MOVE,
    JOINGAME,
    GAMEOVER,
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
                            room: Some(current_room.id),
                            game: None,
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
                                        room: Some(r.id.clone()),
                                        game: None,
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
                            room: Some(input.value.clone()),
                            game: None,
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
                        println!("ws id: {}", self.id.to_string());
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
                            room: Some(current_room.id),
                            game: None,
                        })
                    }
                    ChatType::JOINGAME => {
                        let mut conn = self.db_pool.get().unwrap();

                        let current_user = db::find_user_by_ws(&mut conn, self.id.to_string());

                        let _ = db::join_game(&mut conn, &input.value, &current_user.id);

                        let chat_msg = ChatMessage {
                            chat_type: ChatType::JOINGAME,
                            value: input.value.clone(),
                            user_id: current_user.id,
                        };
                        self.addr.do_send(server::ClientMessage {
                            id: self.id,
                            msg: serde_json::to_string(&chat_msg).unwrap(),
                            room: None,
                            game: Some(input.value.clone()),
                        })
                    }
                    ChatType::MOVE => {
                        println!("MOVE!");
                        let input = data_json.as_ref().unwrap();

                        let mut conn = self.db_pool.get().unwrap();
                        let current_user = db::find_user_by_ws(&mut conn, self.id.to_string());
                        let new_move =
                            serde_json::from_str::<NewTicTacToeMove>(&input.value).unwrap();

                        //verify that  is your turn
                        let your_turn = db::your_turn(
                            &mut conn,
                            new_move.game_id.clone(),
                            current_user.id.clone(),
                        );

                        if !your_turn {
                            return;
                        }
                        //verify that is a legal move, that is has not all ready beeing played
                        let legal_move = db::legal_move(
                            &mut conn,
                            new_move.game_id.clone(),
                            new_move.row_number.clone(),
                            new_move.column_number.clone(),
                        );
                        if !legal_move {
                            return;
                        }
                        let old_game_status =
                            db::get_game_result(&mut conn, new_move.game_id.clone());

                        if old_game_status != "Active" {
                            return;
                        }

                        let (last_move, whos_turn) =
                            db::insert_new_move(&mut conn, new_move.clone()).unwrap();

                        //check if the game is over

                        let game_status = db::update_game_result(&mut conn, new_move);

                        //Im not sure how to send the message only to the game, since games are not rooms.
                        //one is to look up the "room" in both room and games
                        //or expand ClientMessage with type? ChatMessage/TicTacToe/what ever game we put in.
                        // I think looking in games is the easiet for now
                        let game_id = last_move.game_id.clone();
                        let info = TicTacToeInfo {
                            last_move,
                            game_status: game_status.clone(),
                            turn: whos_turn,
                        };

                        let chat_msg = ChatMessage {
                            chat_type: ChatType::MOVE,
                            value: serde_json::to_string(&info).unwrap(),
                            user_id: current_user.id.clone(),
                        };
                        self.addr.do_send(server::ClientMessage {
                            id: self.id,
                            msg: serde_json::to_string(&chat_msg).unwrap(),
                            room: None,
                            game: Some(game_id.clone()),
                        });
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
