import React, { useState, useEffect, SetStateAction, Dispatch } from "react";
import styled from "styled-components";
import { ChatMessage, DisplayGame, Room, User } from "./types";
import { getRooms } from "api";

const StyledItem = styled.h3<{ selected: boolean }>`
  padding: 0.2rem 0;
  text-decoration: ${(props) => (props.selected ? "underline" : "none")};
  color: white;
  cursor: pointer;
`;

const UserContainer = styled.div`
  border-top: 1px solid #4c4c4c;
  margin-top: 0.5rem;
  color: white;
  li {
    margin: 5px 0;
  }
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
    if (rooms.length) {
      return;
    }
    setLoading(true);
    getRooms().then((data) => {
      setRooms(data);

      const generalRoom = data.find((room) => room.name === "General Chat");
      if (!currentRoom && currentRoom?.id !== generalRoom?.id) {
        onChangeRoom(generalRoom);
      }
      setLoading(false);
    });
  }, [currentRoom, onChangeRoom, rooms.length]);

  return (
    <aside>
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
        <ul>
          {users.map((user: any) => (
            <li key={user.username}>{user.username}</li>
          ))}
        </ul>
      </UserContainer>
    </aside>
  );
}

const StyledAside = styled.aside`
  button {
    color: white;
    background-color: black;
    padding: 5px;
    margin: 5px;
    border-radius: 5px;
    cursor: pointer;
    width: 70%;
    div {
      display: flex;
      flex-direction: column;
    }
  }

  ul {
    list-style-type: none;
  }
`;

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

  return (
    <StyledAside>
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
        <h3>Current games</h3>
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
                  <div>
                    <span>Play against:</span>
                    <span>{game.player_1_name}</span>
                  </div>
                </button>
              </li>
            );
          })}
        </ul>
      </div>
    </StyledAside>
  );
};
