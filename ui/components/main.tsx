import { useCallback, useState } from "react";
import ChatList from "../components/rooms";
import Conversation from "../components/conversation";
import useRooms from "../libs/useRooms";
import useWebsocket from "../libs/useWebsocket";
import {
  ChatMessage,
  DisplayMessage,
  Room,
  TicTacToeGame,
  TicTacToeMove,
  User,
} from "./types";
import { Board } from "./games/TicTacToe/board";

export default function Main({ auth, setAuthUser }: any) {
  const [room, setSelectedRoom] = useState<Room | null>(null);
  const [gameId, setGameId] = useState<null | string>(null);
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
  const [moves, setMoves] = useState<TicTacToeMove[]>([]);

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
              setGameId(messageData.value);
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
      //this should have a type
      chat_type: "JOIN",
      value: room.id,
      user_id: auth?.id,
    };
    sendMessage(JSON.stringify(joinRoom));
  };

  const signOut = () => {
    window.localStorage.removeItem("user");
    setAuthUser(false);
  };

  if (gameId) {
    return (
      <Board
        onClose={() => setGameId(null)}
        gameId={gameId}
        moves={moves}
        playerId={auth.id}
      />
    );
  }

  return (
    <main className="flex w-full max-w-[1020px] h-[700px] mx-auto bg-[#FAF9FE] rounded-[25px] backdrop-opacity-30 opacity-95">
      <aside className="bg-[#F0EEF5] w-[325px] h-[700px] rounded-l-[25px] p-4 overflow-auto relative">
        <ChatList onChangeRoom={onChangeRoom} userId={auth.id} users={users} />
        <button
          onClick={() => {
            const newGame: ChatMessage = {
              chat_type: "CREATEGAME",
              value: "Tic_Tac_Toe",
              user_id: auth.id,
            };
            sendMessage(JSON.stringify(newGame));
          }}
        >
          Create new game
        </button>
        <div>
          <span>Current games</span>
          <ul>
            {games.map((game) => (
              <li key={game.id}>
                <button onClick={() => setGameId(game.id)}>
                  Play against: {game.player_1_name}
                </button>
              </li>
            ))}
          </ul>
        </div>
        <button
          onClick={signOut}
          className="text-xs w-full max-w-[295px] p-3 rounded-[10px] bg-violet-200 font-semibold text-violet-600 text-center absolute bottom-4"
        >
          LOG OUT
        </button>
      </aside>

      {room?.id && (
        <section className="rounded-r-[25px] w-full max-w-[690px] grid grid-rows-[80px_80px_minmax(450px,_1fr)_65px]">
          <div>{auth?.username}</div>
          <div className="rounded-tr-[25px] w-full h-16">
            <div className="flex gap-3 p-3 items-center">
              <p className="font-semibold text-gray-600 text-base">
                {room.name}
              </p>
            </div>
            <hr className="bg-[#F0EEF5]" />
          </div>
          {isLoading && room.id && (
            <p className="px-4 text-slate-500">Loading conversation...</p>
          )}
          <Conversation data={messages} auth={auth} />
          <div className="w-full">
            <form
              onSubmit={submitMessage}
              className="flex gap-2 items-center rounded-full border border-violet-500 bg-violet-200 p-1 m-2"
            >
              <input
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
