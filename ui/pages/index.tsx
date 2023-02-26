import Head from "next/head";
import React, { useEffect, useState } from "react";
import Login from "../components/login";
import useLocalStorage from "../libs/useLocalStorage";
import Main from "../components/main";
import styled from "styled-components";

const StyledContainer = styled.div`
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
`;

export default function Home() {
  const [auth, setAuthUser] = useLocalStorage("user", false);

  return (
    <StyledContainer>
      <Head>
        <title>Rust with react chat app</title>
        <meta name="description" content="Rust with react chat app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      {!auth && <Login setAuth={setAuthUser} />}
      {auth && <Main auth={auth} setAuthUser={setAuthUser} />}
    </StyledContainer>
  );
}
