import { ChatMessage } from "components/types";
import { useEffect, useRef } from "react";
import { showErrorToast } from "toast";
const wsUri = process.env.NEXT_PUBLIC_WS_URI;

export default function useWebsocket(
  onMessage: (chatMessage: ChatMessage) => void
) {
  const ws = useRef<WebSocket | null>(null);
  useEffect(() => {
    if (ws.current !== null) return;
    ws.current = new WebSocket(wsUri);
    ws.current.onopen = () => console.log("ws opened");
    ws.current.onclose = () => console.log("ws closed!");
  }, []);

  useEffect(() => {
    if (!ws.current) return;
    ws.current.onmessage = (e: MessageEvent<any>) => {
      try {
        const chatMessage = JSON.parse(e.data) as ChatMessage;
        onMessage(chatMessage);
      } catch (error) {
        console.error(error);
        showErrorToast("Message not parsed");
      }
    };
  }, [onMessage]);

  const sendMessage = (msg: ChatMessage) => {
    if (!ws.current) return;
    ws.current.send(JSON.stringify(msg));
  };

  return sendMessage;
}
