import React, { useState, useEffect } from "react";
import Avatar from "./avatar";
import { Room, User } from "./types";
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
// onChangeRoom={onChangeRoom}
//             room={room}
//             key={room.id}
//             userId={userId}

function ChatListItem({
  onChangeRoom,
  room,
  userId,
}: {
  onChangeRoom: any;
  room: Room;
  userId: string;
}) {
  const { name, created_at } = room;

  const date = new Date(created_at);
  const ampm = date.getHours() >= 12 ? "PM" : "AM";
  const time = `${date.getHours()}:${date.getMinutes()} ${ampm}`;

  return (
    <div
      onClick={() => onChangeRoom(room)}
      className={`p-2 rounded-[10px] shadow-sm cursor-pointer`}
    >
      <div className="flex justify-between items-center gap-3">
        <div className="flex gap-3 items-center w-full">
          <div className="w-full max-w-[150px]">
            <h3 className="font-semibold text-sm text-gray-700">{name}</h3>
            {/* <p className="font-light text-xs text-gray-600 truncate">
              {last_message}
            </p> */}
          </div>
        </div>
        <div className="text-gray-400 min-w-[55px]">
          <span className="text-xs">{time}</span>
        </div>
      </div>
    </div>
  );
}

export default function ChatList({
  onChangeRoom,
  userId,
  users,
}: {
  onChangeRoom: any;
  userId: string;
  users: User[];
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
    <div className="overflow-hidden space-y-3">
      {isLoading && <p>Loading chat lists.</p>}
      {rooms?.map((room: Room, index: number) => {
        return (
          <ChatListItem
            onChangeRoom={onChangeRoom}
            room={room}
            key={room.id}
            userId={userId}
          />
        );
      })}

      <div>
        <p>Users</p>
        {users.map((user: any) => (
          <p key={user.username}>{user.username}</p>
        ))}
      </div>
    </div>
  );
}
