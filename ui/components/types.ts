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
  chat_type: "JOIN" | "CONNECT" | "TEXT" | "LEAVE" | "DISCONNECT";
  value: string;
  user_id: string;
};
