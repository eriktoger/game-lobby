import React, { useState, useEffect, SetStateAction, Dispatch } from "react";
import styled from "styled-components";
import { ChatMessage, DisplayGame, Room, User } from "./types";

async function getRooms() {
  try {
    const url = "http://localhost:8080/rooms";
    let result = await fetch(url);
    return result.json();
  } catch (e) {
    console.log(e);
    return Promise.resolve(null);
  }
}

const StyledItem = styled.h3<{ selected: boolean }>`
  padding: 0.2rem 0;
  text-decoration: ${(props) => (props.selected ? "underline" : "none")};
`;

const UserContainer = styled.div`
  border-top: 1px solid #4c4c4c;
  margin-top: 0.5rem;
`;

function ChatListItem({
  onChangeRoom,
  room,
  currentRoom,
}: {
  onChangeRoom: any;
  room: Room;
  currentRoom: Room;
}) {
  const { name } = room;

  return (
    <StyledItem
      selected={room?.id === currentRoom?.id}
      onClick={() => onChangeRoom(room)}
    >
      {name}
    </StyledItem>
  );
}

export default function ChatList({
  onChangeRoom,
  userId,
  users,
  currentRoom,
}: {
  onChangeRoom: any;
  userId: string;
  users: User[];
  currentRoom: Room;
}) {
  const [rooms, setRooms] = useState<Room[]>([]);
  const [isLoading, setLoading] = useState(false);

  useEffect(() => {
    setLoading(true);
    getRooms().then((data) => {
      setRooms(data);
      setLoading(false);
    });
  }, []);

  return (
    <aside>
      {isLoading && <p>Loading chat lists.</p>}
      {rooms?.map((room) => {
        return (
          <ChatListItem
            onChangeRoom={onChangeRoom}
            room={room}
            key={room.id}
            currentRoom={currentRoom}
          />
        );
      })}

      <UserContainer>
        <h4>Users</h4>
        {users.map((user: any) => (
          <p key={user.username}>{user.username}</p>
        ))}
      </UserContainer>
    </aside>
  );
}

export const Games = ({
  games,
  auth,
  sendMessage,
  setGameId,
  setAuthUser,
}: {
  games: DisplayGame[];
  auth: any;
  sendMessage: (msg: any) => void;
  setGameId: Dispatch<SetStateAction<string>>;
  setAuthUser: any;
}) => {
  const joinGame = (gameId: string) => {
    if (gameId == null) {
      return;
    }

    const data: ChatMessage = {
      chat_type: "JOINGAME",
      value: gameId,
      user_id: auth.id,
    };
    sendMessage(JSON.stringify(data));
  };

  const signOut = () => {
    window.localStorage.removeItem("user");
    setAuthUser(false);
  };

  return (
    <aside>
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
          {games.map((game) => {
            //Games needs to be updatd when someone joins or leave
            if (!game.player_1_name) {
              return null;
            }
            return (
              <li key={game.id}>
                <button
                  onClick={() => {
                    joinGame(game.id);
                    setGameId(game.id);
                  }}
                >
                  Play against: {game.player_1_name}
                </button>
              </li>
            );
          })}
        </ul>
      </div>
      <button onClick={signOut}>LOG OUT</button>
    </aside>
  );
};
