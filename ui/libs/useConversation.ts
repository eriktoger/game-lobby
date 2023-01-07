import { useEffect, useState } from "react";
const fetchRoomData = async (room_id: any) => {
  if (!room_id) return;
  const url = `http://localhost:8080/conversations/${room_id}`;
  try {
    let resp = await fetch(url).then((res) => res.json());
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
  const fetchConversations = (id: any) => {
    setIsLoading(true);
    fetchRoomData(id).then(updateMessages);
  };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  useEffect(() => fetchConversations(room_id), []);
  return [isLoading, messages, setMessages, fetchConversations];
}
