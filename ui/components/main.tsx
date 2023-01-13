import Head from "next/head";
import React, { useCallback, useEffect, useState } from "react";
import Avatar from "../components/avatar";
import ChatList from "../components/rooms";
import Conversation from "../components/conversation";
import Login from "../components/login";
import useConversations from "../libs/useConversation";
import useLocalStorage from "../libs/useLocalStorage";
import useWebsocket from "../libs/useWebsocket";

export default function Main({ auth, setAuthUser }: any) {
  const [room, setSelectedRoom] = useState<any>(null);
  const [users, setSelectedUsers] = useState<any>(null);
  const [isTyping, setIsTyping] = useState(false);
  const [isLoading, messages, setMessages, fetchConversations] =
    useConversations("");
  const handleTyping = (mode: any) => {
    if (mode === "IN") {
      setIsTyping(true);
    } else {
      setIsTyping(false);
    }
  };

  const handleMessage = useCallback(
    (msg: any, userId: any) => {
      setMessages((prev: any) => {
        const item = { content: msg, user_id: userId };
        return [...prev, item];
      });
    },
    [setMessages]
  );

  const onMessage = useCallback(
    (data: any) => {
      try {
        let messageData = JSON.parse(data);
        switch (messageData.chat_type) {
          case "TYPING": {
            handleTyping(messageData.value[0]);
            return;
          }
          case "TEXT": {
            handleMessage(messageData.value[0], messageData.user_id);
            return;
          }
          case "CONNECT": {
            console.log(1, { messageData, auth });
            auth?.id &&
              fetch(
                `http://localhost:8080/users/${auth.id}/session/${messageData.value[0]}`,
                {
                  method: "POST",
                  headers: {
                    "Content-Type": "application/json",
                  },
                }
              );
            //use this data to update the user and give it the websocket-id
            //so that you can use it to send messages when we only have the user_id
            //But it still seems to be a work around.
          }
        }
      } catch (e) {
        console.log(e);
      }
    },
    [auth, handleMessage]
  );

  const sendMessage = useWebsocket(onMessage);
  const updateFocus = () => {
    const data = {
      id: 0,
      chat_type: "TYPING",
      value: ["IN"],
      room_id: room.id,
      user_id: auth.id,
    };
    sendMessage(JSON.stringify(data));
  };

  const onFocusChange = () => {
    const data = {
      id: 0,
      chat_type: "TYPING",
      value: ["OUT"],
      room_id: room.id,
      user_id: auth.id,
    };
    sendMessage(JSON.stringify(data));
  };

  const submitMessage = (e: any) => {
    e.preventDefault();
    let message = e.target.message.value;
    if (message === "") {
      return;
    }

    if (!room.id) {
      alert("Please select chat room!");
      return;
    }

    const data = {
      id: 0,
      chat_type: "TEXT",
      value: [message],
      room_id: room.id,
      user_id: auth.id,
    };
    sendMessage(JSON.stringify(data));
    e.target.message.value = "";
    handleMessage(message, auth.id);
    onFocusChange();
  };

  const updateMessages = async (data: any) => {
    if (!data.room.id) return;
    await fetchConversations(data.room.id, auth?.id);
    setSelectedRoom(data.room);
    setSelectedUsers(data.users);
    const d = {
      chat_type: "JOIN",
      value: data.room.id,
    };
    console.log({ d });
    sendMessage(JSON.stringify(d));
  };

  const signOut = () => {
    window.localStorage.removeItem("user");
    setAuthUser(false);
  };

  return (
    <main className="flex w-full max-w-[1020px] h-[700px] mx-auto bg-[#FAF9FE] rounded-[25px] backdrop-opacity-30 opacity-95">
      <aside className="bg-[#F0EEF5] w-[325px] h-[700px] rounded-l-[25px] p-4 overflow-auto relative">
        <ChatList onChatChange={updateMessages} userId={auth.id} />
        <button
          onClick={signOut}
          className="text-xs w-full max-w-[295px] p-3 rounded-[10px] bg-violet-200 font-semibold text-violet-600 text-center absolute bottom-4"
        >
          LOG OUT
        </button>
      </aside>

      {room?.id && (
        <section className="rounded-r-[25px] w-full max-w-[690px] grid grid-rows-[80px_minmax(450px,_1fr)_65px]">
          <div>{auth?.username}</div>
          <div className="rounded-tr-[25px] w-ful">
            <div className="flex gap-3 p-3 items-center">
              <p className="font-semibold text-gray-600 text-base">
                {room.name}
              </p>
              <div className="text-xs text-gray-400">
                {isTyping ? "Typing..." : "10:15 AM"}
              </div>
            </div>
            <hr className="bg-[#F0EEF5]" />
          </div>
          {isLoading && room.id && (
            <p className="px-4 text-slate-500">Loading conversation...</p>
          )}
          <Conversation data={messages} auth={auth} users={users} />
          <div className="w-full">
            <form
              onSubmit={submitMessage}
              className="flex gap-2 items-center rounded-full border border-violet-500 bg-violet-200 p-1 m-2"
            >
              <input
                onBlur={onFocusChange}
                onFocus={updateFocus}
                name="message"
                className="p-2 placeholder-gray-600 text-sm w-full rounded-full bg-violet-200 focus:outline-none"
                placeholder="Type your message here..."
              />
              <button
                type="submit"
                className="bg-violet-500 rounded-full py-2 px-6 font-semibold text-white text-sm"
              >
                Sent
              </button>
            </form>
          </div>
        </section>
      )}
    </main>
  );
}
