import { useState } from "react";
import {
  DisplayGame,
  DisplayMessage,
  Message,
  TicTacToeGame,
  User,
} from "../components/types";
import { getRoom } from "api";

//maybe should be a context, since the websocket will update it
//then let current game be its own context?
export default function useRooms() {
  const [isLoading, setIsLoading] = useState(true);
  const [messages, setMessages] = useState<DisplayMessage[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [games, setGames] = useState<DisplayGame[]>([]);

  const fetchRoomData = async (room_id: string, user: User) => {
    setIsLoading(true);
    const data = await getRoom(room_id);
    if (!data) {
      //should have some toaster system
      return;
    }
    setUsers([...data.users, user]);
    setMessages(data.messages);
    setIsLoading(false);

    //this should be done on the backend.
    const games = data.games
      .map((game: TicTacToeGame) => ({
        ...game,
        player_1_name:
          data.users.find((user: User) => user.id === game.player_1)
            ?.username ?? null,
        player_2_name:
          data.users.find((user: User) => user.id === game.player_2)
            ?.username ?? null,
      }))
      .filter(
        (game: TicTacToeGame) =>
          game.game_status === "Active" && game.player_2 == null
      );
    setGames(games);
  };

  const uniqueUsers = users.filter(
    (user, index) =>
      index === users.findIndex((user2) => user.username === user2.username)
  );
  return {
    isLoading,
    users: uniqueUsers,
    setUsers,
    messages,
    setMessages,
    games,
    setGames,
    fetchRoomData,
  };
}
