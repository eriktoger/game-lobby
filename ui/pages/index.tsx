import Head from "next/head";
import React, { useCallback, useEffect, useState } from "react";
import Avatar from "../components/avatar";
import ChatList from "../components/rooms";
import Conversation from "../components/conversation";
import Login from "../components/login";
import useConversations from "../libs/useConversation";
import useLocalStorage from "../libs/useLocalStorage";
import useWebsocket from "../libs/useWebsocket";
import Main from "../components/main";

export default function Home() {
  const [showLogIn, setShowLogIn] = useState(false);
  const [auth, setAuthUser] = useLocalStorage("user", false);

  useEffect(() => setShowLogIn(!auth), [auth]);

  return (
    <div>
      <Head>
        <title>Rust with react chat app</title>
        <meta name="description" content="Rust with react chat app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Login show={showLogIn} setAuth={setAuthUser} />
      <div
        className={`${
          !auth && "hidden"
        } bg-gradient-to-b from-orange-400 to-rose-400 h-screen p-12`}
      >
        {auth && <Main auth={auth} setAuthUser={setAuthUser} />}
      </div>
    </div>
  );
}
