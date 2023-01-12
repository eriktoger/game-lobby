import { useEffect, useState } from "react";
const fetchRoomData = async (room_id: any, user_id?: string) => {
  if (!room_id) return;
  const url = `http://localhost:8080/conversations/${room_id}`;
  const url2 = `http://localhost:8080/rooms/join`;
  try {
    const resp = fetch(url).then((res) => res.json());
    // user_id &&
    //   (await fetch(url2, {
    //     method: "POST",
    //     headers: {
    //       "Content-Type": "application/json",
    //     },
    //     body: JSON.stringify({ room: room_id, user: user_id }),
    //   }).then((res) => res.json()));
    return resp;
  } catch (e) {
    console.log(e);
  }
};
export default function useConversations(room_id: any) {
  const [isLoading, setIsLoading] = useState(true);
  const [messages, setMessages] = useState<any>([]);
  const updateMessages = (resp = []) => {
    setIsLoading(false);
    setMessages(resp);
  };
  const fetchConversations = (room_id: any, user_id?: string) => {
    setIsLoading(true);
    fetchRoomData(room_id, user_id).then(updateMessages);
  };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  useEffect(() => fetchConversations(room_id), []);
  return [isLoading, messages, setMessages, fetchConversations];
}
