//should be move one step up
export type Room = {
  id: string;
  name: string;
  created_at: string;
};

export type Message = {
  id: string;
  room_id: string;
  user_id: string;
  content: string;
  created_at: string;
};
export type DisplayMessage = {
  username: string;
  content: string;
};

export type User = {
  id: string;
  username: string;
  phone: string;
  web_socket_session: string;
  created_at: string;
};
export type ChatMessage = {
  chat_type:
    | "JOIN"
    | "CONNECT"
    | "TEXT"
    | "LEAVE"
    | "DISCONNECT"
    | "CREATEGAME"
    | "MOVE"
    | "JOINGAME";

  value: string;
  user_id: string;
};

export type TicTacToeGame = {
  id: string;
  player_1: string;
  player_2: string;
  game_status: string;
  created_at: string;
};

export interface DisplayGame extends TicTacToeGame {
  player_1_name?: string;
  playher_2_name?: string;
}

export type TicTacToeMove = {
  row_number: number;
  column_number: number;
  player_id: string;
  game_id: string;
};
