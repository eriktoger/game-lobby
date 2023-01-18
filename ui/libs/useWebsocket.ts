import { useEffect, useRef } from "react";
export default function useWebsocket(onMessage: any) {
  const ws = useRef<any>(null);
  useEffect(() => {
    if (ws.current !== null) return;
    const wsUri = "ws://localhost:8080/ws";
    ws.current = new WebSocket(wsUri);
    ws.current.onopen = () => console.log("ws opened");
    ws.current.onclose = () => console.log("ws closed");
    const wsCurrent = ws.current;
    return () => {
      //wsCurrent.close(); //this closed my websocket to early
    };
  }, []);
  useEffect(() => {
    if (!ws.current) return;
    ws.current.onmessage = (e: any) => {
      onMessage(e.data);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  const sendMessage = (msg: any) => {
    if (!ws.current) return;
    ws.current.send(msg);
  };
  return sendMessage;
}
