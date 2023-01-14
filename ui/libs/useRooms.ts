import { useState } from "react";
import { Message, User } from "../components/types";

const fetchRoom = async (room_id: string) => {
  const url = `http://localhost:8080/rooms/${room_id}/data`;
  try {
    const resp = fetch(url).then((res) => res.json());

    return resp;
  } catch (e) {
    console.log(e);
  }
};

//maybe should be a context, since the websocket will update it
//then let current game be its own context?
export default function useRooms() {
  const [isLoading, setIsLoading] = useState(true);
  const [messages, setMessages] = useState<Message[]>([]);
  const [users, setUsers] = useState<User[]>([]);

  const fetchRoomData = async (room_id: string, user: User) => {
    setIsLoading(true);
    const data = await fetchRoom(room_id);
    console.log("from fetch room: ", { data, user });
    setUsers([...data.users, user]);
    setMessages(data.messages);
    setIsLoading(false);
  };

  return { isLoading, users, setUsers, messages, setMessages, fetchRoomData };
}
