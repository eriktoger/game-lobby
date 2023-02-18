use crate::{
    models::{
        DisplayMessage, Message, NewTicTacToeMove, Room, RoomResponse, RoomToUser, TicTacToeGame,
        TicTacToeMove, User,
    },
    schema::tic_tac_toe_moves::player_id,
};
use chrono::{DateTime, Utc};
use diesel::{prelude::*, row};
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
    print!("inside: {}", ws_id);
    let res = users
        .filter(web_socket_session.eq(ws_id.to_string()))
        .first::<User>(conn)
        .unwrap();
    print!("inside after: {}", ws_id);
    return res;
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

pub fn insert_new_move(
    conn: &mut SqliteConnection,
    new_move: NewTicTacToeMove,
) -> Result<TicTacToeMove, DbError> {
    use crate::schema::tic_tac_toe_moves::dsl::*;

    use crate::schema::tic_tac_toe_games::dsl::{
        id as game_id, player_1, player_2, tic_tac_toe_games, turn,
    };

    let tic_tac_toe_move = TicTacToeMove {
        id: Uuid::new_v4().to_string(),
        player_id: new_move.player_id.clone(),
        game_id: new_move.game_id.clone(),
        row_number: new_move.row_number,
        column_number: new_move.column_number,
        created_at: iso_date(),
    };
    diesel::insert_into(tic_tac_toe_moves)
        .values(&tic_tac_toe_move)
        .execute(conn)?;

    let (p1, p2): (String, Option<String>) = tic_tac_toe_games
        .filter(game_id.eq(new_move.game_id.clone()))
        .select((player_1, player_2))
        .first(conn)
        .unwrap();

    let new_turn = if new_move.player_id == p1 {
        p2.unwrap()
    } else {
        p1
    };

    diesel::update(tic_tac_toe_games)
        .filter(game_id.eq(new_move.game_id.to_string()))
        .set(turn.eq(new_turn))
        .execute(conn)?;

    Ok(tic_tac_toe_move)
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

pub fn get_oponent(
    conn: &mut SqliteConnection,
    game_id: String,
    ws_id: String,
) -> Result<String, DbError> {
    use crate::schema::tic_tac_toe_games::dsl::*;
    use crate::schema::users::dsl::{id as user_id, users};

    let current_user = find_user_by_ws(conn, ws_id);

    let players: Vec<(String, Option<String>)> = tic_tac_toe_games
        .filter(id.eq(game_id.clone()))
        .order(created_at.desc())
        .select((player_1, player_2))
        .get_results(conn)
        .expect("No opponent found");

    // we want the wsc not the id
    if players.len() > 0 {
        if players[0].0 == current_user.id {
            let p2 = players[0].1.as_ref().unwrap();
            let u: Vec<User> = users.filter(user_id.eq(p2)).get_results(conn)?;
            return Ok(u[0].web_socket_session.clone());
        }
        let p1 = &players[0].0;
        let u: Vec<User> = users.filter(user_id.eq(p1)).get_results(conn)?;
        return Ok(u[0].web_socket_session.clone());
    }
    return Ok("".to_string());
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

pub fn get_games_in_room(
    conn: &mut SqliteConnection,
    room_id: String,
) -> Result<Vec<TicTacToeGame>, DbError> {
    use crate::schema::tic_tac_toe_games;

    use crate::schema::rooms::dsl::{id as r_id, rooms};

    let r: Vec<Room> = rooms.filter(r_id.eq(room_id.clone())).get_results(conn)?;
    if r.len() != 1 || r[0].name != "Tic Tac Toe" {
        return Ok([].to_vec());
    }

    let games: Vec<TicTacToeGame> = tic_tac_toe_games::table.load(conn)?;

    Ok(games)
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

pub fn join_game(conn: &mut SqliteConnection, game_id: &str, user_id: &str) -> Result<(), DbError> {
    use crate::schema::tic_tac_toe_games::dsl::*;

    diesel::update(tic_tac_toe_games)
        .filter(id.eq(game_id.to_string()))
        .set(player_2.eq(user_id))
        .execute(conn)?;

    diesel::update(tic_tac_toe_games)
        .filter(id.eq(game_id.to_string()))
        .set(turn.eq(user_id))
        .execute(conn)?;

    Ok(())
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

pub fn create_tic_tac_toe<'a>(
    conn: &'a mut SqliteConnection,
    ws_id: String,
) -> Result<TicTacToeGame, DbError> {
    use crate::schema::tic_tac_toe_games::dsl::*;

    let current_user = find_user_by_ws(conn, ws_id);

    let new_game_id = Uuid::new_v4().to_string();
    let new_game = TicTacToeGame {
        id: new_game_id.clone(),
        player_1: current_user.id,
        player_2: None,
        turn: None,
        game_status: "Active".to_string(),
        created_at: iso_date(),
    };

    diesel::insert_into(tic_tac_toe_games)
        .values(&new_game)
        .execute(conn)?;
    Ok(new_game)
}

pub fn your_turn<'a>(conn: &'a mut SqliteConnection, game_id: String, user_id: String) -> bool {
    use crate::schema::tic_tac_toe_games::dsl::*;

    let whos_turn: Option<String> = tic_tac_toe_games
        .filter(id.eq(game_id.clone()))
        .select(turn)
        .first(conn)
        .expect("No opponent found");
    match whos_turn {
        Some(t) => t == user_id,
        None => false,
    }
}

pub fn legal_move<'a>(
    conn: &'a mut SqliteConnection,
    game_id: String,
    row_number: i32,
    column_number: i32,
) -> bool {
    use crate::schema::tic_tac_toe_moves::dsl::{
        column_number as c_number, game_id as g_id, row_number as r_number, tic_tac_toe_moves,
    };

    let played_moves: Vec<(i32, i32)> = tic_tac_toe_moves
        .filter(g_id.eq(game_id.clone()))
        .select((r_number, c_number))
        .get_results(conn)
        .unwrap();
    let square_occupied = played_moves
        .into_iter()
        .find(|(r, c)| *r == row_number && *c == column_number);

    match square_occupied {
        Some(_) => false,
        None => true,
    }
}

pub fn game_result<'a>(conn: &'a mut SqliteConnection, new_move: NewTicTacToeMove) -> String {
    use crate::schema::tic_tac_toe_games::dsl::{game_status, id, player_1, tic_tac_toe_games};
    use crate::schema::tic_tac_toe_moves::dsl::{
        column_number, game_id, row_number, tic_tac_toe_moves,
    };

    let your_moves: Vec<(i32, i32)> = tic_tac_toe_moves
        .filter(game_id.eq(new_move.game_id.clone()))
        .filter(player_id.eq(new_move.player_id.clone()))
        .select((row_number, column_number))
        .get_results(conn)
        .unwrap();

    // there is 8 ways to win 3 rows, 3 columns, 2 diagionally
    let mut columns = [0, 0, 0];
    let mut rows = [0, 0, 0];
    let mut diagonals = [0, 0];

    let nr_of_moves = your_moves.len();
    for (row, col) in your_moves.into_iter() {
        columns[col as usize] += 1;
        rows[row as usize] += 1;
        if col == row {
            diagonals[0] += 1;
        }
        if col + row == 2 {
            diagonals[1] += 1;
        }
    }
    let win_condition = &3;
    let column_win = columns.contains(win_condition);
    let row_win = rows.contains(win_condition);
    let diagional_win = diagonals.contains(win_condition);
    if column_win || row_win || diagional_win {
        let p1: String = tic_tac_toe_games
            .filter(id.eq(new_move.game_id.clone()))
            .select(player_1)
            .first(conn)
            .unwrap();

        let new_status = if new_move.player_id == p1 {
            "player_1_won"
        } else {
            "player_2_won"
        };

        let _ = diesel::update(tic_tac_toe_games)
            .filter(id.eq(new_move.game_id))
            .set(game_status.eq(new_status))
            .execute(conn);
        return new_status.to_string();
    }

    //if your_moves.length === 5 (and no win) it is a draw
    if nr_of_moves == 5 {
        let _ = diesel::update(tic_tac_toe_games)
            .filter(id.eq(new_move.game_id))
            .set(game_status.eq("draw"))
            .execute(conn);
        return "draw".to_string();
    }

    "Active".to_string()
}
