import { useState } from "react";
import { TicTacToeMove } from "../../types";

const Square = ({ marker }: { marker: string }) => {
  return (
    <div
      style={{
        display: "flex",
        height: 50,
        width: 50,
        textAlign: "center",
        justifyContent: "center",
        flexDirection: "column",
        border: "1px solid black",
      }}
    >
      {marker}
    </div>
  );
};

export const Board = ({
  onClose,
  gameId,
  moves,
  playerId,
}: {
  onClose: () => void;
  gameId?: string;
  moves: TicTacToeMove[];
  playerId: string;
}) => {
  const [currentGame, setCurrentGame] = useState(gameId);

  // I think I need redux or contexts to handle this, since it needs to get it from the webSockets

  // I want to be able to create a new game
  // load a game (if I have the id)
  // and play moves
  // others should be able to join
  //maybe a crete new game button?
  const createNewGame = () => {
    //fetch new game
  };

  const getMarker = (row: number, col: number) => {
    const move = moves.find((move) => move.row === row && move.column === col);
    if (!move) {
      return "";
    }
    return move.player === playerId ? "x" : "o";
  };

  return (
    <div style={{ backgroundColor: "white" }}>
      <button onClick={onClose} style={{ color: "black" }}>
        Close
      </button>
      {currentGame ? (
        <div
          style={{
            width: 150,
            height: 150,
            display: "grid",
            gap: "1px",
            gridTemplateColumns: "repeat(3, 1fr)",
            color: "red",
          }}
        >
          <Square marker={getMarker(0, 0)} />
          <Square marker={getMarker(0, 1)} />
          <Square marker={getMarker(0, 2)} />
          <Square marker={getMarker(1, 0)} />
          <Square marker={getMarker(1, 1)} />
          <Square marker={getMarker(1, 2)} />
          <Square marker={getMarker(2, 0)} />
          <Square marker={getMarker(2, 1)} />
          <Square marker={getMarker(2, 2)} />
        </div>
      ) : (
        <button style={{ color: "black" }}>Create new game</button>
      )}
    </div>
  );
};
