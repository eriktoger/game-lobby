import { useCallback, useState } from "react";
import ChatList, { Games } from "../components/rooms";
import Conversation from "../components/conversation";
import useRooms from "../libs/useRooms";
import useWebsocket from "../libs/useWebsocket";
import {
  ChatMessage,
  DisplayMessage,
  Room,
  TicTacToeGame,
  TicTacToeInfo,
  TicTacToeMove,
  User,
} from "./types";
import { Board } from "./games/TicTacToe/board";
import styled from "styled-components";

const Container = styled.main`
  padding: 0.5rem;
  background-color: #bdbdbd;
  width: 100vw;
  height: 100vh;
`;

const StyledInfo = styled.section`
  display: flex;
  justify-content: space-around;
  height: 50%;
  aside {
    width: 40vw;
  }
`;

export default function Main({ auth, setAuthUser }: any) {
  const [room, setSelectedRoom] = useState<Room | null>(null);
  const {
    isLoading,
    users,
    setUsers,
    messages,
    setMessages,
    fetchRoomData,
    games,
    setGames,
  } = useRooms();

  //should load the game if we refresh the page?
  //like a useEffect that looks of the user has an active game going
  //
  const [gameId, setGameId] = useState<null | string>(null);
  const [gameStatus, setGameStatus] = useState<null | string>(null);
  const [moves, setMoves] = useState<TicTacToeMove[]>([]);
  const [turn, setTurn] = useState<string | null>(null);

  const handleMessage = useCallback(
    (content: string, username: string) => {
      setMessages((prev: DisplayMessage[]) => {
        const item: DisplayMessage = {
          content,
          username,
        };
        return [...prev, item];
      });
    },
    [setMessages]
  );

  const onMessage = useCallback(
    async (data: any) => {
      try {
        let messageData = JSON.parse(data) as ChatMessage;
        switch (messageData.chat_type) {
          case "TEXT": {
            //should be in try catch or something...
            const { content, username } = JSON.parse(messageData.value);
            handleMessage(content, username);
            break;
          }
          case "CONNECT": {
            auth?.id &&
              fetch(
                `http://localhost:8080/users/${auth.id}/session/${messageData.value}`,
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
            break;
          }

          case "JOIN": {
            const user = JSON.parse(messageData.value) as User;
            setUsers((prev: any[]) => [...prev, user]);
            break;
          }
          case "DISCONNECT":
          case "LEAVE": {
            setUsers((prev: any[]) => {
              const newArray = [
                ...prev.filter((user) => user.username !== messageData.value),
              ];

              return newArray;
            });
            break;
          }
          case "CREATEGAME": {
            const game = JSON.parse(messageData.value) as TicTacToeGame; // should be Display game from backend
            if (messageData.user_id == auth.id) {
              setGameId(game.id);
            } else {
              //add it to games array
              //this should be loaded when you enter the room as well
              setGames((prev) => [
                ...prev,
                {
                  ...game,
                  player_1_name:
                    users.find((user) => user.id === game.player_1)?.username ??
                    null,
                  player_2_name:
                    users.find((user) => user.id === game.player_2)?.username ??
                    null,
                } as TicTacToeGame,
              ]);
            }
            break;
          }
          case "JOINGAME": {
            // Player 2 starts
            setTurn(messageData.user_id);
            break;
          }
          case "MOVE": {
            const info = JSON.parse(messageData.value) as TicTacToeInfo;
            console.log("MOVE", info);
            setTurn(info.turn);
            setGameStatus(info.game_status);
            setMoves((prev) => [...prev, info.last_move]);
            break;
          }
        }
      } catch (e) {
        console.log(e);
      }
    },
    [auth.id, handleMessage, setGames, setUsers, users]
  );

  const sendMessage = useWebsocket(onMessage);

  const submitMessage = (e: any) => {
    e.preventDefault();
    let message = e.target.message.value;
    if (message === "") {
      return;
    }

    if (!room) {
      alert("Please select chat room!");
      return;
    }
    const displayMessage = {
      content: message,
      username: auth.username,
    };
    const data: ChatMessage = {
      chat_type: "TEXT",
      value: JSON.stringify(displayMessage),
      user_id: auth.id,
    };
    sendMessage(JSON.stringify(data));
    e.target.message.value = "";
    handleMessage(message, auth.username);
  };

  const onChangeRoom = async (room: Room) => {
    if (!room.id) return;
    await fetchRoomData(room.id, auth);
    setSelectedRoom(room);

    const joinRoom = {
      chat_type: "JOIN",
      value: room.id,
      user_id: auth?.id,
    };
    sendMessage(JSON.stringify(joinRoom));
  };

  const submitMove = (row: number, column: number) => {
    if (gameId == null) {
      return;
    }

    const move = {
      game_id: gameId,
      row_number: row,
      column_number: column,
      player_id: auth.id,
    };
    const data: ChatMessage = {
      chat_type: "MOVE",
      value: JSON.stringify(move),
      user_id: auth.id,
    };
    //setMoves((prev) => [...prev, move]);
    sendMessage(JSON.stringify(data));
  };

  if (gameId) {
    return (
      <Board
        onClose={() => setGameId(null)}
        submitMove={submitMove}
        gameId={gameId}
        moves={moves}
        playerId={auth.id}
        gameStatus={gameStatus}
        turn={turn}
      />
    );
  }

  return (
    <Container>
      <StyledInfo>
        <ChatList
          onChangeRoom={onChangeRoom}
          userId={auth.id}
          users={users}
          currentRoom={room}
        />
        <Games
          games={games}
          auth={auth}
          sendMessage={sendMessage}
          setGameId={setGameId}
          setAuthUser={setAuthUser}
        />
      </StyledInfo>
      <section>
        {room?.id && (
          <>
            <div>{auth?.username}</div>
            <div>
              <div>
                <p>{room.name}</p>
              </div>
              <hr />
            </div>
            {isLoading && room.id && <p>Loading conversation...</p>}
            <Conversation data={messages} auth={auth} />
            <div className="w-full">
              <form onSubmit={submitMessage}>
                <input name="message" placeholder="Type your message here..." />
                <button type="submit">Sent</button>
              </form>
            </div>
          </>
        )}
      </section>
    </Container>
  );
}
